// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A wrapper around requests to make them easier to work with.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UcpAllocatedNonBlockingRequest(NonNull<u8>);

impl Drop for UcpAllocatedNonBlockingRequest
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_request_free(self.non_null_pointer().as_ptr() as *mut c_void) };
	}
}

impl NonBlockingRequest for UcpAllocatedNonBlockingRequest
{
	#[inline(always)]
	fn non_null_pointer(&self) -> NonNull<u8>
	{
		self.0
	}
}

impl UcpAllocatedNonBlockingRequest
{
	#[inline(always)]
	pub(crate) fn new(pointer: NonNull<u8>) -> Self
	{
		UcpAllocatedNonBlockingRequest(pointer)
	}
}
