// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


trait CallbackQueue
{
	/// Initialize callback queue memory
	///
	/// Equivalent to `ucs_callbackq_init`.
	#[inline(always)]
	fn initialize(&mut self) -> Result<(), ErrorCode>;
	
	/// De-initialize callback queue memory
	///
	/// Equivalent to `ucs_callbackq_cleanup`.
	#[inline(always)]
	fn clean_up(&mut self);
	
	/// Add a callback to the queue.
	///
	/// Equivalent to `ucs_callbackq_add`.
	///
	/// This is *not* safe to call while another thread might be dispatching callbacks.
	/// However, it can be used from the dispatch context (eg a callback may use this function to add another callback).
	#[inline(always)]
	fn add_thread_unsafe<T>(&mut self, callback: ucs_callback_t, callback_data: *mut T, flags: ucs_callbackq_flags) -> CallbackQueueIdentifier;
	
	/// Add a callback to the queue (thread-safe).
	///
	/// Equivalent to `ucs_callbackq_add_safe`.
	///
	/// This can be used from any context and any thread, including but not limited to:-
	/// * A callback can add another callback.
	/// * A thread can add a callback while another thread is dispatching callbacks (`dispatch_thread_unsafe`).
	#[inline(always)]
	fn add_thread_safe<T>(&mut self, callback: ucs_callback_t, callback_data: *mut T, flags: ucs_callbackq_flags) -> CallbackQueueIdentifier;
	
	/// Remove a callback from the queue immediately.
	///
	/// Equivalent to `ucs_callbackq_remove`.
	///
	/// This is *not* safe to call while another thread might be dispatching callbacks (`dispatch_thread_unsafe`).
	/// However, it can be used from the dispatch context (eg a callback may use this function to remove itself or another callback).
	/// In this case, the callback may still be dispatched once after this function returned.
	#[inline(always)]
	fn remove_thread_unsafe<T>(&mut self, identifier: CallbackQueueIdentifier);
	
	/// Remove a callback from the queue lazily and thread safely.
	///
	/// Equivalent to `ucs_callbackq_remove_safe`.
	///
	/// This can be used from any context and any thread, including but not limited to:-
	/// * A callback can remove another callback or itself.
	/// * A thread can't remove a callback while another thread is dispatching callbacks (`dispatch_thread_unsafe`).
	#[inline(always)]
	fn remove_thread_safe<T>(&mut self, identifier: CallbackQueueIdentifier);
	
	/// Remove all callbacks from the queue for which the given predicate returns a non-zero ('true') value.
	///
	/// Equivalent to `ucs_callbackq_remove_if`.
	///
	/// This is *not* safe to call while another thread might be dispatching callbacks (`dispatch_thread_unsafe`).
	/// However, it can be used from the dispatch context (eg a callback may use this function to remove itself or another callback).
	/// In this case, the callback may still be dispatched once after this function returned.
	#[inline(always)]
	fn remove_if_thread_unsafe<T>(&mut self, predicate_callback: ucs_callbackq_predicate_t, predicate_callback_data: *mut T);
	
	//noinspection SpellCheckingInspection
	/// Dispatch callbacks from the callback queue.
	///
	/// Equivalent to `ucs_callbackq_dispatch`.
	///
	/// Must be called from single thread only.
	///
	/// Returns sum of all return values from the dispatched callbacks.
	#[inline(always)]
	fn dispatch_thread_unsafe(&self) -> u32;
}

impl CallbackQueue for ucs_callbackq
{
	#[inline(always)]
	fn initialize(&mut self) -> Result<(), ErrorCode>
	{
		let status = unsafe { ucs_callbackq_init(self) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(()),
			
			Error(error_code) => Err(error_code),
			
			_ => panic!("Unexpected status '{:?}'", status),
		}
	}
	
	#[inline(always)]
	fn clean_up(&mut self)
	{
		unsafe { ucs_callbackq_cleanup(self) }
	}
	
	#[inline(always)]
	fn add_thread_unsafe<T>(&mut self, callback: ucs_callback_t, callback_data: *mut T, flags: ucs_callbackq_flags) -> CallbackQueueIdentifier
	{
		debug_assert_ne!((flags & ucs_callbackq_flags::ONESHOT == ucs_callbackq_flags::ONESHOT) && (flags & ucs_callbackq_flags::FAST == ucs_callbackq_flags::FAST), true, "ONESHOT and FAST can not both be specified in flags");
		
		CallbackQueueIdentifier::new(unsafe { ucs_callbackq_add(self, callback, callback_data as *mut _, flags.0) })
	}
	
	#[inline(always)]
	fn add_thread_safe<T>(&mut self, callback: ucs_callback_t, callback_data: *mut T, flags: ucs_callbackq_flags) -> CallbackQueueIdentifier
	{
		debug_assert_ne!((flags & ucs_callbackq_flags::ONESHOT == ucs_callbackq_flags::ONESHOT) && (flags & ucs_callbackq_flags::FAST == ucs_callbackq_flags::FAST), true, "ONESHOT and FAST can not both be specified in flags");
		
		CallbackQueueIdentifier::new(unsafe { ucs_callbackq_add_safe(self, callback, callback_data as *mut _, flags.0) })
	}
	
	#[inline(always)]
	fn remove_thread_unsafe<T>(&mut self, identifier: CallbackQueueIdentifier)
	{
		unsafe { ucs_callbackq_remove(self, identifier.0) }
	}
	
	#[inline(always)]
	fn remove_thread_safe<T>(&mut self, identifier: CallbackQueueIdentifier)
	{
		unsafe { ucs_callbackq_remove_safe(self, identifier.0) }
	}
	
	#[inline(always)]
	fn remove_if_thread_unsafe<T>(&mut self, predicate_callback: ucs_callbackq_predicate_t, predicate_callback_data: *mut T)
	{
		unsafe { ucs_callbackq_remove_if(self, predicate_callback, predicate_callback_data as *mut _) }
	}
	
	#[inline(always)]
	fn dispatch_thread_unsafe(&self) -> u32
	{
		// This loop works because the final array element's `cb` in `fast_elems` is always a None Sentinel (ie there are UCS_CALLBACKQ_FAST_COUNT - 1 elements).
		// An alternative is to check that `element.id != UCS_CALLBACKQ_ID_NULL`.
		let mut sum_of_all_return_values_from_dispatched_callbacks = 0;
		let mut index = 0;
		unsafe
		{
			let mut element = self.fast_elems.get_unchecked(0);
			while let Some(callback) = element.cb
			{
				sum_of_all_return_values_from_dispatched_callbacks += callback(element.arg);
				index += 1;
				element = self.fast_elems.get_unchecked(index)
			}
		}
		sum_of_all_return_values_from_dispatched_callbacks
	}
}
