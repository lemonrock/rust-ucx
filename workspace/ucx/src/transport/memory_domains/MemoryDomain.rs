// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A memory domain.
///
/// All communications and memory operations are performed in the context of a specific memory domain.
///
/// Therefore one or more must be created before opening a `CommunicationInterfaceContext`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MemoryDomain(NonNull<uct_md>, Arc<MemoryDomainDropSafety>);

// TODO: Need to keep reference alive while CommIntContext is alive.

impl MemoryDomain
{
	/// Creates and opens a new memory domain.
	#[inline(always)]
	pub fn open(memory_domain_component: MemoryDomainComponent) -> Result<Self, ErrorCode>
	{
		let mut handle = unsafe { uninitialized() };
		
		let memory_domain_configuration = memory_domain_component.memory_domain_configuration()?;
		
		let status = unsafe { uct_md_open(memory_domain_component.name().as_ptr(), memory_domain_configuration.as_ptr(), &mut handle) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk =>
			{
				debug_assert!(!handle.is_null(), "handle is null");
				let handle = unsafe { NonNull::new_unchecked(handle) };
				Ok(MemoryDomain(handle, MemoryDomainDropSafety::new(handle)))
			}
			
			Error(error_code) => Err(error_code),
			
			_ => panic!("Unexpected status '{:?}'", status),
		}
	}
	
	/// Query.
	#[inline(always)]
	pub fn available_communication_interfaces(&self) -> Result<AvailableCommunicationInterfaces, ErrorCode>
	{
		AvailableCommunicationInterfaces::query(self)
	}
	
	#[inline(always)]
	pub(crate) fn as_ptr(&self) -> *mut uct_md
	{
		self.0.as_ptr()
	}
	
	#[inline(always)]
	pub(crate) fn drop_safety(&self) -> Arc<MemoryDomainDropSafety>
	{
		self.1.clone()
	}
	
	
	// uct_md_is_mem_type_owned
	// uct_md_is_sockaddr_accessible
	// uct_md_mkey_pack
	// uct_md_open
	// uct_md_query
	// uct_md_query_tl_resources  /   uct_release_tl_resource_list   uct_tl_resource_desc_t   uct_tl_resource_desc
	
	
	// uct_md_mem_advise
	// uct_md_mem_alloc
	// uct_md_mem_dereg
	// uct_md_mem_free
	// uct_md_mem_reg
}

