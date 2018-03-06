// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct uct_rkey_bundle
{
	pub rkey: uct_rkey_t,
	pub handle: *mut c_void,
	pub type_: *mut c_void,
}

impl Default for uct_rkey_bundle
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for uct_rkey_bundle
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "uct_rkey_bundle {{ handle: {:?}, type: {:?} }}", self.handle, self.type_)
	}
}
