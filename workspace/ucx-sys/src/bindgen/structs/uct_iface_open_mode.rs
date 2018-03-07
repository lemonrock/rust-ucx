// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct uct_iface_open_mode(pub u32);

impl BitOr<uct_iface_open_mode> for uct_iface_open_mode
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		uct_iface_open_mode(self.0 | other.0)
	}
}

impl BitOrAssign for uct_iface_open_mode
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: uct_iface_open_mode)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<uct_iface_open_mode> for uct_iface_open_mode
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		uct_iface_open_mode(self.0 & other.0)
	}
}

impl BitAndAssign for uct_iface_open_mode
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: uct_iface_open_mode)
	{
		self.0 &= rhs.0;
	}
}

impl uct_iface_open_mode
{
	pub const DEVICE: Self = uct_iface_open_mode(1);
	pub const SOCKADDR_CLIENT: Self = uct_iface_open_mode(4);
	pub const SOCKADDR_SERVER: Self = uct_iface_open_mode(2);
}
