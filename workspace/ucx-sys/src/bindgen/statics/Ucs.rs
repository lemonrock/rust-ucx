// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}ucs_async_mode_names"] pub static mut ucs_async_mode_names: [*const c_char; 0usize];
	#[link_name = "\u{1}ucs_global_opts"] pub static mut ucs_global_opts: ucs_global_opts_t;
	#[link_name = "\u{1}ucs_stats_formats_names"] pub static mut ucs_stats_formats_names: [*const c_char; 0usize];
}
