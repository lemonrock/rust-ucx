// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Memory domain.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub enum MemoryDomain
{
	/// Asterisk; any.
	Wildcard,
	
	/// Similar to, but not the same as, TransportLayerAliases for shared memory
	sysv,
	
	/// Similar to, but not the same as, TransportLayerAliases for shared memory
	posix,
	
	/// Similar to, but not the same as, TransportLayerAliases for shared memory
	xpmem,
}

impl MemoryDomain
{
	#[inline(always)]
	fn to_memory_allocator_priority_str(&self) -> &'static str
	{
		use self::MemoryDomain::*;
		
		match *self
		{
			Wildcard => "md:*",
			sysv => "md:sysv",
			posix => "md:posix",
			xpmem => "md:xpmem",
		}
	}
}
