// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Default NonBlockingRequestMemoryCustomization which does nothing.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct NoNonBlockingRequestMemoryCustomization;

impl NonBlockingRequestMemoryCustomization for NoNonBlockingRequestMemoryCustomization
{
	#[inline(always)]
	fn function_pointers() -> (ucp_request_init_callback_t, ucp_request_cleanup_callback_t)
	{
		(None, None)
	}
}
