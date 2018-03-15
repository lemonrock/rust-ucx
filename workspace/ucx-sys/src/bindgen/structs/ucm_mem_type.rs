// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ucm_mem_type(pub u32);

impl BitOr<ucm_mem_type> for ucm_mem_type
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		ucm_mem_type(self.0 | other.0)
	}
}

impl BitOrAssign for ucm_mem_type
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: ucm_mem_type)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<ucm_mem_type> for ucm_mem_type
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		ucm_mem_type(self.0 & other.0)
	}
}

impl BitAndAssign for ucm_mem_type
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: ucm_mem_type)
	{
		self.0 &= rhs.0;
	}
}

impl ucm_mem_type
{
	pub const CUDA: Self = ucm_mem_type(1);
}

impl ucm_mem_type
{
}
