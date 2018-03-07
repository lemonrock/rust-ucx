// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Simple type to encapsulate a CPU index.
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct ZeroBasedHyperThreadIndex(u16);

impl ZeroBasedHyperThreadIndex
{
	/// Efficient if VDSO is enabled (which is should be on all modern 4.x series Linux kernels), but, still, should be cached (or put in a thread local) if used repeatedly in tight loops.
	#[inline(always)]
	pub fn for_current_hyper_thread(self) -> Self
	{
		let result = unsafe { sched_getcpu() };
		debug_assert!(result >= 0, "sched_getcpu was negative");
		ZeroBasedHyperThreadIndex(result as u16)
	}
	
	/// As an u16.
	#[inline(always)]
	pub fn as_u16(self) -> u16
	{
		self.0
	}
	
	/// As an u32.
	#[inline(always)]
	pub fn as_u32(self) -> u32
	{
		self.0 as u32
	}
	
	/// As an u64.
	#[inline(always)]
	pub fn as_u64(self) -> u64
	{
		self.0 as u64
	}
	
	/// As an usize.
	#[inline(always)]
	pub fn as_usize(self) -> usize
	{
		self.0 as usize
	}
}
