// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ucm_event_type(pub u32);

impl BitOr<ucm_event_type> for ucm_event_type
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		ucm_event_type(self.0 | other.0)
	}
}

impl BitOrAssign for ucm_event_type
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: ucm_event_type)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<ucm_event_type> for ucm_event_type
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		ucm_event_type(self.0 & other.0)
	}
}

impl BitAndAssign for ucm_event_type
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: ucm_event_type)
	{
		self.0 &= rhs.0;
	}
}

impl ucm_event_type
{
	pub const FLAG_NO_INSTALL: Self = ucm_event_type(16777216);
	pub const MEM_TYPE_ALLOC: Self = ucm_event_type(1048576);
	pub const MEM_TYPE_FREE: Self = ucm_event_type(2097152);
	pub const MMAP: Self = ucm_event_type(1);
	pub const MREMAP: Self = ucm_event_type(4);
	pub const MUNMAP: Self = ucm_event_type(2);
	pub const SBRK: Self = ucm_event_type(32);
	pub const SHMAT: Self = ucm_event_type(8);
	pub const SHMDT: Self = ucm_event_type(16);
	pub const VM_MAPPED: Self = ucm_event_type(65536);
	pub const VM_UNMAPPED: Self = ucm_event_type(131072);
}
