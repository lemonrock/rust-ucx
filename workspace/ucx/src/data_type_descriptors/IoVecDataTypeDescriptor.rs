// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An array of buffers, like `iovec`.
///
/// UCX describes this as "Scatter-gather list with multiple pointers".
///
/// Associated message buffer must be an array of `ucp_dt_iov_t`.
/// * In this case, `MessageBuffer`'s `length()` is either: the number of entries or the total array size;
///*  In this case, `MessageBuffer`'s `address()` is the pointer to the first item in the array.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct IoVecDataTypeDescriptor;

impl DataTypeDescriptor for IoVecDataTypeDescriptor
{
	#[doc(hidden)]
	#[inline(always)]
	fn to_ucp_dt_type(&self) -> ucp_dt_type
	{
		ucp_dt_type::UCP_DATATYPE_IOV
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn to_ucp_datatype_t(&self) -> ucp_datatype_t
	{
		ucp_dt_type::UCP_DATATYPE_IOV as u64
	}
}
