// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// May be used to cancel a callback.
///
/// Note that dropping this struct *does not* cancel the progress callback.
pub struct ProgressCallbackCancel<'progress_engine, PC: ProgressCallback>
{
	callback_queue_identifier: CallbackQueueIdentifier,
	progress_callback: Box<PC>, // Also passed to UCX via FFI asa raw pointer; dropped by `ffi_progress_callback()`.
	progress_engine: &'progress_engine ProgressEngine,
}

impl<'progress_engine, PC: ProgressCallback> ProgressCallbackCancel<'progress_engine, PC>
{
	/// Removes a callback function that has yet to be called ('cancels').
	///
	/// Equivalent to `uct_worker_progress_unregister_safe`.
	#[inline(always)]
	pub fn cancel(self)
	{
		let mut callback_queue_identifier = self.callback_queue_identifier.0;
		
		unsafe { uct_worker_progress_unregister_safe(self.progress_engine.as_ptr(), &mut callback_queue_identifier) };
		
		// This avoids a double-drop (double-free) ***ONLY*** if UCX callback queues ensure that an unregistered callback is not still called (see `ffi_progress_callback()`).
		drop(self.progress_callback)
	}
	
	unsafe extern "C" fn ffi_progress_callback(arg: *mut c_void) -> u32
	{
		debug_assert!(!arg.is_null(), "arg is null");
		let mut boxed_progress_callback = Box::from_raw(arg as *mut PC);
		boxed_progress_callback.invoke().to_c_bool()
	}
}
