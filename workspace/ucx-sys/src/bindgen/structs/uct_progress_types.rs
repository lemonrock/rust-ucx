// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct uct_progress_types(pub u32);

impl BitOr<uct_progress_types> for uct_progress_types
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		uct_progress_types(self.0 | other.0)
	}
}

impl BitOrAssign for uct_progress_types
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: uct_progress_types)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<uct_progress_types> for uct_progress_types
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		uct_progress_types(self.0 & other.0)
	}
}

impl BitAndAssign for uct_progress_types
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: uct_progress_types)
	{
		self.0 &= rhs.0;
	}
}

impl uct_progress_types
{
	pub const RECV: Self = uct_progress_types(2);
	pub const SEND: Self = uct_progress_types(1);
	pub const THREAD_SAFE: Self = uct_progress_types(128);
}
