// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ucs_config_print_flags_t(pub u32);

impl BitOr<ucs_config_print_flags_t> for ucs_config_print_flags_t
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		ucs_config_print_flags_t(self.0 | other.0)
	}
}

impl BitOrAssign for ucs_config_print_flags_t
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: ucs_config_print_flags_t)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<ucs_config_print_flags_t> for ucs_config_print_flags_t
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		ucs_config_print_flags_t(self.0 & other.0)
	}
}

impl BitAndAssign for ucs_config_print_flags_t
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: ucs_config_print_flags_t)
	{
		self.0 &= rhs.0;
	}
}

impl ucs_config_print_flags_t
{
	pub const CONFIG: Self = ucs_config_print_flags_t(1);
	pub const DOC: Self = ucs_config_print_flags_t(4);
	pub const HEADER: Self = ucs_config_print_flags_t(2);
	pub const HIDDEN: Self = ucs_config_print_flags_t(8);
}
