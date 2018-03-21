// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct AsynchronousContextHandleDropSafety(NonNull<ucs_async_context>);

impl Drop for AsynchronousContextHandleDropSafety
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucs_async_context_destroy(self.0.as_ptr()) }
	}
}

impl AsynchronousContextHandleDropSafety
{
	#[inline(always)]
	pub(crate) fn new(value: NonNull<ucs_async_context>) -> Arc<Self>
	{
		Arc::new(AsynchronousContextHandleDropSafety(value))
	}
}
