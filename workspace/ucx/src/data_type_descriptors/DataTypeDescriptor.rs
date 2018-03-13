// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An implementation of a Data Type Descriptor.
///
/// Could have been modelled as an enum, but, for performance, works better as implementations.
pub trait DataTypeDescriptor
{
	// `ucp_dt_type` is a tag which is valid only for bits 0 - 2, ie values 0 to 7 or `u3`.
	#[doc(hidden)]
	#[inline(always)]
	fn to_ucp_dt_type(&self) -> ucp_dt_type;
	
	#[doc(hidden)]
	#[inline(always)]
	fn to_ucp_datatype_t(&self) -> ucp_datatype_t;
}
