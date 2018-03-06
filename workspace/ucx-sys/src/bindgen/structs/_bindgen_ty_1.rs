// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct _bindgen_ty_1(u32);

impl BitOr<_bindgen_ty_1> for _bindgen_ty_1
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		_bindgen_ty_1(self.0 | other.0)
	}
}

impl BitOrAssign for _bindgen_ty_1
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: _bindgen_ty_1)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<_bindgen_ty_1> for _bindgen_ty_1
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		_bindgen_ty_1(self.0 & other.0)
	}
}

impl BitAndAssign for _bindgen_ty_1
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: _bindgen_ty_1)
	{
		self.0 &= rhs.0;
	}
}

impl _bindgen_ty_1
{
	pub const ADVISE: Self = _bindgen_ty_1(16);
	pub const ALLOC: Self = _bindgen_ty_1(1);
	pub const FIXED: Self = _bindgen_ty_1(32);
	pub const NEED_MEMH: Self = _bindgen_ty_1(4);
	pub const NEED_RKEY: Self = _bindgen_ty_1(8);
	pub const REG: Self = _bindgen_ty_1(2);
	pub const RKEY_PTR: Self = _bindgen_ty_1(64);
	pub const SOCKADDR: Self = _bindgen_ty_1(128);
}
