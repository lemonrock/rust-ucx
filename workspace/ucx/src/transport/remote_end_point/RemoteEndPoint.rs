// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A remote end point.
#[derive(Debug)]
pub struct RemoteEndPoint
{
	handle: NonNull<uct_ep>,
	communication_interface_context_handle_drop_safety: Arc<CommunicationInterfaceContextHandleDropSafety>,
	attributes: CommunicationInterfaceContextAttributes,
}

impl Drop for RemoteEndPoint
{
	/// Destroys this remote end point.
	///
	/// Equivalent to `uct_ep_destroy`.
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { (self.transport_interface_operations().ep_destroy)(self.ep()) }
	}
}

impl RemoteEndPoint
{
	/// Create but do not connect to anything.
	///
	/// Equivalent to `uct_ep_create`.
	#[inline(always)]
	pub fn create(communication_interface_context: &CommunicationInterfaceContext) -> Result<Self, ErrorCode>
	{
		let mut handle = unsafe { uninitialized() };
		
		let status = unsafe { uct_ep_create(communication_interface_context.as_ptr(), &mut handle) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk =>
			{
				debug_assert!(!handle.is_null(), "handle is null");
				let handle = unsafe { NonNull::new_unchecked(handle) };
				
				Ok
				(
					Self
					{
						handle,
						communication_interface_context_handle_drop_safety: communication_interface_context.handle_drop_safety(),
						attributes: communication_interface_context.attributes().clone(),
					}
				)
			}
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
	
	/// Create and connect to remote communication interface context.
	///
	/// Equivalent to `uct_ep_create_connected`.
	///
	/// `remote_device_address` should be obtained from the remote `CommunicationInterfaceContext.get_device_address()`.
	///
	/// `remote_interface_address` should be obtained from the remote `CommunicationInterfaceContext.get_interface_address()`.
	///
	/// In a debug build the interface must support the `CONNECT_TO_IFACE` feature.
	#[inline(always)]
	pub fn create_connected(communication_interface_context: &CommunicationInterfaceContext, remote_device_address: &DeviceAddress, remote_interface_address: &InterfaceAddress) -> Result<Self, ErrorCode>
	{
		communication_interface_context.debug_interface_supports_feature(InterfaceFeaturesSupported::CONNECT_TO_IFACE);
		
		let mut handle = unsafe { uninitialized() };
		
		let status = unsafe { uct_ep_create_connected(communication_interface_context.as_ptr(), remote_device_address.is_reachable_address(), remote_interface_address.is_reachable_address(), &mut handle) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk =>
			{
				debug_assert!(!handle.is_null(), "handle is null");
				let handle = unsafe { NonNull::new_unchecked(handle) };
				
				Ok
				(
					Self
					{
						handle,
						communication_interface_context_handle_drop_safety: communication_interface_context.handle_drop_safety(),
						attributes: communication_interface_context.attributes().clone(),
					}
				)
			}
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
	
	//noinspection SpellCheckingInspection
	/// Connect this as a client to a remote server (end point).
	///
	/// Equivalent to `uct_ep_create_sockaddr`.
	///
	/// The user may provide private data to be sent on a connection request to the remote server.
	///
	/// Connection failures are ***not*** reported in the error code; instead, they will be initially successful and then reports to the ***CommunicationInterfaceContext*** error handler.
	///
	/// In a debug build the interface must support the `CONNECT_TO_SOCKADDR` feature.
	/// Currently, only the `RdmaCommunicationManager` memory domain supports this.
	///
	/// The `private_data_in_connection_request` has a maximum length (check `communication_interface_context.attributes().maximum_client_connection_request_private_data()`);
	#[inline(always)]
	pub fn create_client_connected_to_server(communication_interface_context: &CommunicationInterfaceContext, socket_address: &SocketAddress, private_data_in_connection_request: &[u8]) -> Result<Self, ErrorCode>
	{
		communication_interface_context.debug_interface_supports_feature(InterfaceFeaturesSupported::CONNECT_TO_SOCKADDR);
		debug_assert!(private_data_in_connection_request.len() <= communication_interface_context.attributes().maximum_client_connection_request_private_data(), "private_data_in_connection_request is longer than '{:?}'", communication_interface_context.attributes().maximum_client_connection_request_private_data());
		
		let mut handle = unsafe { uninitialized() };
		
		let (addr, addrlen) = socket_address.suitable_for_ffi();
		let socket_address = ucs_sock_addr
		{
			addr,
			addrlen,
		};
		
		let status = unsafe { uct_ep_create_sockaddr(communication_interface_context.as_ptr(), &socket_address, private_data_in_connection_request.as_ptr() as *const _, private_data_in_connection_request.len(), &mut handle) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk =>
			{
				debug_assert!(!handle.is_null(), "handle is null");
				let handle = unsafe { NonNull::new_unchecked(handle) };
				
				Ok
				(
					Self
					{
						handle,
						communication_interface_context_handle_drop_safety: communication_interface_context.handle_drop_safety(),
						attributes: communication_interface_context.attributes().clone(),
					}
				)
			}
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
	
	/// Connect an unconnected end point.
	///
	/// Equivalent to `uct_ep_connect_to_ep`.
	///
	/// `remote_device_address` should be obtained from the remote `CommunicationInterfaceContext.get_device_address()`.
	///
	/// `remote_end_point_address` should be obtained from the remote `RemoteEndPoint.get_end_point_address()`.
	///
	/// In a debug build the interface must support the `CONNECT_TO_EP` feature.
	#[inline(always)]
	pub fn connect_to(&self, remote_device_address: &DeviceAddress, remote_end_point_address: &EndPointAddress) -> Result<(), ErrorCode>
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::CONNECT_TO_EP);
		
		let status = unsafe { uct_ep_connect_to_ep(self.ep(), remote_device_address.is_reachable_address(), remote_end_point_address.is_reachable_address()) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(()),
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
	
	/// Get address of the end point.
	///
	/// Equivalent to `uct_ep_get_address`.
	pub fn get_end_point_address(&self) -> Result<EndPointAddress, ErrorCode>
	{
		let end_point_address = EndPointAddress::new(self.attributes.end_point_address_length());
		
		let status = unsafe { uct_ep_get_address(self.ep(), end_point_address.is_reachable_address() as *mut _) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(end_point_address),
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
	
	/// Check if the remote end point is alive with respect to the UCT library.
	///
	/// Equivalent to `uct_ep_check`.
	///
	///  * `completion_handle`: Modified by this call. It can be null (which means that the call will return the current state of the interface and no completion will be generated in case of outstanding communications). If not-null, then the completion counter is decremented by one (1) when this call completes. The completion callback is called when the completion counter reaches zero (0).
	#[inline(always)]
	pub fn check_if_destination_is_alive<C: CompletionHandler>(&self, completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::EP_CHECK);
		
		let status = unsafe { (self.transport_interface_operations().ep_check)(self.ep(), ReservedForFutureUseFlags, completion.to_raw_pointer()) };
		
		completion.parse_status(status)
	}
	
	/// Creates a simple wrapper to send active messages with a particular identifier.
	#[inline(always)]
	pub fn active_message_sending<'remote_end_point>(&'remote_end_point self, active_message_identifier: ActiveMessageIdentifier) -> ActiveMessageSendingRemoteEndPoint<'remote_end_point>
	{
		ActiveMessageSendingRemoteEndPoint::new(self, active_message_identifier, &self.attributes)
	}
	
	/// Creates a simple wrapper to send tagged messages with a particular tag value.
	#[inline(always)]
	pub fn tagged_message_sending<'remote_end_point>(&'remote_end_point self, tag_value: TagValue) -> TaggedMessageSendingRemoteEndPoint<'remote_end_point>
	{
		TaggedMessageSendingRemoteEndPoint::new(self, tag_value, &self.attributes)
	}
	
	/// Creates a simple wrapper to access remote memory.
	#[inline(always)]
	pub fn remote_memory_access<'remote_end_point>(&'remote_end_point self, unpacked_memory_key: &'remote_end_point UnpackedMemoryKey) -> RemoteMemoryAccessRemoteEndPoint<'remote_end_point>
	{
		RemoteMemoryAccessRemoteEndPoint::new(self, unpacked_memory_key, &self.attributes)
	}
}

/// Resources (RESOURCE).
impl RemoteEndPoint
{
	/// Add a pending request to the end point pending queue.
	///
	/// Equivalent to `uct_ep_pending_add`.
	///
	/// The request will be dispatched when the end point could potentially have additional send resources.
	///
	/// The `pending_request` will be dispatched when more resources become available.
	/// The user is expected to initialize the `func` field.
	///
	/// After the `pending_request` is passed to the function, the request is owned by UCT until the callback is called and returns UCS_OK.
	///
	/// Returns:-
	///
	/// * `UCS_OK`: Request added to pending queue.
	/// * `UCS_ERR_BUSY`: Send resources are not available; retry.
	#[inline(always)]
	fn pending_add(&self, pending_request: Box<uct_pending_req>) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::PENDING);
		
		unsafe { (self.transport_interface_operations().ep_pending_add)(self.ep(), Box::into_raw(pending_request)) }
	}
	
	/// Remove all pending requests from an end point and pass them to the provided `purge_callback`.
	///
	/// Equivalent to `uct_ep_pending_purge`.
	///
	/// Remove pending requests from the given endpoint and pass them to the provided callback function.
	/// The callback return value is ignored.
	#[inline(always)]
	fn pending_purge(&self, callback_to_pass_removed_requests_to: unsafe extern "C" fn(removed_pending_request: *mut uct_pending_req_t, callback_context: *mut c_void), callback_context: *mut c_void)
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::PENDING);
		
		unsafe { (self.transport_interface_operations().ep_pending_purge)(self.ep(), callback_to_pass_removed_requests_to, callback_context) }
	}
	
	/// Flush outstanding communication operations issued on this end prior to this call.
	///
	/// Equivalent to `uct_ep_flush`.
	///
	/// The operations are completed at the origin or at the target as well.
	///
	/// The exact completion semantic depends on the `flags` parameter.
	#[inline(always)]
	pub fn flush<C: CompletionHandler>(&self, flags: uct_flush_flags, completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		let status = unsafe { (self.transport_interface_operations().ep_flush)(self.ep(), flags.0, completion.to_raw_pointer()) };
		
		completion.parse_status(status)
	}
	
	/// Ensures ordering of outstanding communications on the end point.
	///
	/// Equivalent to `uct_ep_fence`.
	#[inline(always)]
	pub fn fence(&self) -> Result<(), ErrorCode>
	{
		let status = unsafe { (self.transport_interface_operations().ep_fence)(self.ep(), ReservedForFutureUseFlags) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(()),
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status, '{:?}'", unexpected_status),
		}
	}
	
	/// Cancel outstanding rendezvous operation.
	///
	/// Equivalent to `uct_ep_tag_rndv_cancel`
	///
	/// The completion callback is not called.
	///
	/// Called via `RendezvousCancellation.cancel()` and so ***not public***.
	#[inline(always)]
	fn cancel_rendezvous_zero_copy_tagged_message(&self, cancellation_handle: *mut c_void) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::TAG_RNDV_ZCOPY);
		
		unsafe { (self.transport_interface_operations().ep_tag_rndv_cancel)(self.ep(), cancellation_handle) }
	}
	
	#[inline(always)]
	fn transport_interface_operations(&self) -> &mut uct_iface_ops
	{
		let ep = unsafe { self.handle.as_ref() };
		let iface = unsafe { &mut * ep.iface };
		&mut iface.ops
	}
	
	#[inline(always)]
	fn ep(&self) -> *mut uct_ep
	{
		self.handle.as_ptr()
	}
	
	#[inline(always)]
	fn debug_interface_supports_feature(&self, required_to_support: InterfaceFeaturesSupported)
	{
		debug_assert!(self.attributes.supports_all_of(required_to_support), "Unsupported");
	}
}
