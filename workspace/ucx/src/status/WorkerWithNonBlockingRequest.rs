// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A simple struct to make it easier to work with non-blocking requests.
/// Dereferences to the parent worker, so one can call `progress()` on this instance.
#[derive(Debug)]
pub struct WorkerWithNonBlockingRequest<'worker, Request = UcxAllocatedNonBlockingRequest>
{
	pub(crate) parent_worker: &'worker Worker,
	
	pub(crate) non_blocking_request: Request,
}

impl<'worker, Request: NonBlockingRequest> Deref for WorkerWithNonBlockingRequest<'worker, Request>
{
	type Target = Worker;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.parent_worker
	}
}

impl<'worker, Request: NonBlockingRequest> WorkerWithNonBlockingRequest<'worker, Request>
{
	#[inline(always)]
	pub(crate) fn new(parent_worker: &'worker Worker, non_blocking_request: Request) -> Self
	{
		Self
		{
			parent_worker,
			non_blocking_request,
		}
	}
	
	/// Blocks until a non-blocking request is complete.
	#[inline(always)]
	pub fn block_until_non_blocking_request_is_complete(self) -> Result<(), ErrorCode>
	{
		self.non_blocking_request.subsequently_block_until_non_blocking_request_is_complete(self.parent_worker)
	}
	
	/// Cancels a non-blocking request.
	#[inline(always)]
	pub fn cancel(self)
	{
		self.non_blocking_request.cancel(self.parent_worker)
	}
	
	/// Check if the request is still in progress.
	///
	/// An Ok(true) means is completed successfully,
	/// An Ok(false) means it is still in progress.
	/// An Err() means it completed with an error.
	#[inline(always)]
	pub fn is_still_in_progress(&self) -> Result<bool, ErrorCode>
	{
		self.non_blocking_request.is_still_in_progress()
	}
}
