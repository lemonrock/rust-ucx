// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A more useful representation of `ucs_status_t`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Status
{
	/// Status is OK.
	Ok,
	
	/// Status is 'in progress'.
	InProgress,
	
	/// Status is an error.
	Error(ErrorCode),
	
	/// Status is an unknown error code.
	UnknownErrorCode(i8),
}

impl Default for Status
{
	#[inline(always)]
	fn default() -> Self
	{
		Status::Ok
	}
}

impl Status
{
	/// Parses a status into something useful.
	/// Returns an error if the status is invalid in some way.
	#[inline(always)]
	pub fn parse_ucs_status_t(status: ucs_status_t) -> Result<Self, i8>
	{
		let status_code = status as i8;
		match status_code
		{
			1 => Ok(Status::InProgress),
			0 => Ok(Status::Ok),
			-1 => Ok(Status::Error(ErrorCode::NoMessage)),
			-2 => Ok(Status::Error(ErrorCode::NoResource)),
			-3 => Ok(Status::Error(ErrorCode::IoError)),
			-4 => Ok(Status::Error(ErrorCode::NoMemory)),
			-5 => Ok(Status::Error(ErrorCode::InvalidParameter)),
			-6 => Ok(Status::Error(ErrorCode::Unreachable)),
			-7 => Ok(Status::Error(ErrorCode::InvalidAddress)),
			-8 => Ok(Status::Error(ErrorCode::NotImplemented)),
			-9 => Ok(Status::Error(ErrorCode::MessageTruncated)),
			-10 => Ok(Status::Error(ErrorCode::NoProgress)),
			-11 => Ok(Status::Error(ErrorCode::BufferTooSmall)),
			-12 => Ok(Status::Error(ErrorCode::NoElement)),
			-13 => Ok(Status::Error(ErrorCode::SomeConnectsFailed)),
			-14 => Ok(Status::Error(ErrorCode::NoDevice)),
			-15 => Ok(Status::Error(ErrorCode::Busy)),
			-16 => Ok(Status::Error(ErrorCode::Cancelled)),
			-17 => Ok(Status::Error(ErrorCode::ShmemSegment)),
			-18 => Ok(Status::Error(ErrorCode::AlreadyExists)),
			-19 => Ok(Status::Error(ErrorCode::OutOfRange)),
			-20 => Ok(Status::Error(ErrorCode::TimedOut)),
			-21 => Ok(Status::Error(ErrorCode::ExceedsLimit)),
			-22 => Ok(Status::Error(ErrorCode::Unsupported)),
			-39 ... -23 => Ok(Status::UnknownErrorCode(status_code)),
			-59 ... -40 => Ok(Status::Error(ErrorCode::LinkFailure((-status_code) as u8 - 40))),
			-79 ... -60 => Ok(Status::Error(ErrorCode::EndPointFailure((-status_code) as u8 - 60))),
			-80 => Ok(Status::Error(ErrorCode::EndPointTimeOut)),
			-100 ... -81 => Ok(Status::UnknownErrorCode(status_code)),
			_ => Err(status_code),
		}
	}
}
