// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
pub struct uct_iface_attr
{
	pub cap: uct_iface_attr__bindgen_ty_1,
	pub device_addr_len: usize,
	pub iface_addr_len: usize,
	pub ep_addr_len: usize,
	pub max_conn_priv: usize,
	pub overhead: f64,
	pub bandwidth: f64,
	pub latency: uct_linear_growth_t,
	pub priority: u8,
}

impl Default for uct_iface_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for uct_iface_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "uct_iface_attr {{ cap: {:?}, latency: {:?} }}", self.cap, self.latency)
	}
}
