// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ucp_get_version"] pub fn ucp_get_version(major_version: *mut c_uint, minor_version: *mut c_uint, release_number: *mut c_uint);
	#[link_name = "\u{1}_ucp_get_version_string"] pub fn ucp_get_version_string() -> *const c_char;
}
