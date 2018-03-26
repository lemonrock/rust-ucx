// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	pub fn ucs_global_opts_clone(dst: *mut c_void) -> ucs_status_t;
	pub fn ucs_global_opts_get_value(name: *const c_char, value: *mut c_char, max: usize) -> ucs_status_t;
	pub fn ucs_global_opts_init();
	pub fn ucs_global_opts_print(stream: *mut FILE, print_flags: ucs_config_print_flags_t);
	pub fn ucs_global_opts_release();
	pub fn ucs_global_opts_set_value(name: *const c_char, value: *const c_char) -> ucs_status_t;
}
