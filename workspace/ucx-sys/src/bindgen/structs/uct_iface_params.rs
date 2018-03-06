// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
pub struct uct_iface_params
{
	pub cpu_mask: ucs_cpu_set_t,
	pub open_mode: u64,
	pub mode: uct_iface_params__bindgen_ty_1,
	pub stats_root: *mut ucs_stats_node_t,
	pub rx_headroom: usize,
	pub err_handler_arg: *mut c_void,
	pub err_handler: uct_error_handler_t,
	pub eager_arg: *mut c_void,
	pub eager_cb: uct_tag_unexp_eager_cb_t,
	pub rndv_arg: *mut c_void,
	pub rndv_cb: uct_tag_unexp_rndv_cb_t,
}

impl Default for uct_iface_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for uct_iface_params
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "uct_iface_params {{ cpu_mask: {:?}, mode: {:?}, stats_root: {:?}, err_handler_arg: {:?}, eager_arg: {:?}, rndv_arg: {:?} }}", self.cpu_mask, self.mode, self.stats_root, self.err_handler_arg, self.eager_arg, self.rndv_arg)
	}
}
