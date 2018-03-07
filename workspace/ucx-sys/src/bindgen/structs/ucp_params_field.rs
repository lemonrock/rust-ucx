// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ucp_params_field(pub u32);

impl BitOr<ucp_params_field> for ucp_params_field
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		ucp_params_field(self.0 | other.0)
	}
}

impl BitOrAssign for ucp_params_field
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: ucp_params_field)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<ucp_params_field> for ucp_params_field
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		ucp_params_field(self.0 & other.0)
	}
}

impl BitAndAssign for ucp_params_field
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: ucp_params_field)
	{
		self.0 &= rhs.0;
	}
}

impl ucp_params_field
{
	pub const ESTIMATED_NUM_EPS: Self = ucp_params_field(64);
	pub const FEATURES: Self = ucp_params_field(1);
	pub const MT_WORKERS_SHARED: Self = ucp_params_field(32);
	pub const REQUEST_CLEANUP: Self = ucp_params_field(8);
	pub const REQUEST_INIT: Self = ucp_params_field(4);
	pub const REQUEST_SIZE: Self = ucp_params_field(2);
	pub const TAG_SENDER_MASK: Self = ucp_params_field(16);
}
