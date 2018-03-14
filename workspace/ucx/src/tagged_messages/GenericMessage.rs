// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A generic message.
#[derive(Debug)]
pub struct GenericMessage<SerializedAndDeserialized: Debug, Operations: GenericDataTypeDescriptorOperations<SerializedAndDeserialized=SerializedAndDeserialized>>
{
	message: SerializedAndDeserialized,
	generic_data_type_descriptor: Arc<GenericDataTypeDescriptor<Operations>>
}

impl<SerializedAndDeserialized: Debug, Operations: GenericDataTypeDescriptorOperations<SerializedAndDeserialized=SerializedAndDeserialized>> Message for GenericMessage<SerializedAndDeserialized, Operations>
{
	#[inline(always)]
	fn address(&self) -> NonNull<u8>
	{
		unsafe { NonNull::new_unchecked(&self.message as *const SerializedAndDeserialized as *mut SerializedAndDeserialized as *mut u8) }
	}
	
	#[inline(always)]
	fn count(&self) -> usize
	{
		Self::element_size()
	}
	
	#[inline(always)]
	fn data_type_descriptor(&self) -> ucp_datatype_t
	{
		self.generic_data_type_descriptor.to_ucp_datatype_t()
	}
	
	#[inline(always)]
	fn compute_count_from_length_in_bytes(_length_in_bytes: usize) -> usize
	{
		Self::element_size()
	}
}

impl<SerializedAndDeserialized: Debug, Operations: GenericDataTypeDescriptorOperations<SerializedAndDeserialized=SerializedAndDeserialized>> GenericMessage<SerializedAndDeserialized, Operations>
{
	/// Creates new instance.
	pub fn new(message: SerializedAndDeserialized, generic_data_type_descriptor: Arc<GenericDataTypeDescriptor<Operations>>) -> Self
	{
		Self
		{
			message,
			generic_data_type_descriptor
		}
	}
	
	#[inline(always)]
	fn element_size() -> usize
	{
		size_of::<SerializedAndDeserialized>()
	}
}
