// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A wrapper around UCX configuration.
#[derive(Debug)]
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
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		let print_flags = ucs_config_print_flags_t::CONFIG | ucs_config_print_flags_t::DOC | ucs_config_print_flags_t::HEADER | ucs_config_print_flags_t::HIDDEN;
		
		let title = c_str!("UCP Configuration");
		unsafe { ucp_config_print(self.handle, stream, title.as_ptr(), print_flags) };
	}
}

impl UcxConfigurationWrapper
{
	/// Parses a new configuration from environment variables.
	#[inline(always)]
	pub fn parse_environment_variables(environment_variable_prefix: &str) -> Result<Self, ConfigurationParseError>
	{
		let mut handle = unsafe { uninitialized() };
		
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
	
//	/// applicationContextFeaturesIdeallySupported and contextWillBeSharedByMultipleWorkersFromDifferentThreads are programmer choices; how the code will be designed
//	/// tagSenderMask and estimatedNumberOfEndPoints are configuration / per-invocation choices
//	/// contextWillBeSharedByMultipleWorkersFromDifferentThreads should ideally be false
//	#[inline(always)]
//	pub fn initialiseApplicationContext(&self, applicationContextFeaturesIdeallySupported: &ApplicationContextFeaturesIdeallySupported, contextWillBeSharedByMultipleWorkersFromDifferentThreads: bool, tagSenderMask: u64, estimatedNumberOfEndPoints: usize) -> Result<ApplicationContext, ApplicationContextInitialisationError>
//	{
//		use ucp_params_field::*;
//
//		let parameters = ucp_params_t
//		{
//			field_mask: UCP_PARAM_FIELD_FEATURES as u64 | UCP_PARAM_FIELD_REQUEST_SIZE as u64 | UCP_PARAM_FIELD_REQUEST_INIT as u64 | UCP_PARAM_FIELD_REQUEST_CLEANUP as u64 | UCP_PARAM_FIELD_TAG_SENDER_MASK as u64 | UCP_PARAM_FIELD_MT_WORKERS_SHARED as u64 | UCP_PARAM_FIELD_ESTIMATED_NUM_EPS as u64,
//			features: applicationContextFeaturesIdeallySupported.as_u64(tagSenderMask),
//
//			// Really of use to MPI
//			request_size: 0, // reservedSpaceInNonBlockingRequests,
//			request_init: None,
//			request_cleanup: None,
//
//			tag_sender_mask: tagSenderMask,
//			mt_workers_shared: if contextWillBeSharedByMultipleWorkersFromDifferentThreads
//			{
//				1
//			}
//			else
//			{
//				0
//			},
//			estimated_num_eps: estimatedNumberOfEndPoints,
//		};
//
//		let mut context = unsafe { uninitialized() };
//
//		let status = unsafe { ucp_init_version(UCP_API_MAJOR, UCP_API_MINOR, &parameters, self.handle, &mut context) };
//
//		if likely(status.isOk())
//		{
//			Ok
//			(
//				ApplicationContext::new(context)
//			)
//		}
//		else
//		{
//			use self::UcpFailure::*;
//			use self::UcpPermanentFailureReason::*;
//			use self::ApplicationContextInitialisationError::*;
//
//			let failure = UcpFailure::convertError(status);
//
//			match failure
//			{
//				Permanent(reason) => match reason
//				{
//					OutOfMemory => panic!("Out of memory"),
//					UnimplementedFunctionality => Err(FunctionalityNotImplementedOrSupported),
//					UnsupportedSubSetOfFunctionality => Err(FunctionalityNotImplementedOrSupported),
//					_ => panic!("Permanent failure to initialise an application context because '{:?}'", reason)
//				},
//				_ => panic!("Inappropriate failure for UCP API '{}'", failure)
//			}
//		}
//	}
}
