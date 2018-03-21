// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A progress engine and a domain for allocating communication resources.
///
/// Also known as a worker.
///
/// Different workers are progressed independently.
#[derive(Debug, Clone)]
pub struct ProgressEngine
{
	handle: NonNull<uct_worker>,
	handle_drop_safety: Arc<ProgressEngineHandleDropSafety>,
}

impl Drop for ProgressEngine
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { uct_worker_destroy(self.as_ptr()) }
	}
}

impl ProgressEngine
{
	/// Create a new progress engine.
	///
	/// Equivalent to `uct_worker_create`.
	///
	/// Specify `no_thread_synchronization` if using the created `ProgressEngine` from just the thread that creates it.
	///
	/// This will perform faster and allow for external synchronization as necessary.
	#[inline(always)]
	pub fn create(asynchronous_context: Option<&AsynchronousContext>, no_thread_synchronization: bool) -> Result<Self, ErrorCode>
	{
		let (asynchronous_context_pointer, asynchronous_context_handle_drop_safety) = match asynchronous_context
		{
			None => (null_mut(), None),
			Some(asynchronous_context) => (asynchronous_context.as_ptr(), Some(asynchronous_context.handle_drop_safety())),
		};
		
		let mut handle = unsafe { uninitialized() };
		
		let status = unsafe { uct_worker_create(asynchronous_context_pointer, WorkerThreadMode::thread_sharing_mode(no_thread_synchronization), &mut handle) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk =>
			{
				debug_assert!(!handle.is_null(), "handle is null");
				let handle = unsafe { NonNull::new_unchecked(handle) };
				Ok
				(
					Self
					{
						handle,
						handle_drop_safety: ProgressEngineHandleDropSafety::new(handle, asynchronous_context_handle_drop_safety),
					}
				)
			}
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status)
		}
	}
	
	/// Explicit progress.
	///
	/// Equivalent to `uct_worker_progress`.
	///
	/// Not thread safe.
	///
	/// This routine explicitly progresses any outstanding communication operations and active message requests.
	///
	/// Returns `true` if communication progressed, `false` otherwise.
	#[inline(always)]
	pub fn progress_thread_unsafe(&self) -> bool
	{
		self.reference().progress_q.dispatch_thread_unsafe().from_c_bool()
		// ucp_worker then does ucs_async_check_miss(&worker->async);
	}
	
	/// Add a progress callback function.
	///
	/// Equivalent to `uct_worker_progress_register_safe`.
	///
	/// The `CallbackQueueIdentifier` can be used to remove this callback if necessary.
	#[inline(always)]
	pub fn progress_register_thread_safe<'progress_engine, PC: ProgressCallback>(&'progress_engine self, progress_callback: Box<PC>) -> ProgressCallbackCancel<'progress_engine, PC>
	{
		let (ffi_progress_callback_data, progress_callback) =
		{
			let raw_pointer = Box::into_raw(progress_callback);
			(raw_pointer as *mut c_void, unsafe { Box::from_raw(raw_pointer) })
		};
		
		let flags = progress_callback.kind() as u32;
		
		let mut progress_callback_cancel = ProgressCallbackCancel
		{
			callback_queue_identifier: CallbackQueueIdentifier(UCS_CALLBACKQ_ID_NULL),
			progress_callback,
			progress_engine: self,
		};
		
		unsafe { uct_worker_progress_register_safe(self.as_ptr(), ProgressCallbackCancel::<PC>::ffi_progress_callback, ffi_progress_callback_data, flags, &mut progress_callback_cancel.callback_queue_identifier.0) };
		
		progress_callback_cancel
	}
	
	#[inline(always)]
	pub(crate) fn as_ptr(&self) -> *mut uct_worker
	{
		self.handle.as_ptr()
	}
	
	#[inline(always)]
	fn reference(&self) -> &uct_worker
	{
		unsafe { self.handle.as_ref() }
	}
}
