// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.



use super::*;
use ::spin_locks::*;
use ::std::collections::HashMap;
use ::std::mem::replace;
use ::std::ptr::NonNull;
use ::std::sync::atomic::fence;
use ::std::sync::atomic::Ordering::SeqCst;


pub const MaximumNumberOfHyperThreads: usize = 256;

struct All
{
	writer_hyper_thread: ZeroBasedHyperThreadIndex,
	master: TheirRemotelyAccessibleThreadLocalEntry,
	per_thread_entries: [TheirRemotelyAccessibleThreadLocalEntry; MaximumNumberOfHyperThreads],
}

impl All
{
	#[inline(always)]
	pub fn update<F: FnOnce(&mut TheirRemotelyAccessible) -> Result<(), R>, R>(&mut self, update_operation: F) -> Result<(), R>
	{
		debug_assert!(ZeroBasedHyperThreadIndex::for_current_hyper_thread() == self.writer_hyper_thread, "Writer is a single producer and must be locked to a thread");
		
		self.master.update(update_operation)?;
		
		self.loop_over_per_thread_copies_of_master_and_update_only_taking_spin_lock_if_it_is_free();
		
		Ok(())
	}
	
	#[inline(always)]
	fn loop_over_per_thread_copies_of_master_and_update_only_taking_spin_lock_if_it_is_free(&mut self)
	{
		macro_rules! try_to_update
		{
			($self: expr, $were_locked_retry: expr, $were_locked_and_remain_to_update: expr, $per_thread_entry: expr) =>
			{
				{
					let were_locked_and_remain_to_update = $were_locked_and_remain_to_update;
					let mut were_locked_retry_index = 0;
					let mut index = 0;
					while index < were_locked_and_remain_to_update
					{
						let mut per_thread_entry = unsafe { NonNull::new_unchecked($per_thread_entry(index)) };
						let per_thread_entry = unsafe { per_thread_entry.as_mut() };
						
						if per_thread_entry.try_to_acquire_spin_lock()
						{
							per_thread_entry.replace($self.master_their_remotely_accessible_clone());
							per_thread_entry.unlock_spin_lock();
						}
						else
						{
							*(unsafe { $were_locked_retry.get_unchecked_mut(were_locked_retry_index) }) = per_thread_entry;
							were_locked_retry_index += 1;
						}
						
						index += 1;
					}
					
					were_locked_retry_index
				}
			}
		}
		
		let mut were_locked_retry: [*mut TheirRemotelyAccessibleThreadLocalEntry; MaximumNumberOfHyperThreads] = unsafe { uninitialized() };
		
		let mut were_locked_retry_index = try_to_update!(self, were_locked_retry, MaximumNumberOfHyperThreads, |index| self.mutable_thread_entry(index));
		
		// Repeatedly loop until none remain to update.
		while were_locked_retry_index != 0
		{
			were_locked_retry_index = try_to_update!(self, were_locked_retry, were_locked_retry_index, |index| * unsafe { were_locked_retry.get_unchecked(index) });
		}
	}
	
	#[inline(always)]
	fn master_their_remotely_accessible_clone(&self) -> TheirRemotelyAccessible
	{
		self.master.their_remotely_accessible_clone()
	}
	
	#[inline(always)]
	fn mutable_thread_entry(&mut self, index: usize) -> *mut TheirRemotelyAccessibleThreadLocalEntry
	{
		debug_assert!(index < MaximumNumberOfHyperThreads, "index '{}' equals or exceeds MaximumNumberOfHyperThreads '{}'", index, MaximumNumberOfHyperThreads);
		
		unsafe { self.per_thread_entries.get_unchecked_mut(index) }
	}
}

/// 128-byte aligned to eliminate false sharing on x86_64.
#[repr(C, align(128))]
#[derive(Debug, Default)]
struct TheirRemotelyAccessibleThreadLocalEntry
{
	// On modern Intel hardware, this is a HLE SpinLock.
	spin_lock: BestSpinLockForCompilationTarget,
	their_remotely_accessible: TheirRemotelyAccessible
}

impl TheirRemotelyAccessibleThreadLocalEntry
{
	#[inline(always)]
	fn update<F: FnOnce(&mut TheirRemotelyAccessible) -> Result<(), R>, R>(&mut self, update_operation: F) -> Result<(), R>
	{
		update_operation(&mut self.their_remotely_accessible)
	}
	
	#[inline(always)]
	fn replace(&mut self, master: TheirRemotelyAccessible)
	{
		drop(replace(&mut self.their_remotely_accessible, master));
		fence(SeqCst);
	}
	
	#[inline(always)]
	fn their_remotely_accessible_clone(&self) -> TheirRemotelyAccessible
	{
		self.their_remotely_accessible.clone()
	}
	
	#[inline(always)]
	fn try_to_acquire_spin_lock(&self) -> bool
	{
		self.spin_lock.try_to_acquire_spin_lock()
	}
	
	#[inline(always)]
	fn unlock_spin_lock(&self)
	{
		self.spin_lock.unlock_spin_lock()
	}
}


#[derive(Debug, Clone, Default)]
pub struct TheirRemotelyAccessible
{
	devices: HashMap<String, TheirRemotelyAccessibleDevice>,
}

#[derive(Debug, Clone, Default)]
pub struct TheirRemotelyAccessibleDevice
{
	remotely_accessible_workers: HashMap<String, TheirRemotelyAccessibleWorkerAddress>,
	remotely_accessible_memory: HashMap<String, TheirRemotelyAccessibleMemoryAddress>,
	remotely_accessible_servers: HashMap<String, TheirRemotelyAccessibleServerAddress>,
}



