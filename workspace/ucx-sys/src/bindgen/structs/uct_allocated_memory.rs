// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
pub struct uct_allocated_memory
{
	pub address: *mut c_void,
	pub length: usize,
	pub method: uct_alloc_method_t,
	pub mem_type: uct_memory_type_t,
	pub md: uct_md_h,
	pub memh: uct_mem_h,
}

impl Default for uct_allocated_memory
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for uct_allocated_memory
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "uct_allocated_memory {{ address: {:?}, method: {:?}, mem_type: {:?} }}", self.address, self.method, self.mem_type)
	}
}
