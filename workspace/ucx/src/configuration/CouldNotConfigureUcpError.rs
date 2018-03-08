// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


quick_error!
{
	/// Errors when creating UCP configuration wrapper.
	#[derive(Debug, Clone, PartialEq, Eq)]
	pub enum CouldNotConfigureUcpError
	{
		/// Parse error.
		Parse(cause: ConfigurationParseError)
		{
			cause(cause)
			description(cause.description())
			display("Parse failed: {}", cause)
			from()
		}
		
		/// Modify error.
		Modify(cause: ConfigurationModifyError)
		{
			cause(cause)
			description(cause.description())
			display("Modify failed: {}", cause)
			from()
		}
	}
}
