// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
pub struct ucp_worker_params
{
	pub field_mask: u64,
	pub thread_mode: ucs_thread_mode_t,
	pub cpu_mask: ucs_cpu_set_t,
	pub events: c_uint,
	pub user_data: *mut c_void,
	pub event_fd: c_int,
}

impl Default for ucp_worker_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ucp_worker_params
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ucp_worker_params {{ thread_mode: {:?}, cpu_mask: {:?}, user_data: {:?} }}", self.thread_mode, self.cpu_mask, self.user_data)
	}
}
