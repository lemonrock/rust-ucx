// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Error handler.
pub trait ErrorHandler
{
	/// Handle an error.
	#[inline(always)]
	fn handle(&self, end_point: NonNull<uct_ep>, error_code: ErrorCode) -> Result<(), ErrorCode>;
}
