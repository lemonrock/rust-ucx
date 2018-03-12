// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Simple type to encapsulate a logical CPU index.
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct ZeroBasedHyperThreadIndex(u16);

impl ZeroBasedHyperThreadIndex
{
	/// The maximum number of hyper threads.
	pub const MaximumNumberOfHyperThreads: usize = 256;
	
	/// Efficient, thread-local cached, current hyper thread; only works when threads are bound to CPUs.
	#[inline(always)]
	pub fn for_current_hyper_thread() -> Self
	{
		const UnsetSentinel: ZeroBasedHyperThreadIndex = ZeroBasedHyperThreadIndex(::std::u16::MAX);
		
		#[thread_local] static mut CachedCurrentHyperThreadIndex: ZeroBasedHyperThreadIndex = UnsetSentinel;
		
		let current = unsafe { CachedCurrentHyperThreadIndex };
		if current == UnsetSentinel
		{
			// Efficient if VDSO is enabled (which is should be on all modern 4.x series Linux kernels), but, still, should be cached (or put in a thread local) if used repeatedly in tight loops.
			let result = unsafe { sched_getcpu() };
			debug_assert!(result >= 0, "sched_getcpu was negative");
			let current = ZeroBasedHyperThreadIndex(result as u16);
			unsafe { CachedCurrentHyperThreadIndex = current };
			current
		}
		else
		{
			current
		}
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
	
	/// Find a per-hyper thread value reference from an array for all hyper threads.
	#[inline(always)]
	pub fn per_hyper_thread_value<T>(self, per_hyper_thread_values: &[T; Self::MaximumNumberOfHyperThreads]) -> &T
	{
		let index = self.as_usize();
		
		debug_assert!(index < Self::MaximumNumberOfHyperThreads, "self '{:?}' equals or exceeds MaximumNumberOfHyperThreads '{}'", index, Self::MaximumNumberOfHyperThreads);
		
		unsafe { per_hyper_thread_values.get_unchecked(index) }
	}
	
	/// Find a per-hyper thread value mutable reference from an array for all hyper threads.
	#[inline(always)]
	pub fn per_hyper_thread_value_mut<T>(self, per_hyper_thread_values: &mut [T; Self::MaximumNumberOfHyperThreads]) -> &T
	{
		let index = self.as_usize();
		
		debug_assert!(index < Self::MaximumNumberOfHyperThreads, "self '{:?}' equals or exceeds MaximumNumberOfHyperThreads '{}'", self, Self::MaximumNumberOfHyperThreads);
		
		unsafe { per_hyper_thread_values.get_unchecked_mut(index) }
	}
}
