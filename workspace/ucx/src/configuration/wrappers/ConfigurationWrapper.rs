// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A trait to worker with various UCX configuration approaches that are similar but differ slightly.
pub trait ConfigurationWrapper
{
	#[doc(hidden)]
	#[inline(always)]
	fn modify_<Setting: ConfigurationSetting>(&self, configuration_setting: &Setting) -> Result<(), ConfigurationModifyError>
	{
		let (name, value) = configuration_setting.name_and_value();
		
		let status = unsafe { self.ucx_config_modify(name.as_ptr(), value.as_ptr()) };
		
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
	
	#[doc(hidden)]
	#[inline(always)]
	unsafe fn ucx_config_modify(&self, name: *const c_char, value: *const c_char) -> ucs_status_t;
}
