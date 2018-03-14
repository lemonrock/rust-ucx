// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


quick_error!
{
	/// Error codes; far more useful than `ucs_status_t`.
	#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
	pub enum ErrorCode
	{
		/// Only seems to be relevant to receiving.
		/// Does not seem to ever escape stats internal code.
		///
		/// Seems to be a transient failure (but one we could treat as fatal as we shouldn't be expecting it).
		NoPendingMessage
		{
		}
		
		/// Seems to be caused by flush(); try-again.
		///
		/// Seems to be a transient failure.
		NoResourcesAreAvailableToInitiateTheOperation
		{
		}
		
		/// Input or Output error.
		///
		/// May be a transient failure; used for a lot of scenarios in UCX.
		InputOutputError
		{
		}
		
		/// Could not allocate (usually with `malloc`) memory.
		///
		/// Pretty much fatal.
		OutOfMemory
		{
		}

		///
		/// Pretty much fatal.
		InvalidParameter
		{
		}

		/// Usually called because we can not connect to the remote memory access key (`rkey`).
		/// Tear down `RemoteMemoryAccessKey` and probably `TheirRemotelyAccessibleEndPointEndPoint`.
		///
		/// Seems to be a recoverable failure.
		DestinationIsUnreachable
		{
		}

		/// Failures with this reason occur early on because the address is just plain wrong
		/// *Except* for the UCS stats client, which returns this if `gethostaddr()` fails, ie if DNS fails.
		/// Caused by:-
		/// * invalid remote address.
		/// * TCP address is not IP V6.
		/// * Can not pack remote access key into remote address buffer.
		///
		/// Pretty much fatal.
		InvalidAddress
		{
		}
		
		/// Function not implemented.
		///
		/// Pretty much fatal.
		FunctionNotImplemented
		{
		}
		
		/// Message truncated?
		MessageTruncated
		{
		}

		/// It is believed that this should not leak up to the UCP API; seems to indicate internal ucx programming failure.
		///
		/// Pretty much fatal.
		NoProgress
		{
		}
		
		/// Never occurs in current UCX code.
		ProvidedBufferIsTooSmall
		{
		}
		
		/// Apart from configuration-time discovering that there are no devices (`ucs_error`) this seems to indicate internal ucx programming failure.
		///
		/// Pretty much fatal.
		NoSuchElement
		{
		}

		/// Never occurs in current UCX code.
		FailedToConnectToSomeOfTheRequestedEndPoints
		{
		}

		/// Should occur quite early in program execution; in essence, there are no suitable devices available for a given transport, eg we asked for InfiniBand and there are no InfiniBand cards / ports.
		///
		/// Pretty much fatal.
		NoSuchDevice
		{
		}
		
		/// Device is busy.
		///
		/// Should be possible to try the operation again.
		DeviceIsBusy
		{
		}
		
		/// Passed to a send or receive callback as a result of cancelling a non-blocking request.
		RequestCancelled
		{
		}

		/// ?
		ShmemSegment
		{
		}

		/// Seems to indicate internal ucx programming failure.
		///
		/// Pretty much fatal.
		ElementAlreadyExists
		{
		}

		/// Seems to indicate internal ucx programming failure; only used in stats code.
		/// Also means a (C string) name is too long for an internal buffer.
		///
		/// Pretty much fatal.
		IndexOutOfRange
		{
		}

		/// Never occurs in current UCX code.
		OperationTimedOut
		{
		}
		
		/// ?
		UserDefinedLimitWasExceeded
		{
		}

		/// Differs to `FunctionNotImplemented` in that a particular function exists but a particular path of (reasonable from an user's perspective) logic through it is not supported.
		///
		/// Pretty much fatal.
		UnsupportedOperation
		{
		}
		
		/// ?
		LinkFailure(index: u8)
		{
			display("LinkFailure {}", index)
		}
		
		/// ?
		EndPointFailure(index: u8)
		{
			display("EndPointFailure {}", index)
		}

		/// Tear down RemoteMemoryAccessKey and probably TheirRemotelyAccessibleEndPointEndPoint.
		///
		/// Seems to be a recoverable failure.
		EndPointTimeOut
		{
		}
	}
}

impl ErrorCode
{
	#[inline(always)]
	pub(crate) fn to_ucs_status_t(&self) -> ucs_status_t
	{
		use self::ErrorCode::*;
		
		let value = match *self
		{
			NoPendingMessage => -1,
			NoResourcesAreAvailableToInitiateTheOperation => -2,
			InputOutputError => -3,
			OutOfMemory => -4,
			InvalidParameter => -5,
			DestinationIsUnreachable => -6,
			InvalidAddress => -7,
			FunctionNotImplemented => -8,
			MessageTruncated => -9,
			NoProgress => -10,
			ProvidedBufferIsTooSmall => -11,
			NoSuchElement => -12,
			FailedToConnectToSomeOfTheRequestedEndPoints => -13,
			NoSuchDevice => -14,
			DeviceIsBusy => -15,
			RequestCancelled => -16,
			ShmemSegment => -17,
			ElementAlreadyExists => -18,
			IndexOutOfRange => -19,
			OperationTimedOut => -20,
			UserDefinedLimitWasExceeded => -21,
			UnsupportedOperation => -22,
			LinkFailure(relative_code) => -(relative_code as i8 + 40) as i8,
			EndPointFailure(relative_code) => -(relative_code as i8 + 60) as i8,
			EndPointTimeOut => -80,
		};
		
		unsafe { transmute(value) }
	}
}
