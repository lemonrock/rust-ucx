// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::buffers::*;
use super::status::*;
use ::libc::c_void;
use ::std::mem::forget;
use ::std::mem::uninitialized;
use ::ucx_sys::*;


include!("GenericDataTypeDescriptor.rs");
include!("GenericDataTypeDescriptorOperations.rs");
include!("GenericDataTypeDescriptorOperationsDeserializer.rs");
include!("GenericDataTypeDescriptorOperationsSerializer.rs");
include!("TagForLowestThreeBits.rs");


/// Default is an (unsigned) byte array descriptor.
pub enum DataTypeDescriptor<Operations: GenericDataTypeDescriptorOperations>
{
	/// An array of elements of `element_size`.
	Contiguous
	{
		/// Maximum element size is 2^61 - 1.
		element_size: u64,
	},
	
	/// ?
	Strided,
	
	/// An array of buffers, like `iovec`.
	///
	/// UCX describes this as "Scatter-gather list with multiple pointers".
	IoVec,
	
	/// Generic
	Generic(GenericDataTypeDescriptor<Operations>)
}

impl<Operations: GenericDataTypeDescriptorOperations> Default for DataTypeDescriptor<Operations>
{
	#[inline(always)]
	fn default() -> Self
	{
		DataTypeDescriptor::ByteArrayDataTypeDescriptor
	}
}

impl<Operations: GenericDataTypeDescriptorOperations> DataTypeDescriptor<Operations>
{
	const UCP_DATATYPE_SHIFT: u64 = 3;
	
	/// Byte array data type descriptor.
	const ByteArrayDataTypeDescriptor: Self = DataTypeDescriptor::Contiguous { element_size: 1 };
	
	/// `ucp_dt_type` is a tag which is valid only for bits 0 - 2, ie values 0 to 7 or `u3`.
	#[inline(always)]
	fn to_ucp_dt_type(&self) -> ucp_dt_type
	{
		use self::DataTypeDescriptor::*;
		
		match *self
		{
			Contiguous { .. } => ucp_dt_type::UCP_DATATYPE_CONTIG,
			
			Strided => ucp_dt_type::UCP_DATATYPE_STRIDED,
			
			IoVec => ucp_dt_type::UCP_DATATYPE_IOV,
			
			Generic(_) => ucp_dt_type::UCP_DATATYPE_GENERIC,
		}
	}
	
	#[inline(always)]
	fn to_ucp_datatype_t(&self) -> ucp_datatype_t
	{
		use self::DataTypeDescriptor::*;
		
		match *self
		{
			Contiguous { element_size } =>
			{
				const TwoToTheSixtyOne: u64 = 0x2000000000000030;
				
				debug_assert_ne!(element_size, 0, "element_size must not be zero");
				debug_assert!(element_size < TwoToTheSixtyOne, "element_size must be less than 2^61");
				
				element_size << Self::UCP_DATATYPE_SHIFT | (ucp_dt_type::UCP_DATATYPE_CONTIG as u64)
			}
			
			Strided => ucp_dt_type::UCP_DATATYPE_STRIDED as u64,
			
			IoVec => ucp_dt_type::UCP_DATATYPE_IOV as u64,
			
			Generic(ref generic_data_type_descriptor) => generic_data_type_descriptor.handle.unwrap(),
		}
	}
}
