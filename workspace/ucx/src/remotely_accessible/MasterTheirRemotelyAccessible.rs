// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


pub(crate) struct MasterTheirRemotelyAccessible
{
	master: TheirRemotelyAccessibleThreadLocalEntry,
	per_thread_entries: [TheirRemotelyAccessibleThreadLocalEntry; ZeroBasedHyperThreadIndex::MaximumNumberOfHyperThreads],
}

impl Default for MasterTheirRemotelyAccessible
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			master: TheirRemotelyAccessibleThreadLocalEntry::default(),
			per_thread_entries:
			{
				let mut array: [TheirRemotelyAccessibleThreadLocalEntry; ZeroBasedHyperThreadIndex::MaximumNumberOfHyperThreads] = unsafe { uninitialized() };
				for entry in array.iter_mut()
				{
					*entry = TheirRemotelyAccessibleThreadLocalEntry::default();
				}
				array
			},
		}
	}
}

impl MasterTheirRemotelyAccessible
{
	#[inline(always)]
	pub(crate) fn read(&self) -> TheirRemotelyAccessibleGuard
	{
		let our_hyper_thread_index = ZeroBasedHyperThreadIndex::for_current_hyper_thread();
		
		let reference = our_hyper_thread_index.per_hyper_thread_value(&self.per_thread_entries);
		
		reference.acquire_spin_lock();
		TheirRemotelyAccessibleGuard(reference)
	}
	
	#[inline(always)]
	pub(crate) fn update<R, F: FnOnce(&mut TheirRemotelyAccessible) -> Result<(), R>>(&self, update_operation: F) -> Result<(), R>
	{
		self.lock_update();
		
		{
			let result = self.update_master(update_operation);
			if let Err(error) = result
			{
				return self.unlock_update(Err(error));
			}
			
			self.loop_over_per_thread_copies_of_master_and_update_only_taking_spin_lock_if_it_is_free();
		}
		
		self.unlock_update(Ok(()))
	}
	
	#[inline(always)]
	fn lock_update(&self)
	{
		self.master.acquire_spin_lock()
	}
	
	#[inline(always)]
	fn update_master<R, F: FnOnce(&mut TheirRemotelyAccessible) -> Result<(), R>>(&self, update_operation: F) -> Result<(), R>
	{
		self.master.update(update_operation)
	}
	
	#[inline(always)]
	fn unlock_update<R>(&self, result: Result<(), R>) -> Result<(), R>
	{
		self.master.unlock_spin_lock();
		result
	}
	
	#[inline(always)]
	fn loop_over_per_thread_copies_of_master_and_update_only_taking_spin_lock_if_it_is_free(&self)
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
						let per_thread_entry = unsafe { &* $per_thread_entry(index) };
						
						if per_thread_entry.try_to_acquire_spin_lock()
						{
							per_thread_entry.replace_hyper_thread_local_copy($self.master_their_remotely_accessible_clone());
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
		
		let mut were_locked_retry: [*const TheirRemotelyAccessibleThreadLocalEntry; ZeroBasedHyperThreadIndex::MaximumNumberOfHyperThreads] = unsafe { uninitialized() };
		
		// Loop over all per-thread copies and try to update them.
		// If the spin lock is not available (ie they are locked by a reading thread), add them to a list (`were_locked_retry`) to retry.
		// `were_locked_retry_index` is, after `try_to_update`, a length (number_of_items).
		let mut were_locked_retry_index = try_to_update!(self, were_locked_retry, ZeroBasedHyperThreadIndex::MaximumNumberOfHyperThreads, |index| self.thread_entry(index));
		
		// Retry any per-thread copies that were locked.
		// Loops repeatedly until there are no more per-thread copies that are locked.
		while were_locked_retry_index != 0
		{
			// `were_locked_retry_index` is, after `try_to_update`, a length (number_of_items).
			were_locked_retry_index = try_to_update!(self, were_locked_retry, were_locked_retry_index, |index| * were_locked_retry.get_unchecked(index));
		}
	}
	
	#[inline(always)]
	fn master_their_remotely_accessible_clone(&self) -> TheirRemotelyAccessible
	{
		self.master.their_remotely_accessible_clone()
	}
	
	#[inline(always)]
	fn thread_entry(&self, index: usize) -> *const TheirRemotelyAccessibleThreadLocalEntry
	{
		debug_assert!(index < ZeroBasedHyperThreadIndex::MaximumNumberOfHyperThreads, "index '{}' equals or exceeds MaximumNumberOfHyperThreads '{}'", index, ZeroBasedHyperThreadIndex::MaximumNumberOfHyperThreads);
		
		unsafe { self.per_thread_entries.get_unchecked(index) }
	}
}
