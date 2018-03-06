// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ucp_config_modify"] pub fn ucp_config_modify(config: *mut ucp_config_t, name: *const c_char, value: *const c_char) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_config_print"] pub fn ucp_config_print(config: *const ucp_config_t, stream: *mut FILE, title: *const c_char, print_flags: ucs_config_print_flags_t);
	#[link_name = "\u{1}_ucp_config_read"] pub fn ucp_config_read(env_prefix: *const c_char, filename: *const c_char, config_p: *mut *mut ucp_config_t) -> ucs_status_t;
	#[link_name = "\u{1}_ucp_config_release"] pub fn ucp_config_release(config: *mut ucp_config_t);
}
