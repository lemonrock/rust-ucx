// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn null_or_empty_c_string(raw: *mut c_char) -> Option<CString>
{
	if raw.is_null()
	{
		return None;
	}
	let c_str = unsafe { CStr::from_ptr(raw) };
	if c_str.to_bytes().is_empty()
	{
		None
	}
	else
	{
		Some(c_str.to_owned())
	}
}
