// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


quick_error!
{
	/// Errors when reading UCX configuration.
	#[derive(Debug, Clone, PartialEq, Eq)]
	pub enum ConfigurationParseError
	{
		/// Environment variable prefix not valid (fatal).
		EnvironmentVariablePrefixWasNotValidAsACString(cause: ::std::ffi::NulError)
		{
			cause(cause)
			description(cause.description())
			display("Environment variable prefix was not valid as a CString")
			from()
		}
		
		/// Invalid status (fatal).
		InvalidStatus(cause: ::errors::InvalidStatusError)
		{
			cause(cause)
			description(cause.description())
			display("Status was invalid: {}", cause)
			from()
		}
	
		/// No transport devices exist that are suitable.
		NoTransportDevicesExistThatAreSuitable
		{
			display("No transport devices exist that are suitable")
		}
		
		/// Functionality is not implemented or supported.
		FunctionalityNotImplementedOrSupported
		{
			display("Functionality not implemented or supported")
		}
	}
}
