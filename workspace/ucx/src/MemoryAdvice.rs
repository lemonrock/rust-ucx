// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Memory Advice.
///
/// Defaults to `Normal`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MemoryAdvice
{
	/// No special treatment.
	Normal,
	
	/// Can be used on memory originally mapped using `ApplicationContext.register_memory_as_remotely_accessible(non_blocking = true)` to speed up memory mapping and to avoid page faults when the memory is accessed for the first time.
	WillNeed,
}

impl Default for MemoryAdvice
{
	#[inline(always)]
	fn default() -> Self
	{
		MemoryAdvice::Normal
	}
}

impl MemoryAdvice
{
	#[inline(always)]
	fn to_ucp_mem_advice_t(&self) -> ucp_mem_advice_t
	{
		use self::MemoryAdvice::*;
		
		match *self
		{
			Normal => ucp_mem_advice_t::UCP_MADV_NORMAL,
			
			WillNeed => ucp_mem_advice_t::UCP_MADV_WILLNEED,
		}
	}
}
