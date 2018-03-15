// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Priority of this memory event handler
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MemoryEventHandlerPriority
{
	/// Called before original implementation.
	CalledBeforeOriginalImplementation
	{
		/// Larger values make it a lower priority.
		rank: u16,
	},
	
	/// Called after original implementation.
	CalledAfterOriginalImplementation
	{
		/// Larger values make it a lower priority.
		rank: u16,
	},
}

impl Default for MemoryEventHandlerPriority
{
	#[inline(always)]
	fn default() -> Self
	{
		MemoryEventHandlerPriority::CalledAfterOriginalImplementation
		{
			rank: 0
		}
	}
}

impl MemoryEventHandlerPriority
{
	#[inline(always)]
	fn to_priority(self) -> i32
	{
		use self::MemoryEventHandlerPriority::*;
		
		match self
		{
			CalledBeforeOriginalImplementation { rank } => -((rank as i32) + 1),
			
			CalledAfterOriginalImplementation { rank } => rank as i32,
		}
	}
}
