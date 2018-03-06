// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ucp_feature(u32);

impl BitOr<ucp_feature> for ucp_feature
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		ucp_feature(self.0 | other.0)
	}
}

impl BitOrAssign for ucp_feature
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: ucp_feature)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<ucp_feature> for ucp_feature
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		ucp_feature(self.0 & other.0)
	}
}

impl BitAndAssign for ucp_feature
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: ucp_feature)
	{
		self.0 &= rhs.0;
	}
}

impl ucp_feature
{
	pub const AMO32: Self = ucp_feature(4);
	pub const AMO64: Self = ucp_feature(8);
	pub const RMA: Self = ucp_feature(2);
	pub const STREAM: Self = ucp_feature(32);
	pub const TAG: Self = ucp_feature(1);
	pub const WAKEUP: Self = ucp_feature(16);
}
