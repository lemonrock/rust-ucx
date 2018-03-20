// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A memory domain.
///
/// All communications and memory operations are performed in the context of a specific memory domain.
///
/// Therefore one or more must be created before opening a `CommunicationInterfaceContext`.
pub struct MemoryDomain(*mut uct_md);

// TODO: Need to keep reference alive while CommIntContext is alive.

impl Drop for MemoryDomain
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if !self.0.is_null()
		{
			unsafe { uct_md_close(self.0) }
		}
	}
}

impl MemoryDomain
{
	/// Creates and opens a new memory domain.
	#[inline(always)]
	pub fn open(memory_domain_component: MemoryDomainComponent) -> Result<Self, ErrorCode>
	{
		let mut this = MemoryDomain(null_mut());
		
		let memory_domain_configuration = memory_domain_component.memory_domain_configuration()?;
		
		let status = unsafe { uct_md_open(memory_domain_component.name().as_ptr(), memory_domain_configuration.as_ptr(), &mut this.0) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(this),
			
			Error(error_code) => Err(error_code),
			
			_ => panic!("Unexpected status '{:?}'", status),
		}
	}
	
	
	// uct_md_iface_config_read
	
	// uct_md_is_mem_type_owned
	// uct_md_is_sockaddr_accessible
	// uct_md_mkey_pack
	// uct_md_open
	// uct_md_query
	// uct_md_query_tl_resources
	
	// uct_md_mem_advise
	// uct_md_mem_alloc
	// uct_md_mem_dereg
	// uct_md_mem_free
	// uct_md_mem_reg
}

