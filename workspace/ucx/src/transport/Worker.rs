// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A progress engine and a domain for allocating communication resources.
///
/// Different workers are progressed independently.
#[derive(Debug)]
pub(crate) struct Worker(uct_worker);

impl Worker
{
	/// Progress queue
	#[inline(always)]
	pub(crate) fn callback_progress_queue(&self) -> &ucs_callbackq
	{
		&self.0.progress_q
	}
	
	/// Progress queue
	#[inline(always)]
	pub(crate) fn callback_progress_queue_mut(&mut self) -> &mut ucs_callbackq
	{
		&mut self.0.progress_q
	}
	
	/// Explicit progress for UCT worker.
	///
	/// Equivalent to `uct_worker_progress`.
	///
	/// Not thread safe.
	///
	/// This routine explicitly progresses any outstanding communication operations and active message requests.
	///
	/// Returns `true` if communication progressed, `false` otherwise.
	#[inline(always)]
	pub(crate) fn progress_thread_unsafe(&self) -> bool
	{
		self.0.progress_q.dispatch_thread_unsafe().from_c_bool()
	}
}
