// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ucm_config_modify"] pub fn ucm_config_modify(name: *const c_char, value: *const c_char) -> ucs_status_t;
	#[link_name = "\u{1}_ucm_config_print"] pub fn ucm_config_print(stream: *mut FILE, print_flags: ucs_config_print_flags_t);
}
