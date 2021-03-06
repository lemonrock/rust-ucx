// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A wrapper around UCP configuration for an `ApplicationContext`.
/// The configuration is initially populated from environment variables.
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UcpConfigurationWrapper
{
	pub(crate) handle: *mut ucp_config,
}

impl Drop for UcpConfigurationWrapper
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_config_release(self.handle) };
	}
}

impl Debug for UcpConfigurationWrapper
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		self.debug_fmt(f)
	}
}

impl PrintInformation for UcpConfigurationWrapper
{
	const DebugName: &'static str = "UcpConfigurationWrapper";
	
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		let print_flags = ucs_config_print_flags_t::CONFIG | ucs_config_print_flags_t::DOC | ucs_config_print_flags_t::HEADER | ucs_config_print_flags_t::HIDDEN;
		
		let title = c_str!("UCP Configuration");
		unsafe { ucp_config_print(self.handle, stream, title.as_ptr(), print_flags) };
	}
}

impl ConfigurationWrapper for UcpConfigurationWrapper
{
	#[inline(always)]
	unsafe fn ucx_config_modify(&self, name: *const c_char, value: *const c_char) -> ucs_status_t
	{
		ucp_config_modify(self.handle, name, value)
	}
}

impl UcpConfigurationWrapper
{
	/// Parses a new configuration from environment variables.
	/// The prefix defaults to `UCX_` if not specified.
	#[inline(always)]
	pub fn parse_environment_variables(environment_variable_prefix: Option<&str>) -> Result<Self, ConfigurationParseError>
	{
		let mut handle = unsafe { uninitialized() };
		
		let environment_variable_prefix = environment_variable_prefix.unwrap_or("UCX_");
		let status = if environment_variable_prefix.is_empty()
		{
			unsafe { ucp_config_read(null(), null(), &mut handle) }
		}
		else
		{
			let prefix = CString::new(environment_variable_prefix)?;
			unsafe { ucp_config_read(prefix.as_ptr(), null(), &mut handle) }
		};
		
		use self::Status::*;
		use self::ErrorCode::*;
		use self::ConfigurationParseError::*;
		
		match status.parse()
		{
			IsOk => Ok
			(
				Self
				{
					handle
				}
			),
			
			Error(error_code) => match error_code
			{
				NoSuchDevice | NoSuchElement => Err(NoTransportDevicesExistThatAreSuitable),
				
				FunctionNotImplemented | UnsupportedOperation => Err(FunctionalityNotImplementedOrSupported),
				
				OutOfMemory => panic!("Out of memory"),
				
				_ => panic!("Unexpected error for 'ucp_config_read' of '{}'", error_code),
			},
			
			UnknownErrorCode(_) | OperationInProgress => panic!("Status should not be possible for 'ucp_config_read'"),
		}
	}
}

impl UcpConfigurationWrapper
{
	/// Modify configuration.
	#[inline(always)]
	pub fn modify<Setting: UcpConfigurationSetting>(&self, configuration_setting: &Setting) -> Result<(), ConfigurationModifyError>
	{
		self.modify_(configuration_setting)
	}
	
	// Get a value: there is no API?
}
