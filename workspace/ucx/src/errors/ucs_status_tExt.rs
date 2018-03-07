// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An extension trait for `ucs_status_t`.
#[allow(non_camel_case_types)]
pub trait ucs_status_tExt
{
	/// A function to convert this enum into something more useful.
	#[inline(always)]
	fn parse(self) -> Result<Status, InvalidStatusError>;
	
	/// A function equivalent to the ucs macro `UCS_IS_LINK_ERROR`.
	#[allow(non_snake_case)]
	#[inline(always)]
	fn UCS_IS_LINK_ERROR(self) -> bool;
	
	/// A function equivalent to the ucs macro `UCS_IS_ENDPOINT_ERROR`.
	#[allow(non_snake_case)]
	#[inline(always)]
	fn UCS_IS_ENDPOINT_ERROR(self) -> bool;
	
	/// String message for this error.
	#[inline(always)]
	fn string(self) -> &'static CStr;
	
	/// Is this error actually `OK`?
	#[inline(always)]
	fn is_ok(self) -> bool;
	
	/// Is this error actually just in progress?
	#[inline(always)]
	fn is_in_progress(self) -> bool;
	
	/// Is this just busy?
	#[inline(always)]
	fn is_busy(self) -> bool;
	
	/// To a status pointer
	#[inline(always)]
	fn to_pointer(self) -> ucs_status_ptr_t;
}

impl ucs_status_tExt for ucs_status_t
{
	#[inline(always)]
	fn parse(self) -> Result<Status, InvalidStatusError>
	{
		Status::parse_ucs_status_t(self)
	}
	
	#[inline(always)]
	fn UCS_IS_LINK_ERROR(self) -> bool
	{
		let code = self as i8;
		code <= UCS_ERR_FIRST_LINK_FAILURE as i8 && code >= UCS_ERR_LAST_LINK_FAILURE as i8
	}
	
	#[inline(always)]
	fn UCS_IS_ENDPOINT_ERROR(self) -> bool
	{
		let code = self as i8;
		code <= UCS_ERR_FIRST_ENDPOINT_FAILURE as i8 && code >= UCS_ERR_LAST_ENDPOINT_FAILURE as i8
	}
	
	#[inline(always)]
	fn string(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(ucs_status_string(self)) }
	}
	
	#[inline(always)]
	fn is_ok(self) -> bool
	{
		self == UCS_OK
	}
	
	#[inline(always)]
	fn is_in_progress(self) -> bool
	{
		self == UCS_INPROGRESS
	}
	
	#[inline(always)]
	fn is_busy(self) -> bool
	{
		self == UCS_ERR_BUSY
	}
	
	#[inline(always)]
	fn to_pointer(self) -> ucs_status_ptr_t
	{
		unsafe { transmute(self as isize as *mut u8) }
	}
}
