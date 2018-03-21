// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[derive(Debug)]
struct MemoryDomainComponentConfiguration(*mut uct_md_config);

impl Drop for MemoryDomainComponentConfiguration
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if !self.0.is_null()
		{
			unsafe { uct_config_release(self.0 as *mut c_void) }
		}
	}
}

impl MemoryDomainComponentConfiguration
{
	#[inline(always)]
	fn read_from_environment(memory_domain_component_name: &CStr, environment_variable_prefix: &CStr) -> Result<Self, ErrorCode>
	{
		let mut this = MemoryDomainComponentConfiguration(null_mut());
		
		let unsupported_file_name = null_mut();
		
		let status = unsafe { uct_md_config_read(memory_domain_component_name.as_ptr(), environment_variable_prefix.as_ptr(), unsupported_file_name, &mut this.0) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(this),
			
			Error(error_code) => Err(error_code),
			
			_ => panic!("Unexpected status '{:?}'", status),
		}
	}
	
	#[inline(always)]
	fn as_ptr(&self) -> *mut uct_md_config
	{
		self.0
	}
}
