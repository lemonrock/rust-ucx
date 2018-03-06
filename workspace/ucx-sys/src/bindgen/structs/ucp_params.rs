// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
pub struct ucp_params
{
	pub field_mask: u64,
	pub features: u64,
	pub request_size: usize,
	pub request_init: ucp_request_init_callback_t,
	pub request_cleanup: ucp_request_cleanup_callback_t,
	pub tag_sender_mask: u64,
	pub mt_workers_shared: c_int,
	pub estimated_num_eps: usize,
}

impl Default for ucp_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ucp_params
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ucp_params {{  }}")
	}
}
