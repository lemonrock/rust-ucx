// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// ?
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct StridedDataTypeDescriptor;

impl DataTypeDescriptor for StridedDataTypeDescriptor
{
	#[doc(hidden)]
	#[inline(always)]
	fn to_ucp_dt_type(&self) -> ucp_dt_type
	{
		ucp_dt_type::UCP_DATATYPE_STRIDED
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn to_ucp_datatype_t(&self) -> ucp_datatype_t
	{
		ucp_dt_type::UCP_DATATYPE_STRIDED as u64
	}
}
