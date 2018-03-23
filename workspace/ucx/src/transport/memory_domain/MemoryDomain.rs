// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A memory domain.
///
/// All communications and memory operations are performed in the context of a specific memory domain.
///
/// Therefore one or more must be created before opening a `CommunicationInterfaceContext`.
#[derive(Debug, Clone)]
pub struct MemoryDomain
{
	handle: NonNull<uct_md>,
	handle_drop_safety: Arc<MemoryDomainHandleDropSafety>,
	memory_domain_component_and_transport_layer: MemoryDomainComponentAndTransportLayer,
	attributes: MemoryDomainAttributes,
}

impl HasAttributes for MemoryDomain
{
	type Attributes = MemoryDomainAttributes;
	
	#[inline(always)]
	fn attributes(&self) -> &Self::Attributes
	{
		&self.attributes
	}
}

impl MemoryDomain
{
	/// Creates and opens a new memory domain.
	#[inline(always)]
	pub fn open(memory_domain_component_and_transport_layer: MemoryDomainComponentAndTransportLayer) -> Result<Self, ErrorCode>
	{
		let mut handle = unsafe { uninitialized() };
		
		let memory_domain_configuration = memory_domain_component_and_transport_layer.memory_domain_configuration()?;
		
		let status = unsafe { uct_md_open(memory_domain_component_and_transport_layer.memory_domain_component_name().as_ptr(), memory_domain_configuration.as_ptr(), &mut handle) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk =>
			{
				debug_assert!(!handle.is_null(), "handle is null");
				let handle = unsafe { NonNull::new_unchecked(handle) };
				Ok
				(
					MemoryDomain
					{
						handle,
						handle_drop_safety: MemoryDomainHandleDropSafety::new(handle),
						memory_domain_component_and_transport_layer,
						attributes: MemoryDomainAttributes::query(handle),
					}
				)
			}
			
			Error(error_code) => Err(error_code),
			
			_ => panic!("Unexpected status '{:?}'", status),
		}
	}
	
	/// Creates and opens a new communication interface context for this thread.
	#[inline(always)]
	pub fn open_communication_interface_context_for_thread<SCR: ServerConnectionRequest, E: ErrorHandler, UETM: UnexpectedTaggedMessageHandler, AT: ActiveMessageTracer, A0: ActiveMessageHandler, A1: ActiveMessageHandler, A2: ActiveMessageHandler, A3: ActiveMessageHandler, A4: ActiveMessageHandler, A5: ActiveMessageHandler, A6: ActiveMessageHandler, A7: ActiveMessageHandler, A8: ActiveMessageHandler, A9: ActiveMessageHandler, A10: ActiveMessageHandler, A11: ActiveMessageHandler, A12: ActiveMessageHandler, A13: ActiveMessageHandler, A14: ActiveMessageHandler, A15: ActiveMessageHandler, A16: ActiveMessageHandler, A17: ActiveMessageHandler, A18: ActiveMessageHandler, A19: ActiveMessageHandler, A20: ActiveMessageHandler, A21: ActiveMessageHandler, A22: ActiveMessageHandler, A23: ActiveMessageHandler, A24: ActiveMessageHandler, A25: ActiveMessageHandler, A26: ActiveMessageHandler, A27: ActiveMessageHandler, A28: ActiveMessageHandler, A29: ActiveMessageHandler, A30: ActiveMessageHandler, A31: ActiveMessageHandler>(&self, end_point_address: CommunicationInterfaceContextEndPointAddress<SCR>, error_handler: E, unexpected_tagged_message_handler: UETM) -> Result<Box<CommunicationInterfaceContext<SCR, E, UETM, AT, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20, A21, A22, A23, A24, A25, A26, A27, A28, A29, A30, A31>>, ErrorCode>
	{
		CommunicationInterfaceContext::open(ZeroBasedHyperThreadIndex::for_current_hyper_thread(), self, end_point_address, error_handler, unexpected_tagged_message_handler, &ProgressEngine::create(None, true)?)
	}
	
	/// Creates and opens a new communication interface context.
	#[inline(always)]
	pub fn open_communication_interface_context<SCR: ServerConnectionRequest, E: ErrorHandler, UETM: UnexpectedTaggedMessageHandler, AT: ActiveMessageTracer, A0: ActiveMessageHandler, A1: ActiveMessageHandler, A2: ActiveMessageHandler, A3: ActiveMessageHandler, A4: ActiveMessageHandler, A5: ActiveMessageHandler, A6: ActiveMessageHandler, A7: ActiveMessageHandler, A8: ActiveMessageHandler, A9: ActiveMessageHandler, A10: ActiveMessageHandler, A11: ActiveMessageHandler, A12: ActiveMessageHandler, A13: ActiveMessageHandler, A14: ActiveMessageHandler, A15: ActiveMessageHandler, A16: ActiveMessageHandler, A17: ActiveMessageHandler, A18: ActiveMessageHandler, A19: ActiveMessageHandler, A20: ActiveMessageHandler, A21: ActiveMessageHandler, A22: ActiveMessageHandler, A23: ActiveMessageHandler, A24: ActiveMessageHandler, A25: ActiveMessageHandler, A26: ActiveMessageHandler, A27: ActiveMessageHandler, A28: ActiveMessageHandler, A29: ActiveMessageHandler, A30: ActiveMessageHandler, A31: ActiveMessageHandler>(&self, hyper_thread_index: ZeroBasedHyperThreadIndex, end_point_address: CommunicationInterfaceContextEndPointAddress<SCR>, error_handler: E, unexpected_tagged_message_handler: UETM, progress_engine: &ProgressEngine) -> Result<Box<CommunicationInterfaceContext<SCR, E, UETM, AT, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20, A21, A22, A23, A24, A25, A26, A27, A28, A29, A30, A31>>, ErrorCode>
	{
		CommunicationInterfaceContext::open(hyper_thread_index, self, end_point_address, error_handler, unexpected_tagged_message_handler, progress_engine)
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
		self.debug_assert_supports_feature(_bindgen_ty_1::SOCKADDR);
		
		let (addr, addrlen) = socket_address.suitable_for_ffi();
		let ucs_socket_address = ucs_sock_addr
		{
			addr,
			addrlen
		};
		
		unsafe { uct_md_is_sockaddr_accessible(self.as_ptr(), &ucs_socket_address, mode) }.from_c_bool()
	}
	
	/// Allocate a memory region for RMA and (optionally, `support_atomic_operations`) atomic operations.
	///
	/// Equivalent to `uct_md_mem_alloc`.
	///
	/// Allocated length may exceed `requested_length`.
	///
	/// If `faster_allocation_but_slower_access` is specified, then memory mapping will be deferred until it is accessed by the CPU or device, and memory locking will not occur.
	/// This is useful if using a memory allocation for a short period of time.
	///
	/// `requested_length` can not be zero (0).
	///
	/// `FIXED` allocations may not be supported by the underlying memory domain.
	///
	/// The underlying memory domain must support allocations (`ALLOC`).
	#[inline(always)]
	pub fn allocate_memory_region(&self, address_allocation_request: MemoryRegionAddressAllocationRequest, mut requested_length: usize, faster_allocation_but_slower_access: bool) -> Result<MemoryRegion, ErrorCode>
	{
		self.debug_assert_supports_feature(_bindgen_ty_1::ALLOC);
		debug_assert_ne!(requested_length, 0, "request_length can not be zero");
		#[cfg(debug_assertions)]
		{
			if address_allocation_request.is_fixed()
			{
				self.debug_assert_supports_feature(_bindgen_ty_1::FIXED);
			}
		}
		
		// Of the RMA / ATOMIC flags, only InfiniBand takes any notice of atomic; everything else is ignored.
		// So everything essentially assumes RMA.
		// Additionally, internally, uct checks that ACCESS_REMOTE_PUT, ACCESS_REMOTE_GET and ACCESS_REMOTE_ATOMIC are set.
		let mut flags = uct_md_mem_flags::ACCESS_ALL;
		
		let was_allocated_non_blocking = if faster_allocation_but_slower_access
		{
			flags |= uct_md_mem_flags::FLAG_NONBLOCK;
			true
		}
		else
		{
			flags |= uct_md_mem_flags::FLAG_LOCK;
			false
		};
		
		let (mut address, flags) = address_allocation_request.for_allocate(flags);
		
		let mut handle = unsafe { uninitialized() };
		
		let status = unsafe { uct_md_mem_alloc(self.as_ptr(), &mut requested_length, &mut address, flags.0, b"0".as_ptr() as *mut c_char, &mut handle) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk =>
			{
				debug_assert!(!address.is_null());
				debug_assert!(!handle.is_null());
				debug_assert_ne!(requested_length, 0);
				let handle = unsafe { NonNull::new_unchecked(handle) };
				Ok
				(
					MemoryRegion
					{
						handle,
						handle_drop_safety: MemoryRegionHandleDropSafety::new(handle, self.handle, self.handle_drop_safety()),
						address: unsafe { NonNull::new_unchecked(address as *mut u8) },
						length: requested_length,
						packed_memory_key: self.packed_memory_key(handle)?,
						memory_domain_handle: self.handle,
						#[cfg(debug_assertions)] was_allocated_non_blocking,
						#[cfg(debug_assertions)] memory_advice_is_supported: self.supports_feature(_bindgen_ty_1::ADVISE),
					}
				)
			},
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status)
		}
	}
	
	/// Register memory allocated externally to UCT for zero-copy sends and remote access.
	///
	/// Equivalent to `uct_md_mem_reg`.
	///
	/// Memory domains needs to support registrations.
	///
	/// `length` can not be zero (0).
	#[inline(always)]
	pub fn register_memory_for_zero_copy_sends_and_remote_access(&self, address: NonNull<u8>, length: usize) -> Result<MemoryRegistration, ErrorCode>
	{
		self.debug_assert_supports_feature(_bindgen_ty_1::REG);
		debug_assert_ne!(length, 0, "length can not be zero");
		
		// Of the RMA / ATOMIC flags, only InfiniBand takes any notice of atomic; everything else is ignored.
		// So everything essentially assumes RMA.
		// Additionally, internally, uct checks that ACCESS_REMOTE_PUT, ACCESS_REMOTE_GET and ACCESS_REMOTE_ATOMIC are set.
		const flags: uct_md_mem_flags = uct_md_mem_flags::ACCESS_ALL;
		
		let mut handle = unsafe { uninitialized() };
		
		let status = unsafe { uct_md_mem_reg(self.as_ptr(), address.as_ptr() as *mut c_void, length, flags.0, &mut handle) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk =>
			{
				debug_assert!(!handle.is_null());
				let handle = unsafe { NonNull::new_unchecked(handle) };
				Ok
				(
					MemoryRegistration
					{
						handle,
						handle_drop_safety: MemoryRegistrationHandleDropSafety::new(handle, self.handle, self.handle_drop_safety()),
						address,
						length,
						packed_memory_key: self.packed_memory_key(handle)?,
					}
				)
			}
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status)
		}
	}
	
	#[inline(always)]
	fn packed_memory_key(&self, memory_handle: NonNull<c_void>) -> Result<Vec<u8>, ErrorCode>
	{
		let packed_memory_key_length = self.attributes().packed_memory_key_length();
		let mut packed_memory_key = Vec::with_capacity(packed_memory_key_length);
		unsafe { packed_memory_key.set_len(packed_memory_key_length) };
		
		let status = unsafe { uct_md_mkey_pack(self.as_ptr(), memory_handle.as_ptr(), packed_memory_key.as_mut_ptr() as *mut c_void) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(packed_memory_key),
			
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
		self.handle.as_ptr()
	}
	
	#[inline(always)]
	pub(crate) fn handle_drop_safety(&self) -> Arc<MemoryDomainHandleDropSafety>
	{
		self.handle_drop_safety.clone()
	}
	
	#[inline(always)]
	pub(crate) fn transport_layer(&self) -> &MemoryDomainComponentAndTransportLayer
	{
		&self.memory_domain_component_and_transport_layer
	}
	
	#[inline(always)]
	fn debug_assert_supports_feature(&self, feature: _bindgen_ty_1)
	{
		debug_assert!(self.supports_feature(feature), "Does not support feature");
	}
	
	#[inline(always)]
	fn supports_feature(&self, feature: _bindgen_ty_1) -> bool
	{
		self.attributes().supports_feature(feature)
	}
}

