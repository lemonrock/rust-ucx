// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A memory domain.
///
/// All communications and memory operations are performed in the context of a specific memory domain.
///
/// Therefore one or more must be created before opening a `CommunicationInterfaceContext`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MemoryDomain(NonNull<uct_md>, Arc<MemoryDomainDropSafety>, MemoryDomainComponentAndTransportLayer);

impl HasAttributes for MemoryDomain
{
	type Attributes = MemoryDomainAttributes;
	
	#[inline(always)]
	fn attributes(&self) -> Self::Attributes
	{
		Self::Attributes::query(self.0)
	}
}

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
	
	/// Obtains a packed memory key (rkey).
	#[inline(always)]
	pub fn packed_memory_key(&self, memory_handle: uct_mem_h) -> Result<PackedMemoryKey, ErrorCode>
	{
		let packed_memory_key = PackedMemoryKey::new(self.attributes().packed_memory_key_length());
		
		let status = unsafe { uct_md_mkey_pack(self.as_ptr(), memory_handle, packed_memory_key.address().as_ptr() as *mut c_void) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(packed_memory_key),
			
			Error(error_code) => Err(error_code),
			
			_ => panic!("Unexpected status '{:?}'", status),
		}
	}
	
	/// Does this memory domain own this memory?
	#[inline(always)]
	pub fn owns_memory(&self, does_it_own_memory_at_address: NonNull<u8>, does_it_own_memory_length: usize) -> bool
	{
		unsafe { uct_md_is_mem_type_owned(self.as_ptr(), does_it_own_memory_at_address.as_ptr() as *mut c_void, does_it_own_memory_length) }.from_c_bool()
	}
	
	//noinspection SpellCheckingInspection
	/// Is this socket address accessible?
	#[inline(always)]
	pub fn is_socket_address_accessible(&self, socket_address: &SocketAddress, mode: uct_sockaddr_accessibility_t) -> bool
	{
		debug_assert!(self.attributes().supports_feature(_bindgen_ty_1::SOCKADDR), "Does not support socket addresses");
		
		let (addr, addrlen) = socket_address.suitable_for_ffi();
		let ucs_socket_address = ucs_sock_addr
		{
			addr,
			addrlen
		};
		
		unsafe { uct_md_is_sockaddr_accessible(self.as_ptr(), &ucs_socket_address, mode) }.from_c_bool()
	}
	
	// ?UCT_MD_FLAG_SOCKADDR?
	
	// uct_mem_alloc   UCT_MD_MEM_FLAG_FIXED
	// uct_mem_free
	
	// uct_md_mem_alloc   UCT_MD_FLAG_ALLOC
	// uct_md_mem_free
	// uct_md_mem_advise      UCT_MD_FLAG_ADVISE
	// uct_md_mem_reg
	// uct_md_mem_dereg
	
	// uct_iface_mem_alloc    uct_md_mem_flags
	// uct_iface_mem_free
	
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
}

