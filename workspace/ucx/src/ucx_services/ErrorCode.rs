// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Error codes; far more useful than `ucs_status_t`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ErrorCode
{
	#[allow(missing_docs)]
	NoMessage,
	
	#[allow(missing_docs)]
	NoResource,
	
	#[allow(missing_docs)]
	IoError,
	
	#[allow(missing_docs)]
	NoMemory,
	
	#[allow(missing_docs)]
	InvalidParameter,
	
	#[allow(missing_docs)]
	Unreachable,
	
	#[allow(missing_docs)]
	InvalidAddress,
	
	#[allow(missing_docs)]
	NotImplemented,
	
	#[allow(missing_docs)]
	MessageTruncated,
	
	#[allow(missing_docs)]
	NoProgress,
	
	#[allow(missing_docs)]
	BufferTooSmall,
	
	#[allow(missing_docs)]
	NoElement,
	
	#[allow(missing_docs)]
	SomeConnectsFailed,
	
	#[allow(missing_docs)]
	NoDevice,
	
	#[allow(missing_docs)]
	Busy,
	
	#[allow(missing_docs)]
	Cancelled,
	
	#[allow(missing_docs)]
	ShmemSegment,
	
	#[allow(missing_docs)]
	AlreadyExists,
	
	#[allow(missing_docs)]
	OutOfRange,
	
	#[allow(missing_docs)]
	TimedOut,
	
	#[allow(missing_docs)]
	ExceedsLimit,
	
	#[allow(missing_docs)]
	Unsupported,
	
	#[allow(missing_docs)]
	LinkFailure(u8),
	
	#[allow(missing_docs)]
	EndPointFailure(u8),
	
	#[allow(missing_docs)]
	EndPointTimeOut,
}
