// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct uct_msg_flags(pub u32);

impl BitOr<uct_msg_flags> for uct_msg_flags
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		uct_msg_flags(self.0 | other.0)
	}
}

impl BitOrAssign for uct_msg_flags
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: uct_msg_flags)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<uct_msg_flags> for uct_msg_flags
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		uct_msg_flags(self.0 & other.0)
	}
}

impl BitAndAssign for uct_msg_flags
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: uct_msg_flags)
	{
		self.0 &= rhs.0;
	}
}

impl uct_msg_flags
{
	pub const SIGNALED: Self = uct_msg_flags(1);
}
