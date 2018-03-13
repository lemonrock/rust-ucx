// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A wrapper to make it possible to work with the API `EndPoint.non_blocking_send_tagged_message_v2()`.
pub struct UserAllocatedNonBlockingRequest([u8; ApplicationContextAttributes::ReservedSpaceInNonBlockingRequestsRoundedUp]);

impl Default for UserAllocatedNonBlockingRequest
{
	#[inline(always)]
	fn default() -> Self
	{
		UserAllocatedNonBlockingRequest(unsafe { uninitialized() })
	}
}

impl NonBlockingRequest for UserAllocatedNonBlockingRequest
{
	// Weirdly, should point just after the last element.
	#[inline(always)]
	fn non_null_pointer(&self) -> NonNull<u8>
	{
		unsafe { NonNull::new_unchecked((self.0.as_ptr() as *mut u8).offset(ApplicationContextAttributes::ReservedSpaceInNonBlockingRequestsRoundedUp as isize)) }
	}
}
