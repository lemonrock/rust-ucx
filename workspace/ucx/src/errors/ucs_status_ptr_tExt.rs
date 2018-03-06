// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An extension trait for `ucs_status_ptr_t`, which can represent OK (`NULL`), a valid pointer, or an error code `ERR_*`.
#[allow(non_camel_case_types)]
pub trait ucs_status_ptr_tExt
{
	/// A function safer than the ucs macros `UCS_PTR_STATUS`, `UCS_PTR_IS_ERR` and `UCS_PTR_IS_PTR`.
	#[allow(non_snake_case)]
	#[inline(always)]
	fn parse(self) -> Result<StatusOrPointer, InvalidStatusError>;
	
	/// Is this pointer actually an OK non-error?
	#[inline(always)]
	fn ucs_is_ok(self) -> bool;
}

impl ucs_status_ptr_tExt for ucs_status_ptr_t
{
	#[inline(always)]
	fn parse(self) -> Result<StatusOrPointer, InvalidStatusError>
	{
		StatusOrPointer::parse_ucs_status_ptr_t(self)
	}
	
	#[inline(always)]
	fn ucs_is_ok(self) -> bool
	{
		self.is_null()
	}
}
