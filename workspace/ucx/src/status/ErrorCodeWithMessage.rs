// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Error codes with a Message; far more useful than `ucs_status_t`.
#[derive(Debug)]
pub struct ErrorCodeWithMessage<M: Message>
{
	/// Error code
	pub error_code: ErrorCode,
	
	/// Message buffer; can be re-used at this point.
	pub message_buffer: M,
}

impl<M: Message> Display for ErrorCodeWithMessage<M>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "Error code '{}' with message buffer", self.error_code)
	}
}

impl<M: Message> Error for ErrorCodeWithMessage<M>
{
	#[inline(always)]
	fn description(&self) -> &str
	{
		"Error code with message buffer"
	}
	
	#[inline(always)]
	fn cause(&self) -> Option<&Error>
	{
		Some(&self.error_code)
	}
}

impl<M: Message> ErrorCodeWithMessage<M>
{
	#[inline(always)]
	pub(crate) fn new(error_code: ErrorCode, message_buffer: M) -> Self
	{
		Self
		{
			error_code,
			message_buffer,
		}
	}
}
