// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A memory domain.
///
/// All communications and memory operations are performed in the context of a specific memory domain.
///
/// Therefore one or more must be created before opening a `CommunicationInterfaceContext`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MemoryDomain(NonNull<uct_md>, Arc<MemoryDomainDropSafety>, MemoryDomainComponentAndTransportLayer);

impl MemoryDomain
{
	/// Creates and opens a new memory domain.
	#[inline(always)]
	pub fn open(memory_domain_component: MemoryDomainComponentAndTransportLayer) -> Result<Self, ErrorCode>
	{
		let mut handle = unsafe { uninitialized() };
		
		let memory_domain_configuration = memory_domain_component.memory_domain_configuration()?;
		
		let status = unsafe { uct_md_open(memory_domain_component.memory_domain_component_name().as_ptr(), memory_domain_configuration.as_ptr(), &mut handle) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk =>
			{
				debug_assert!(!handle.is_null(), "handle is null");
				let handle = unsafe { NonNull::new_unchecked(handle) };
				Ok(MemoryDomain(handle, MemoryDomainDropSafety::new(handle), memory_domain_component))
			}
			
			Error(error_code) => Err(error_code),
			
			_ => panic!("Unexpected status '{:?}'", status),
		}
	}
	
	/// Creates and opens a new communication interface context.
	#[inline(always)]
	pub fn open_communication_interface_context<SCR: ServerConnectionRequest, E: ErrorHandler, UETM: UnexpectedTaggedMessageHandler, AT: ActiveMessageTracer, A0: ActiveMessageHandler, A1: ActiveMessageHandler, A2: ActiveMessageHandler, A3: ActiveMessageHandler, A4: ActiveMessageHandler, A5: ActiveMessageHandler, A6: ActiveMessageHandler, A7: ActiveMessageHandler, A8: ActiveMessageHandler, A9: ActiveMessageHandler, A10: ActiveMessageHandler, A11: ActiveMessageHandler, A12: ActiveMessageHandler, A13: ActiveMessageHandler, A14: ActiveMessageHandler, A15: ActiveMessageHandler, A16: ActiveMessageHandler, A17: ActiveMessageHandler, A18: ActiveMessageHandler, A19: ActiveMessageHandler, A20: ActiveMessageHandler, A21: ActiveMessageHandler, A22: ActiveMessageHandler, A23: ActiveMessageHandler, A24: ActiveMessageHandler, A25: ActiveMessageHandler, A26: ActiveMessageHandler, A27: ActiveMessageHandler, A28: ActiveMessageHandler, A29: ActiveMessageHandler, A30: ActiveMessageHandler, A31: ActiveMessageHandler>(&self, hyper_thread_index: ZeroBasedHyperThreadIndex, end_point_address: CommunicationInterfaceContextEndPointAddress<SCR>, error_handler: E, unexpected_tagged_message_handler: UETM, worker: *mut uct_worker) -> Result<Box<CommunicationInterfaceContext<SCR, E, UETM, AT, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20, A21, A22, A23, A24, A25, A26, A27, A28, A29, A30, A31>>, ErrorCode>
	{
		CommunicationInterfaceContext::open(hyper_thread_index, self, end_point_address, error_handler, unexpected_tagged_message_handler, worker)
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
	
	#[inline(always)]
	pub(crate) fn transport_layer(&self) -> &MemoryDomainComponentAndTransportLayer
	{
		&self.2
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

