// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_uct_mem_alloc"] pub fn uct_mem_alloc(addr: *mut c_void, min_length: usize, flags: c_uint, methods: *mut uct_alloc_method_t, num_methods: c_uint, mds: *mut uct_md_h, num_mds: c_uint, name: *const c_char, mem: *mut uct_allocated_memory_t) -> ucs_status_t;
	#[link_name = "\u{1}_uct_mem_free"] pub fn uct_mem_free(mem: *const uct_allocated_memory_t) -> ucs_status_t;
}
