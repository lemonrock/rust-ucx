// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
pub struct uct_iface_attr__bindgen_ty_1__bindgen_ty_4__bindgen_ty_2
{
	pub max_short: usize,
	pub max_bcopy: usize,
	pub max_zcopy: usize,
	pub max_iov: usize,
}

impl Default for uct_iface_attr__bindgen_ty_1__bindgen_ty_4__bindgen_ty_2
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for uct_iface_attr__bindgen_ty_1__bindgen_ty_4__bindgen_ty_2
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "uct_iface_attr__bindgen_ty_1__bindgen_ty_4__bindgen_ty_2 {{  }}")
	}
}
