// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Exists to ensure that the message_buffer used for the tagged message is not dropped, re-used or written to until this request completes.
///
/// If a `SendingTaggedMessageNonBlockingRequest` is neither cancelled or completed (ie it falls out of scope) then the request will be cancelled and the `message_buffer` dropped.
#[derive(Debug)]
pub struct SendingTaggedMessageNonBlockingRequest<'worker, MessageBuffer: ByteBuffer>
{
	drop_limitation_on_moving_out_work_around: Option<(WorkerWithNonBlockingRequest<'worker>, MessageBuffer)>,
}

impl<'worker, MessageBuffer: ByteBuffer> Drop for SendingTaggedMessageNonBlockingRequest<'worker, MessageBuffer>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if let Some((worker_with_non_blocking_request, message_buffer)) = self.drop_limitation_on_moving_out_work_around.take()
		{
			drop(message_buffer);
			worker_with_non_blocking_request.cancel()
		}
	}
}

impl<'worker, MessageBuffer: ByteBuffer> Deref for SendingTaggedMessageNonBlockingRequest<'worker, MessageBuffer>
{
	type Target = Worker;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.drop_limitation_on_moving_out_work_around.as_ref().map(|&(ref worker_with_non_blocking_request, ref _message_buffer)| worker_with_non_blocking_request).unwrap()
	}
}

impl<'worker, MessageBuffer: ByteBuffer> SendingTaggedMessageNonBlockingRequest<'worker, MessageBuffer>
{
	#[inline(always)]
	pub(crate) fn new(worker_with_non_blocking_request: WorkerWithNonBlockingRequest<'worker>, message_buffer: MessageBuffer) -> Self
	{
		Self
		{
			drop_limitation_on_moving_out_work_around: Some((worker_with_non_blocking_request, message_buffer)),
		}
	}
	
	/// Blocks until a non-blocking request is complete.
	#[inline(always)]
	pub fn block_until_non_blocking_request_is_complete(mut self) -> Result<MessageBuffer, ErrorCodeWithMessageBuffer<MessageBuffer>>
	{
		let (worker_with_non_blocking_request, message_buffer) = self.drop_limitation_on_moving_out_work_around.take().unwrap();
		
		match worker_with_non_blocking_request.block_until_non_blocking_request_is_complete()
		{
			Ok(()) => Ok(message_buffer),
			
			Err(error_code) => Err(ErrorCodeWithMessageBuffer::new(error_code, message_buffer))
		}
	}
	
	/// Cancels a non-blocking request.
	///
	/// Returns the message buffer for re-use.
	#[inline(always)]
	pub fn cancel(mut self) -> MessageBuffer
	{
		let (worker_with_non_blocking_request, message_buffer) = self.drop_limitation_on_moving_out_work_around.take().unwrap();
		
		worker_with_non_blocking_request.cancel();
		message_buffer
	}
	
	/// Check if the request is still in progress.
	///
	/// An Ok(MessageBuffer) means is completed successfully; returns the message buffer for re-use.
	///
	/// An Ok(Self) means it is still in progress.
	///
	/// An Err() means it completed with an error.
	#[inline(always)]
	pub fn is_still_in_progress(mut self) -> Result<NonBlockingRequestCompletedOrInProgress<MessageBuffer, Self>, ErrorCodeWithMessageBuffer<MessageBuffer>>
	{
		let (worker_with_non_blocking_request, message_buffer) = self.drop_limitation_on_moving_out_work_around.take().unwrap();
		
		match worker_with_non_blocking_request.is_still_in_progress()
		{
			Ok(true) => Ok(Completed(message_buffer)),
			
			Ok(false) => Ok(InProgress(Self::new(worker_with_non_blocking_request, message_buffer))),
			
			Err(error_code) => Err(ErrorCodeWithMessageBuffer::new(error_code, message_buffer)),
		}
	}
}
