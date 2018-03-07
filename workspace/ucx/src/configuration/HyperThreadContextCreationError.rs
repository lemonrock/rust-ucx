// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


quick_error!
{
	/// Errors when creating application context for a hyper thread.
	#[derive(Debug, Clone, PartialEq, Eq)]
	pub enum HyperThreadContextCreationError
	{
		/// Invalid status.
		InvalidStatus(cause: ::errors::InvalidStatusError)
		{
			cause(cause)
			description(cause.description())
			display("Status was invalid: {}", cause)
			from()
		}
		
		/// Functionality is not implemented or supported.
		FunctionalityNotImplementedOrSupported
		{
			display("Functionality not implemented or supported")
		}
	}
}