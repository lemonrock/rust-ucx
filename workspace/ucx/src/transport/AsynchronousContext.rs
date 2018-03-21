// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An asynchronous context encapsulates how timer and input-output events are managed.
#[derive(Debug)]
pub struct AsynchronousContext
{
	handle: NonNull<ucs_async_context>,
	handle_drop_safety: Arc<AsynchronousContextHandleDropSafety>,
}

impl Drop for AsynchronousContext
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucs_async_context_destroy(self.as_ptr()) }
	}
}

impl AsynchronousContext
{
	/// Creates a new asynchronous context.
	#[inline(always)]
	pub fn new(&self, mode: ucs_async_mode_t) -> Result<Self, ErrorCode>
	{
		let mut handle = unsafe { uninitialized() };
		
		let status = unsafe { ucs_async_context_create(mode, &mut handle) };
		
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
							handle_drop_safety: AsynchronousContextHandleDropSafety::new(handle)
						}
					)
				}
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status)
		}
	}
	
	#[inline(always)]
	pub(crate) fn as_ptr(&self) -> *mut ucs_async_context
	{
		self.handle.as_ptr()
	}
	
	#[inline(always)]
	pub(crate) fn handle_drop_safety(&self) -> Arc<AsynchronousContextHandleDropSafety>
	{
		self.handle_drop_safety.clone()
	}
}
