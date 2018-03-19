// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


trait CallbackQueue
{
	//noinspection SpellCheckingInspection
	/// Dispatch callbacks from the callback queue.
	///
	/// Equivalent to `ucs_callbackq_dispatch`.
	///
	/// Must be called from single thread only.
	///
	/// Returns sum of all return values from the dispatched callbacks.
	#[inline(always)]
	unsafe fn dispatch(&self) -> u32;
}

impl CallbackQueue for ucs_callbackq
{
	#[inline(always)]
	unsafe fn dispatch(&self) -> u32
	{
		// This loop works because the final array element's `cb` in `fast_elems` is always a None Sentinel (ie there are UCS_CALLBACKQ_FAST_COUNT - 1 elements).
		let mut sum_of_all_return_values_from_dispatched_callbacks = 0;
		let mut index = 0;
		let mut element = self.fast_elems.get_unchecked(0);
		while let Some(callback) = element.cb
		{
			sum_of_all_return_values_from_dispatched_callbacks += callback(element.arg);
			index += 1;
			element = self.fast_elems.get_unchecked(index)
		}
		sum_of_all_return_values_from_dispatched_callbacks
	}
}
