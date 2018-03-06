// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ucs_callbackq_flags(u32);

impl BitOr<ucs_callbackq_flags> for ucs_callbackq_flags
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		ucs_callbackq_flags(self.0 | other.0)
	}
}

impl BitOrAssign for ucs_callbackq_flags
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: ucs_callbackq_flags)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<ucs_callbackq_flags> for ucs_callbackq_flags
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		ucs_callbackq_flags(self.0 & other.0)
	}
}

impl BitAndAssign for ucs_callbackq_flags
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: ucs_callbackq_flags)
	{
		self.0 &= rhs.0;
	}
}

impl ucs_callbackq_flags
{
	pub const FAST: Self = ucs_callbackq_flags(1);
	pub const ONESHOT: Self = ucs_callbackq_flags(2);
}
