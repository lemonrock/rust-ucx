// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
pub struct ucp_ep_params
{
	pub field_mask: u64,
	pub address: *const ucp_address_t,
	pub err_mode: ucp_err_handling_mode_t,
	pub err_handler: ucp_err_handler_t,
	pub user_data: *mut c_void,
	pub flags: c_uint,
	pub sockaddr: ucs_sock_addr_t,
}

impl Default for ucp_ep_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ucp_ep_params
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ucp_ep_params {{ address: {:?}, err_mode: {:?}, err_handler: {:?}, user_data: {:?}, sockaddr: {:?} }}", self.address, self.err_mode, self.err_handler, self.user_data, self.sockaddr)
	}
}
