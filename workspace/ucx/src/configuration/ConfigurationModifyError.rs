// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


quick_error!
{
	/// Error when modifying UCX configuration.
	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub enum ConfigurationModifyError
	{
		/// Invalid status.
		InvalidStatus(cause: ::errors::InvalidStatusError)
		{
			cause(cause)
			description(cause.description())
			display("Status was invalid: {}", cause)
			from()
		}
		
		/// No such configuration key exists.
		NoSuchNamedKey(key_name: String, configuration_value: CString)
		{
			display("Configuration key named '{}' does not exist (and so we can't set value '{:?}'", key_name, configuration_value)
		}
		
		/// A parameter was invalid.
		InvalidParameterNameOrValue(key_name: String, configuration_value: CString)
		{
			display("Configuration value '{:?}' for key named '{}' is invalid", configuration_value, key_name)
		}
	}
}
