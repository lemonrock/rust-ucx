// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ucs_handle_error_t(pub u32);

impl BitOr<ucs_handle_error_t> for ucs_handle_error_t
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		ucs_handle_error_t(self.0 | other.0)
	}
}

impl BitOrAssign for ucs_handle_error_t
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: ucs_handle_error_t)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<ucs_handle_error_t> for ucs_handle_error_t
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		ucs_handle_error_t(self.0 & other.0)
	}
}

impl BitAndAssign for ucs_handle_error_t
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: ucs_handle_error_t)
	{
		self.0 &= rhs.0;
	}
}

impl ucs_handle_error_t
{
	pub const BACKTRACE: Self = ucs_handle_error_t(0);
	pub const DEBUG: Self = ucs_handle_error_t(2);
	pub const FREEZE: Self = ucs_handle_error_t(1);
	pub const LAST: Self = ucs_handle_error_t(3);
}
