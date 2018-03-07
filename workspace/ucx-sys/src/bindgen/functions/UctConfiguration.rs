// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_uct_config_get"] pub fn uct_config_get(config: *mut c_void, name: *const c_char, value: *mut c_char, max: usize) -> ucs_status_t;
	#[link_name = "\u{1}_uct_config_modify"] pub fn uct_config_modify(config: *mut c_void, name: *const c_char, value: *const c_char) -> ucs_status_t;
	#[link_name = "\u{1}_uct_config_release"] pub fn uct_config_release(config: *mut c_void);
}
