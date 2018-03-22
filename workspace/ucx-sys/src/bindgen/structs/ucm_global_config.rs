// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
pub struct ucm_global_config
{
	pub log_level: ucs_log_level_t,
	pub enable_events: c_int,
	pub enable_mmap_reloc: c_int,
	pub enable_malloc_hooks: c_int,
	pub enable_malloc_reloc: c_int,
	pub enable_cuda_reloc: c_int,
	pub enable_dynamic_mmap_thresh: c_int,
	pub alloc_alignment: usize,
}

impl Default for ucm_global_config
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ucm_global_config
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ucm_global_config {{ log_level: {:?} }}", self.log_level)
	}
}
