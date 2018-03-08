// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A more sensible type than `ucs_status_ptr_t`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StatusOrNonBlockingRequest
{
	/// Status.
	Status(Status),
	
	/// Pointer.
	NonBlockingRequest(NonBlockingRequest),
}

impl Default for StatusOrNonBlockingRequest
{
	#[inline(always)]
	fn default() -> Self
	{
		StatusOrNonBlockingRequest::Status(Status::IsOk)
	}
}

impl StatusOrNonBlockingRequest
{
	/// Parses a status into something useful.
	/// Returns an error if the status is invalid in some way.
	#[inline(always)]
	pub fn parse_ucs_status_ptr_t(status_or_status_pointer: ucs_status_ptr_t) -> Result<Self, InvalidStatusError>
	{
		let as_isize = status_or_status_pointer as isize;
		match as_isize
		{
			-100 ... 1 => Ok(StatusOrNonBlockingRequest::Status(Status::parse_ucs_status_t(unsafe { transmute(as_isize as i8) })?)),
			_ => Ok(StatusOrNonBlockingRequest::NonBlockingRequest(NonBlockingRequest(unsafe { NonNull::new_unchecked(as_isize as *mut u8) })))
		}
	}
}
