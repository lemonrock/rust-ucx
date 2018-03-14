// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Exists to ensure that the message used for the tagged message is not dropped, re-used or written to until this request completes.
///
/// If a `SendingStreamNonBlockingRequest` is neither cancelled or completed (ie it falls out of scope) then the request will be cancelled and the `message` dropped.
#[derive(Debug)]
pub struct SendingTaggedMessageNonBlockingRequest<'worker, M: Message, Request = UcxAllocatedNonBlockingRequest>
where Request: NonBlockingRequest
{
	drop_limitation_on_moving_out_work_around: Option<(WorkerWithNonBlockingRequest<'worker, Request>, M)>,
}

impl<'worker, M: Message, Request: NonBlockingRequest> Drop for SendingTaggedMessageNonBlockingRequest<'worker, M, Request>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if let Some((worker_with_non_blocking_request, message)) = self.drop_limitation_on_moving_out_work_around.take()
		{
			drop(message);
			worker_with_non_blocking_request.cancel()
		}
	}
}

impl<'worker, M: Message, Request: NonBlockingRequest> Deref for SendingTaggedMessageNonBlockingRequest<'worker, M, Request>
{
	type Target = Worker;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.drop_limitation_on_moving_out_work_around.as_ref().map(|&(ref worker_with_non_blocking_request, ref _message_buffer)| worker_with_non_blocking_request).unwrap()
	}
}

impl<'worker, M: Message, Request: NonBlockingRequest> SendingTaggedMessageNonBlockingRequest<'worker, M, Request>
{
	#[inline(always)]
	pub(crate) fn new(worker_with_non_blocking_request: WorkerWithNonBlockingRequest<'worker, Request>, message: M) -> Self
	{
		Self
		{
			drop_limitation_on_moving_out_work_around: Some((worker_with_non_blocking_request, message)),
		}
	}
	
	/// Cancels a non-blocking request.
	///
	/// Returns the message for re-use.
	#[inline(always)]
	pub fn cancel(mut self) -> M
	{
		let (worker_with_non_blocking_request, message) = self.drop_limitation_on_moving_out_work_around.take().unwrap();
		
		worker_with_non_blocking_request.cancel();
		message
	}
	
	/// Blocks until a non-blocking request is complete.
	#[inline(always)]
	pub fn block_until_non_blocking_request_is_complete(mut self) -> Result<M, ErrorCodeWithMessage<M>>
	{
		let (worker_with_non_blocking_request, message) = self.drop_limitation_on_moving_out_work_around.take().unwrap();
		
		match worker_with_non_blocking_request.block_until_non_blocking_request_is_complete()
		{
			Ok(()) => Ok(message),
			
			Err(error_code) => Err(ErrorCodeWithMessage::new(error_code, message))
		}
	}
	
	/// Check if the request is still in progress.
	///
	/// An Ok(M) means is completed successfully; returns the message for re-use.
	///
	/// An Ok(Self) means it is still in progress.
	///
	/// An Err() means it completed with an error.
	#[inline(always)]
	pub fn is_still_in_progress(mut self) -> Result<NonBlockingRequestCompletedOrInProgress<M, Self>, ErrorCodeWithMessage<M>>
	{
		let (worker_with_non_blocking_request, message) = self.drop_limitation_on_moving_out_work_around.take().unwrap();
		
		match worker_with_non_blocking_request.is_still_in_progress()
		{
			Ok(true) => Ok(Completed(message)),
			
			Ok(false) => Ok(InProgress(Self::new(worker_with_non_blocking_request, message))),
			
			Err(error_code) => Err(ErrorCodeWithMessage::new(error_code, message)),
		}
	}
}
