// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Called back when complete.
pub trait CompletionHandler
{
	/// Called back when completed OK, including when completed immediately.
	#[inline(always)]
	fn completed_ok(&self);
	
	/// Called back when completed with an error, including when completed immediately.
	#[inline(always)]
	fn completed_with_error(&self, error_code: ErrorCode);
}
