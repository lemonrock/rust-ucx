// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct uct_tag_context
{
	pub tag_consumed_cb: Option<unsafe extern "C" fn(self_: *mut uct_tag_context_t)>,
	pub completed_cb: Option<unsafe extern "C" fn(self_: *mut uct_tag_context_t, stag: uct_tag_t, imm: u64, length: usize, status: ucs_status_t)>,
	pub rndv_cb: Option<unsafe extern "C" fn(self_: *mut uct_tag_context_t, stag: uct_tag_t, header: *const c_void, header_length: c_uint, status: ucs_status_t)>,
	pub priv_: [c_char; 32usize],
}

impl Default for uct_tag_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for uct_tag_context
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"uct_tag_context {{ tag_consumed_cb: {:?}, completed_cb: {:?}, rndv_cb: {:?}, priv: [{}] }}",
			self.tag_consumed_cb,
			self.completed_cb,
			self.rndv_cb,
			self.priv_
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>()
		)
	}
}
