// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A custom (generic) data type descriptor.
#[derive(Debug)]
pub struct GenericDataTypeDescriptor<Operations: GenericDataTypeDescriptorOperations>
{
	// The `handle` in UCX-land is a actually a tagged pointer (to ucp_dt_generic_t) with the lower three bits set to `ucp_dt_type::UCP_DATATYPE_GENERIC`...
	handle: ucp_datatype_t,
	operations: Operations,
}

impl<Operations: GenericDataTypeDescriptorOperations> Drop for GenericDataTypeDescriptor<Operations>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.handle != Self::NoHandle
		{
			unsafe { ucp_dt_destroy(self.handle) }
		}
	}
}

impl<Operations: GenericDataTypeDescriptorOperations> DataTypeDescriptor for GenericDataTypeDescriptor<Operations>
{
	#[inline(always)]
	fn to_ucp_dt_type(&self) -> ucp_dt_type
	{
		ucp_dt_type::UCP_DATATYPE_GENERIC
	}
	
	#[inline(always)]
	fn to_ucp_datatype_t(&self) -> ucp_datatype_t
	{
		self.handle
	}
}

impl<Operations: GenericDataTypeDescriptorOperations> GenericDataTypeDescriptor<Operations>
{
	const NoHandle: ucp_datatype_t = !0;
	
	/// Creates new (global) instance.
	///
	/// Currently, the only known way this can fail is due to `ErrorCode::OutOfMemory`.
	///
	/// Returns an `Arc` because when this instance goes out of scope, it will drop the underlying generic type used by UCX.
	///
	/// It would probably be possible to use a `Rc` or a reference (stored as a field on a Worker) with some care.
	#[inline(always)]
	pub fn new(operations: Operations) -> Result<Arc<UnsafeCell<Self>>, ErrorCode>
	{
		let this = Arc::new
		(
			UnsafeCell::new
			(
				Self
				{
					handle: Self::NoHandle,
					operations,
				}
			)
		);
		
		let operations = ucp_generic_dt_ops_t
		{
			start_pack: Some(Self::start_pack),
			start_unpack: Some(Self::start_unpack),
			packed_size: Some(Self::packed_size),
			pack: Some(Self::pack),
			unpack: Some(Self::unpack),
			finish: Some(Self::finish),
		};
		
		let raw_this: *mut Self = this.get();
		let handle_pointer = &mut (unsafe { &mut * raw_this }).handle;
		let status = unsafe { ucp_dt_create_generic(&operations, raw_this as *mut c_void, handle_pointer) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(this),
			
			Error(error_code) => Err(error_code),
			
			unexpected @ _ => panic!("Unexpected status '{:?}'", unexpected),
		}
	}
	
	unsafe extern "C" fn start_pack(context: *mut c_void, buffer: *const c_void, count: usize) -> *mut c_void
	{
		debug_assert!(!buffer.is_null(), "buffer is null");
		debug_assert_eq!(count, size_of::<Operations::SerializedAndDeserialized>(), "count '{}' is not size_of::<Operations::SerializedAndDeserialized>() '{}'", count, size_of::<Operations::SerializedAndDeserialized>());
		
		let untagged_pointer = Box::into_raw(Self::context_to_operations(context).start_serialization(& * (buffer as *const Operations::SerializedAndDeserialized))) as *mut c_void;
		TagForLowestThreeBits::SerializerTag.tag(untagged_pointer)
	}
	
	unsafe extern "C" fn start_unpack(context: *mut c_void, buffer: *mut c_void, count: usize) -> *mut c_void
	{
		debug_assert!(!buffer.is_null(), "buffer is null");
		
		let untagged_pointer = Box::into_raw(Self::context_to_operations(context).start_deserialization(UcxAllocatedByteBuffer::new(buffer, count))) as *mut c_void;
		TagForLowestThreeBits::DeserializerTag.tag(untagged_pointer)
	}
	
	unsafe extern "C" fn packed_size(state: *mut c_void) -> usize
	{
		if TagForLowestThreeBits::untag_just_tag(state).is(TagForLowestThreeBits::SerializerTag)
		{
			let serializer = Self::tagged_pointer_to_serializer(state);
			
			let size = serializer.size();
			
			forget(serializer);
			
			size
		}
		else
		{
			let deserializer = Self::tagged_pointer_to_deserializer(state);
			
			let size = deserializer.size();
			
			forget(deserializer);
			
			size
		}
	}
	
	unsafe extern "C" fn pack(state: *mut c_void, offset: usize, dest: *mut c_void, max_length: usize) -> usize
	{
		let serializer = Self::tagged_pointer_to_serializer(state);
		
		let data_written_to_output_buffer = serializer.serialize(offset, UcxAllocatedByteBuffer::new(dest, max_length));
		
		forget(serializer);
		
		debug_assert!(data_written_to_output_buffer <= max_length, "data_written_to_output_buffer '{}' exceeds max_length '{}'", data_written_to_output_buffer, max_length);
		
		data_written_to_output_buffer
	}
	
	unsafe extern "C" fn unpack(state: *mut c_void, offset: usize, src: *const c_void, count: usize) -> ucs_status_t
	{
		let deserializer = Self::tagged_pointer_to_deserializer(state);
		
		let status = match deserializer.deserialize(offset, UcxAllocatedByteBuffer::new(src as *mut _, count))
		{
			Ok(()) => ucs_status_t::UCS_OK,
			Err(error_code) => error_code.to_ucs_status_t(),
		};
		
		forget(deserializer);
		
		status
	}
	
	unsafe extern "C" fn finish(state: *mut c_void)
	{
		if TagForLowestThreeBits::untag_just_tag(state).is(TagForLowestThreeBits::SerializerTag)
		{
			drop(Self::tagged_pointer_to_serializer(state))
		}
		else
		{
			drop(Self::tagged_pointer_to_deserializer(state))
		}
	}
	
	#[inline(always)]
	fn context_to_operations<'fake>(context: *mut c_void) -> &'fake Operations
	{
		debug_assert!(!context.is_null(), "context is null");
		
		let this = unsafe { & * (context as *mut Self) };
		&this.operations
	}
	
	#[inline(always)]
	fn tagged_pointer_to_serializer(state: *mut c_void) -> Box<Operations::Serializer>
	{
		debug_assert!(!state.is_null(), "state is null");
		
		let pointer = TagForLowestThreeBits::untag_just_pointer(state);
		unsafe { Box::from_raw(pointer as *mut Operations::Serializer) }
	}
	
	#[inline(always)]
	fn tagged_pointer_to_deserializer(state: *mut c_void) -> Box<Operations::Deserializer>
	{
		debug_assert!(!state.is_null(), "state is null");
		
		let pointer = TagForLowestThreeBits::untag_just_pointer(state);
		unsafe { Box::from_raw(pointer as *mut Operations::Deserializer) }
	}
}
