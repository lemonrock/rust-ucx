// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
pub struct ucm_event__bindgen_ty_1
{
	pub result: *mut c_void,
	pub address: *mut c_void,
	pub size: usize,
	pub prot: c_int,
	pub flags: c_int,
	pub fd: c_int,
	pub offset: off_t,
}

impl Default for ucm_event__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ucm_event__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ucm_event__bindgen_ty_1 {{ result: {:?}, address: {:?} }}", self.result, self.address)
	}
}
