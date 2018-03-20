// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


// Holds a reference to the memory domain to prevent it being closed.
#[derive(Debug)]
pub(crate) struct CommunicationInterfaceConfiguration(NonNull<uct_iface_config>, Arc<MemoryDomainDropSafety>);

impl Drop for CommunicationInterfaceConfiguration
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { uct_config_release(self.0.as_ptr() as *mut c_void) }
	}
}

impl CommunicationInterfaceConfiguration
{
	// `transport_layer_name` can be null for a client or server socket.
	#[inline(always)]
	fn read_from_environment(transport_layer_name: *const c_char, environment_variable_prefix: &CStr, memory_domain: &MemoryDomain) -> Result<Self, ErrorCode>
	{
		let mut handle = unsafe { uninitialized() };
		
		let unsupported_file_name = null_mut();
		
		let status = unsafe { uct_md_iface_config_read(memory_domain.as_ptr(), transport_layer_name, environment_variable_prefix.as_ptr(), unsupported_file_name, &mut handle) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk =>
			{
				debug_assert!(!handle.is_null(), "handle is null");
				let handle = unsafe { NonNull::new_unchecked(handle) };
				
				Ok(CommunicationInterfaceConfiguration(handle, memory_domain.drop_safety()))
			}
			
			Error(error_code) => Err(error_code),
			
			_ => panic!("Unexpected status '{:?}'", status),
		}
	}
	
	#[inline(always)]
	fn as_ptr(&self) -> *mut uct_iface_config
	{
		self.0.as_ptr()
	}
}
