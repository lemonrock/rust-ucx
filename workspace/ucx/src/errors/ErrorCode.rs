// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Error codes; far more useful than `ucs_status_t`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ErrorCode
{
	#[allow(missing_docs)]
	NoPendingMessage,
	
	#[allow(missing_docs)]
	NoResourcesAreAvailableToInitiateTheOperation,
	
	#[allow(missing_docs)]
	InputOutputError,
	
	#[allow(missing_docs)]
	OutOfMemory,
	
	#[allow(missing_docs)]
	InvalidParameter,
	
	#[allow(missing_docs)]
	DestinationIsUnreachable,
	
	#[allow(missing_docs)]
	InvalidAddress,
	
	#[allow(missing_docs)]
	FunctionNotImplemented,
	
	#[allow(missing_docs)]
	MessageTruncated,
	
	#[allow(missing_docs)]
	NoProgress,
	
	#[allow(missing_docs)]
	ProvidedBufferIsTooSmall,
	
	#[allow(missing_docs)]
	NoSuchElement,
	
	#[allow(missing_docs)]
	FailedToConnectToSomeOfTheRequestedEndPoints,
	
	#[allow(missing_docs)]
	NoSuchDevice,
	
	#[allow(missing_docs)]
	DeviceIsBusy,
	
	#[allow(missing_docs)]
	RequestCancelled,
	
	#[allow(missing_docs)]
	ShmemSegment,
	
	#[allow(missing_docs)]
	ElementAlreadyExists,
	
	#[allow(missing_docs)]
	IndexOutOfRange,
	
	#[allow(missing_docs)]
	OperationTimedOut,
	
	#[allow(missing_docs)]
	UserDefinedLimitWasExceeded,
	
	#[allow(missing_docs)]
	UnsupportedOperation,
	
	#[allow(missing_docs)]
	LinkFailure(u8),
	
	#[allow(missing_docs)]
	EndPointFailure(u8),
	
	#[allow(missing_docs)]
	EndPointTimeOut,
}
