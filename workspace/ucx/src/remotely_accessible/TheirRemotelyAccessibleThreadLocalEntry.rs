// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// 128-byte aligned to eliminate false sharing on x86_64.
/// Uses HLE-aware spin locks which on the fast path have no penalty; on a slower path, have one fetch-and-add-like operation.
#[repr(C, align(128))]
#[derive(Debug, Default)]
struct TheirRemotelyAccessibleThreadLocalEntry
{
	spin_lock: BestSpinLockForCompilationTarget,
	their_remotely_accessible: UnsafeCell<TheirRemotelyAccessible>,
}

impl TheirRemotelyAccessibleThreadLocalEntry
{
	#[inline(always)]
	fn update<F: FnOnce(&mut TheirRemotelyAccessible) -> Result<(), R>, R>(&self, update_operation: F) -> Result<(), R>
	{
		update_operation(self.master_mutable_reference())
	}
	
	#[inline(always)]
	fn replace_hyper_thread_local_copy(&self, clone_of_master: TheirRemotelyAccessible)
	{
		drop(replace(self.master_mutable_reference(), clone_of_master));
		fence(SeqCst);
	}
	
	#[inline(always)]
	fn their_remotely_accessible_clone(&self) -> TheirRemotelyAccessible
	{
		self.master_reference().clone()
	}
	
	#[inline(always)]
	fn acquire_spin_lock(&self)
	{
		self.spin_lock.acquire_spin_lock()
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
	
	#[inline(always)]
	fn master_reference(&self) -> &TheirRemotelyAccessible
	{
		unsafe { & * self.their_remotely_accessible.get() }
	}
	
	#[inline(always)]
	fn master_mutable_reference(&self) -> &mut TheirRemotelyAccessible
	{
		unsafe { &mut * self.their_remotely_accessible.get() }
	}
}
