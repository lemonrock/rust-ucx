// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A wrapper around UCX configuration.
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UcxConfigurationWrapper
{
	handle: *mut ucp_config_t,
}

impl Drop for UcxConfigurationWrapper
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_config_release(self.handle) };
	}
}

impl PrintInformation for UcxConfigurationWrapper
{
	const DebugName: &'static str = "UcxConfigurationWrapper";
	
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		let print_flags = ucs_config_print_flags_t::CONFIG | ucs_config_print_flags_t::DOC | ucs_config_print_flags_t::HEADER | ucs_config_print_flags_t::HIDDEN;
		
		let title = c_str!("UCP Configuration");
		unsafe { ucp_config_print(self.handle, stream, title.as_ptr(), print_flags) };
	}
}

impl Debug for UcxConfigurationWrapper
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		self.debug_fmt(f)
	}
}

impl UcxConfigurationWrapper
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
		
		match status.parse()?
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
	
	/// Modify configuration.
	#[inline(always)]
	pub fn modify<Setting: ConfigurationSetting>(&self, configuration_setting: &Setting) -> Result<(), ConfigurationModifyError>
	{
		let (name, value) = configuration_setting.name_and_value();
		
		let status = unsafe { ucp_config_modify(self.handle, name.as_ptr(), value.as_ptr()) };
		
		use self::Status::*;
		use self::ErrorCode::*;
		use self::ConfigurationModifyError::*;
		
		match status.parse()?
		{
			IsOk => Ok(()),
			
			Error(error_code) => match error_code
			{
				NoSuchElement =>
				{
					Err(NoSuchNamedKey(name.to_string_lossy().into_owned(), value))
				},
				
				InvalidParameter => Err(InvalidParameterNameOrValue(name.to_string_lossy().into_owned(), value)),
				
				OutOfMemory => panic!("Out of memory"),
				
				_ => panic!("Unexpected error for 'ucp_config_modify' of '{}'", error_code),
			},
			
			UnknownErrorCode(_) | OperationInProgress => panic!("Status should not be possible for 'ucp_config_modify'"),
		}
	}
	
	/// Creates a per-hyper thread application context.
	#[inline(always)]
	pub fn per_hyper_thread_application_context<MemoryCustomization: NonBlockingRequestMemoryCustomization>(&self, application_context_configuration: &ApplicationContextConfiguration) -> Result<HyperThreadContext, HyperThreadContextCreationError>
	{
		application_context_configuration.per_hyper_thread::<MemoryCustomization>(self)
	}
}
