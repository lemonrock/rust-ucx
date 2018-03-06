// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
pub struct uct_md_attr
{
	pub cap: uct_md_attr__bindgen_ty_1,
	pub reg_cost: uct_linear_growth_t,
	pub component_name: [c_char; 8usize],
	pub rkey_packed_size: usize,
	pub local_cpus: cpu_set_t,
}

impl Default for uct_md_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for uct_md_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "uct_md_attr {{ cap: {:?}, reg_cost: {:?}, component_name: {:?} }}", self.cap, self.reg_cost, self.component_name)
	}
}
