// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Remotely accessible memory is only needed for RMA and AMO32/AMO64 (atomic operations).
///
/// It is *not* needed for sending or receiving tagged tagged_messages.
///
/// The memory could be registered to one or multiple network resources that are supported by UCP, such as InfiniBand, Gemini, and others.
///
/// This is an opaque object representing a memory region allocated through UCP library, which is optimized for remote memory access operations (particularly, zero-copy operations).
/// It abstracts the information required to access the memory region locally, while a `TheirRemotelyAccessibleMemory` is used to access it remotely.
#[derive(Debug)]
pub struct OurRemotelyAccessibleMemory
{
	handle: ucp_mem_h,
	our_remotely_accessible_memory_handle_drop_safety: Rc<OurRemotelyAccessibleMemoryHandleDropSafety>,
	application_context_handle: ucp_context_h,
	attributes: OurRemotelyAccessibleMemoryAttributes,
}

impl HasAttributes for OurRemotelyAccessibleMemory
{
	type Attributes = OurRemotelyAccessibleMemoryAttributes;
	
	#[inline(always)]
	fn attributes(&self) -> &Self::Attributes
	{
		&self.attributes
	}
}

impl OurRemotelyAccessibleMemory
{
	/// Advises UCP about how to handle memory range beginning at `address` and size of `length` bytes.
	/// This call does not influence the semantics of the application, but may influence its performance.
	/// The advice may be ignored.
	///
	/// The `offset` and `length` define a range which lie entirely must be within this memory; the top (high) address of this range is exclusive.
	///
	/// It is not clear if `length` can be zero.
	#[inline(always)]
	pub fn advise(&self, offset: usize, length: usize, memory_advice: MemoryAdvice)
	{
		let attributes = self.attributes();
		let (our_address, our_length) = (attributes.address().as_ptr() as usize, attributes.length());
		
		debug_assert!(offset + length <= our_length, "offset '{}' + length '{}' exceeds that mapped", offset, length);
		
		let mut parameters = ucp_mem_advise_params_t
		{
			field_mask: (ucp_mem_advise_params_field::ADDRESS | ucp_mem_advise_params_field::LENGTH | ucp_mem_advise_params_field::ADVICE).0 as u64,
			address: (our_address + offset) as *mut c_void,
			length,
			advice: memory_advice.to_ucp_mem_advice_t(),
		};
		
		panic_on_error!(ucp_mem_advise, self.application_context_handle, self.handle, &mut parameters);
	}
	
	/// This creates a `OurRemotelyAccessibleMemoryAddress`, which is an opaque piece of data that needs to be sent to other machines so that they can uniquely connect to our remotely accessible memory.
	#[inline(always)]
	pub fn our_remotely_accessible_memory_access_address(&self) -> OurRemotelyAccessibleMemoryAddress
	{
		let mut address = unsafe { uninitialized() };
		let mut length = unsafe { uninitialized() };
		panic_on_error!(ucp_rkey_pack, self.application_context_handle, self.handle, &mut address, &mut length);
		
		debug_assert!(!address.is_null(), "address is null");
		debug_assert_ne!(length, 0, "length is zero");
		
		OurRemotelyAccessibleMemoryAddress
		{
			address: unsafe { NonNull::new_unchecked(address as *mut u8) },
			length,
			our_remotely_accessible_memory_handle_drop_safety: self.our_remotely_accessible_memory_handle_drop_safety.clone(),
		}
	}
}
