// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An array of elements of `element_size`.
///
/// Default is an (unsigned) byte array descriptor.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ContiguousDataTypeDescriptor
{
	element_size: u64,
}

impl Default for ContiguousDataTypeDescriptor
{
	#[inline(always)]
	fn default() -> Self
	{
		ContiguousDataTypeDescriptor::ByteArrayDataTypeDescriptor
	}
}

impl<'message_buffer> DataTypeDescriptor for ContiguousDataTypeDescriptor
{
	#[inline(always)]
	fn to_ucp_dt_type(&self) -> ucp_dt_type
	{
		ucp_dt_type::UCP_DATATYPE_CONTIG
	}
	
	#[inline(always)]
	fn to_ucp_datatype_t(&self) -> ucp_datatype_t
	{
		self.element_size << Self::UCP_DATATYPE_SHIFT | (ucp_dt_type::UCP_DATATYPE_CONTIG as u64)
	}
}

impl ContiguousDataTypeDescriptor
{
	const UCP_DATATYPE_SHIFT: u64 = 3;
	
	/// Byte array data type descriptor.
	const ByteArrayDataTypeDescriptor: Self = Self { element_size: 1 };
	
	/// Creates a new instance.
	///
	/// `element_size` can not be zero.
	/// `element_size` must be less than 2^61.
	pub fn new(element_size: u64) -> Self
	{
		const TwoToTheSixtyOne: u64 = 0x2000000000000030;
		
		debug_assert_ne!(element_size, 0, "element_size must not be zero");
		debug_assert!(element_size < TwoToTheSixtyOne, "element_size must be less than 2^61");
		
		Self
		{
			element_size
		}
	}
}
