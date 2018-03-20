// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Does nothing.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DoNothingErrorHandler;

impl ErrorHandler for DoNothingErrorHandler
{
	#[inline(always)]
	fn handle(&self, _end_point: NonNull<uct_ep>, error_code: ErrorCode) -> Result<(), ErrorCode>
	{
		Err(error_code)
	}
}
