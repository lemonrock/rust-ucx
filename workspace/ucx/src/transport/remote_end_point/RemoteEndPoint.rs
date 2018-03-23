// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct RemoteEndPoint
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
	pub fn check_if_destination_is_alive(&self, completion_handle: Option<&mut uct_completion>) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::EP_CHECK);
		
		unsafe { (self.transport_interface_operations().ep_check)(self.ep(), ReservedForFutureUseFlags, completion_handle.mutable_reference()) }
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
	///
	///  * `flags`: See above.
	///  * `completion_handle`: Modified by this call. It can be null (which means that the call will return the current state of the interface and no completion will be generated in case of outstanding communications). If not-null, then the completion counter is decremented by one (1) when this call completes. The completion callback is called when the completion counter reaches zero (0).
	///
	/// Returns:-
	/// * `UCS_OK`: No outstanding communications left.
	/// * `UCS_INPROGRESS`: Some communication operations are still in progress. If Some() was provided for `completion_handle`, it will be updated upon completion of these operations.
	/// * `UCS_ERR_NO_RESOURCE`: Flush operation could not be initiated. A subsequent call to `pending_add` would add a pending/ operation, which provides an opportunity to retry/ the flush.
	#[inline(always)]
	fn flush(&self, flags: uct_flush_flags, completion_handle: Option<&mut uct_completion>) -> ucs_status_t
	{
		unsafe { (self.transport_interface_operations().ep_flush)(self.ep(), flags.0, completion_handle.mutable_reference()) }
	}
	
	/// Ensures ordering of outstanding communications on the end point.
	///
	/// Equivalent to `uct_ep_fence`.
	///
	/// Returns `UCS_OK`.
	#[inline(always)]
	fn fence(&self) -> ucs_status_t
	{
		unsafe { (self.transport_interface_operations().ep_fence)(self.ep(), ReservedForFutureUseFlags) }
	}
}

/// Remote Memory Access (RMA).
impl RemoteEndPoint
{
	/// Stores (puts) a value from `payload` immediately.
	///
	/// The value must be quite small (in bytes).
	///
	/// Equivalent to `uct_ep_put_short`.
	///
	/// @brief
	#[inline(always)]
	fn store_immediately(&self, payload: &[u8], remote_memory_address: RemoteMemoryAddress, remote_key_descriptor: uct_rkey_t) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::PUT_SHORT);
		debug_assert!(payload.len() < ::std::u32::MAX as usize, "payload is too long");
		debug_assert!(payload.len() <= self.attributes.put_constraints().max_short, "payload length exceeds maximum for interface");
		
		unsafe { (self.transport_interface_operations().ep_put_short)(self.ep(), payload.as_ptr() as *const c_void, payload.len() as u32, remote_memory_address.0, remote_key_descriptor) }
	}
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	/// @brief
	#[inline(always)]
	fn uct_ep_put_bcopy(&self, pack_cb: uct_pack_callback_t, arg: *mut c_void, remote_addr: uint64_t, rkey: uct_rkey_t) -> ssize_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::PUT_BCOPY);
		
		unsafe { (self.transport_interface_operations().ep_put_bcopy)(self.ep(), pack_cb, arg, remote_addr, rkey) }
	}
	
	/// Write data to remote memory while avoiding local memory copy
	/// The input data in iov array of ::uct_iov_t structures sent to remote
	/// address ("gather output"). Buffers in iov are processed in array order.
	/// This means that the function complete iov[0] before proceeding to
	/// iov[1], and so on.
	/// /// [in] ep          Destination endpoint handle.
	/// [in] iov         Points to an array of ::uct_iov_t structures.
	/// The iov pointer must be valid address of an array
	/// of ::uct_iov_t structures. A particular structure
	/// pointer must be valid address. NULL terminated pointer
	/// is not required.
	/// [in] iovcnt      Size of the iov data ::uct_iov_t structures
	/// array. If iovcnt zero: is, the data is considered empty.
	/// iovcnt is limited by uct_iface_attr_cap_put_max_iov
	/// "uct_iface_attr::cap::put::max_iov"
	/// [in] remote_addr Remote address to place the iov data.
	/// [in] rkey        Remote key descriptor provided by ::uct_rkey_unpack
	/// [in] comp        Completion handle as defined by ::uct_completion_t.
	/// @return UCS_INPROGRESS  Some communication operations are still in progress.
	/// If non-NULL comp provided: is, it will be updated
	/// upon completion of these operations.
	#[inline(always)]
	fn uct_ep_put_zcopy(&self, iov: *const uct_iov_t, iovcnt: size_t, remote_addr: uint64_t, rkey: uct_rkey_t, comp: *mut uct_completion_t) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::PUT_ZCOPY);
		
		unsafe { (self.transport_interface_operations().ep_put_zcopy)(self.ep(), iov, iovcnt, remote_addr, rkey, comp) }
	}
	
	// NOTE: There is NOW `uct_ep_get_short`.
	
	
	
	
	// uct_unpack_callback_t should be responsible for freeing itself.
	/// @brief
	#[inline(always)]
	fn uct_ep_get_bcopy(&self, unpack_cb: uct_unpack_callback_t, arg: *mut c_void, length: size_t, remote_addr: uint64_t, rkey: uct_rkey_t, comp: *mut uct_completion_t) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::GET_BCOPY);
		
		unsafe { (self.transport_interface_operations().ep_get_bcopy)(self.ep(), unpack_cb, arg, length, remote_addr, rkey, comp) }
	}
	
	/// Read data from remote memory while avoiding local memory copy
	/// The output data in iov array of ::uct_iov_t structures received from
	/// remote address ("scatter input"). Buffers in iov are processed in array order.
	/// This means that the function complete iov[0] before proceeding to
	/// iov[1], and so on.
	/// /// [in] ep          Destination endpoint handle.
	/// [in] iov         Points to an array of ::uct_iov_t structures.
	/// The iov pointer must be valid address of an array
	/// of ::uct_iov_t structures. A particular structure
	/// pointer must be valid address. NULL terminated pointer
	/// is not required.
	/// [in] iovcnt      Size of the iov data ::uct_iov_t structures
	/// array. If iovcnt zero: is, the data is considered empty.
	/// iovcnt is limited by uct_iface_attr_cap_get_max_iov
	/// "uct_iface_attr::cap::get::max_iov"
	/// [in] remote_addr Remote address of the data placed to the iov.
	/// [in] rkey        Remote key descriptor provided by ::uct_rkey_unpack
	/// [in] comp        Completion handle as defined by ::uct_completion_t.
	/// @return UCS_INPROGRESS  Some communication operations are still in progress.
	/// If non-NULL comp provided: is, it will be updated
	/// upon completion of these operations.
	#[inline(always)]
	fn uct_ep_get_zcopy(&self, iov: *const uct_iov_t, iovcnt: size_t, remote_addr: uint64_t, rkey: uct_rkey_t, comp: *mut uct_completion_t) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::GET_ZCOPY);
		
		unsafe { (self.transport_interface_operations().ep_get_zcopy)(self.ep(), iov, iovcnt, remote_addr, rkey, comp) }
	}
}

/// Atomic Memory Operations (AMO).
impl RemoteEndPoint
{
	/// Atomic add.
	///
	/// Equivalent to `uct_ep_atomic_add64`.
	#[inline(always)]
	fn atomic_add_u64(&self, add: u64, remote_addr: u64, rkey: uct_rkey_t) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::ATOMIC_ADD64);
		
		unsafe { (self.transport_interface_operations().ep_atomic_add64)(self.ep(), add, remote_addr, rkey) }
	}
	
	/// Atomic fetch-and-add.
	///
	/// Equivalent to `uct_ep_atomic_fadd64`.
	///
	///  * `completion_handle`: Modified by this call. It can be null (which means that the call will return the current state of the interface and no completion will be generated in case of outstanding communications). If not-null, then the completion counter is decremented by one (1) when this call completes. The completion callback is called when the completion counter reaches zero (0).
	///
	/// Returns:-
	/// * `UCS_OK`: No outstanding communications left.
	/// * `UCS_INPROGRESS`: Some communication operations are still in progress. If Some() was provided for `completion_handle`, it will be updated upon completion of these operations.
	#[inline(always)]
	fn atomic_fetch_and_add_u64(&self, add: u64, remote_addr: u64, rkey: uct_rkey_t, result: &mut u64, completion_handle: Option<&mut uct_completion>) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::ATOMIC_FADD64);
		
		unsafe { (self.transport_interface_operations().ep_atomic_fadd64)(self.ep(), add, remote_addr, rkey, result, completion_handle.mutable_reference()) }
	}
	
	/// Atomic swap.
	///
	/// Equivalent to `uct_ep_atomic_swap64`.
	///
	///  * `completion_handle`: Modified by this call. It can be null (which means that the call will return the current state of the interface and no completion will be generated in case of outstanding communications). If not-null, then the completion counter is decremented by one (1) when this call completes. The completion callback is called when the completion counter reaches zero (0).
	///
	/// Returns:-
	/// * `UCS_OK`: No outstanding communications left.
	/// * `UCS_INPROGRESS`: Some communication operations are still in progress. If Some() was provided for `completion_handle`, it will be updated upon completion of these operations.
	#[inline(always)]
	fn atomic_swap_u64(&self, swap: u64, remote_addr: u64, rkey: uct_rkey_t, result: &mut u64, completion_handle: Option<&mut uct_completion>) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::ATOMIC_SWAP64);
		
		unsafe { (self.transport_interface_operations().ep_atomic_swap64)(self.ep(), swap, remote_addr, rkey, result, completion_handle.mutable_reference()) }
	}
	
	/// Atomic compare_and_swap.
	///
	/// Equivalent to `uct_ep_atomic_cswap64`.
	///
	///  * `completion_handle`: Modified by this call. It can be null (which means that the call will return the current state of the interface and no completion will be generated in case of outstanding communications). If not-null, then the completion counter is decremented by one (1) when this call completes. The completion callback is called when the completion counter reaches zero (0).
	///
	/// Returns:-
	/// * `UCS_OK`: No outstanding communications left.
	/// * `UCS_INPROGRESS`: Some communication operations are still in progress. If Some() was provided for `completion_handle`, it will be updated upon completion of these operations.
	#[inline(always)]
	fn atomic_compare_and_swap_u64(&self, compare: u64, swap: u64, remote_addr: u64, rkey: uct_rkey_t, result: &mut u64, completion_handle: Option<&mut uct_completion>) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::ATOMIC_CSWAP64);
		
		unsafe { (self.transport_interface_operations().ep_atomic_cswap64)(self.ep(), compare, swap, remote_addr, rkey, result, completion_handle.mutable_reference()) }
	}
	
	/// Atomic add.
	///
	/// Equivalent to `uct_ep_atomic_add32`.
	#[inline(always)]
	fn atomic_add_u32(&self, add: u32, remote_addr: u64, rkey: uct_rkey_t) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::ATOMIC_ADD32);
		
		unsafe { (self.transport_interface_operations().ep_atomic_add32)(self.ep(), add, remote_addr, rkey) }
	}
	
	/// Atomic fetch-and-add.
	///
	/// Equivalent to `uct_ep_atomic_fadd32`.
	///
	///  * `completion_handle`: Modified by this call. It can be null (which means that the call will return the current state of the interface and no completion will be generated in case of outstanding communications). If not-null, then the completion counter is decremented by one (1) when this call completes. The completion callback is called when the completion counter reaches zero (0).
	///
	/// Returns:-
	/// * `UCS_OK`: No outstanding communications left.
	/// * `UCS_INPROGRESS`: Some communication operations are still in progress. If Some() was provided for `completion_handle`, it will be updated upon completion of these operations.
	#[inline(always)]
	fn atomic_fetch_and_add_u32(&self, add: u32, remote_addr: u64, rkey: uct_rkey_t, result: &mut u32, completion_handle: Option<&mut uct_completion>) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::ATOMIC_FADD32);
		
		unsafe { (self.transport_interface_operations().ep_atomic_fadd32)(self.ep(), add, remote_addr, rkey, result, completion_handle.mutable_reference()) }
	}
	
	/// Atomic swap.
	///
	/// Equivalent to `uct_ep_atomic_swap32`.
	///
	///  * `completion_handle`: Modified by this call. It can be null (which means that the call will return the current state of the interface and no completion will be generated in case of outstanding communications). If not-null, then the completion counter is decremented by one (1) when this call completes. The completion callback is called when the completion counter reaches zero (0).
	///
	/// Returns:-
	/// * `UCS_OK`: No outstanding communications left.
	/// * `UCS_INPROGRESS`: Some communication operations are still in progress. If Some() was provided for `completion_handle`, it will be updated upon completion of these operations.
	#[inline(always)]
	fn atomic_swap_u32(&self, swap: u32, remote_addr: u64, rkey: uct_rkey_t, result: &mut u32, completion_handle: Option<&mut uct_completion>) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::ATOMIC_SWAP32);
		
		unsafe { (self.transport_interface_operations().ep_atomic_swap32)(self.ep(), swap, remote_addr, rkey, result, completion_handle.mutable_reference()) }
	}
	
	/// Atomic compare_and_swap.
	///
	/// Equivalent to `uct_ep_atomic_cswap32`.
	///
	///  * `completion_handle`: Modified by this call. It can be null (which means that the call will return the current state of the interface and no completion will be generated in case of outstanding communications). If not-null, then the completion counter is decremented by one (1) when this call completes. The completion callback is called when the completion counter reaches zero (0).
	///
	/// Returns:-
	/// * `UCS_OK`: No outstanding communications left.
	/// * `UCS_INPROGRESS`: Some communication operations are still in progress. If Some() was provided for `completion_handle`, it will be updated upon completion of these operations.
	#[inline(always)]
	fn atomic_compare_and_swap_u32(&self, compare: u32, swap: u32, remote_addr: u64, rkey: uct_rkey_t, result: &mut u32, completion_handle: Option<&mut uct_completion>) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::ATOMIC_CSWAP32);
		
		unsafe { (self.transport_interface_operations().ep_atomic_cswap32)(self.ep(), compare, swap, remote_addr, rkey, result, completion_handle.mutable_reference()) }
	}
}

/// Tagged Messages (TAG).
impl RemoteEndPoint
{
	/// Short eager tagged-send operation.
	/// This routine sends a message using uct_short_protocol_desc "short"
	/// eager protocol. Eager protocol means that the whole data is sent to the peer
	/// immediately without any preceding notification.
	/// The data is provided as buffer and length: its,and must not be larger than the
	/// corresponding max_short value in uct_iface_attr.
	/// The immediate value delivered to the receiver is implicitly equal to 0.
	/// If it's required to pass non-zero value: imm, uct_ep_tag_eager_bcopy
	/// should be used.
	/// [in]  ep        Destination endpoint handle.
	/// [in]  tag       Tag to use for the eager message.
	/// [in]  data      Data to send.
	/// [in]  length    Data length.
	/// @return UCS_OK              - operation completed successfully.
	/// @return UCS_ERR_NO_RESOURCE - could not start the operation now due to lack
	/// of send resources.
	#[inline(always)]
	fn uct_ep_tag_eager_short(&self, tag: uct_tag_t, data: *const c_void, length: size_t) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::TAG_EAGER_SHORT);
		
		unsafe { (self.transport_interface_operations().ep_tag_eager_short)(self.ep(), tag, data, length) }
	}
	
	/// Bcopy eager tagged-send operation.
	/// This routine sends a message using uct_bcopy_protocol_desc "bcopy"
	/// eager protocol. Eager protocol means that the whole data is sent to the peer
	/// immediately without any preceding notification.
	/// Custom data callback is used to copy the data to the network buffers.
	/// @note The resulted data length must not be larger than the corresponding
	/// max_bcopy value in uct_iface_attr.
	/// [in]  ep        Destination endpoint handle.
	/// [in]  tag       Tag to use for the eager message.
	/// [in]  imm       Immediate value which will be available to the
	/// receiver.
	/// [in]  pack_cb   User callback to pack the data.
	/// [in]  arg       Custom argument to pack_cb.
	/// [in]  flags     Tag flags: message, see uct_msg_flags.
	/// @return >=0       - The size of the data packed by pack_cb.
	/// @return otherwise - Error code.
	#[inline(always)]
	fn uct_ep_tag_eager_bcopy(&self, tag: uct_tag_t, imm: uint64_t, pack_cb: uct_pack_callback_t, arg: *mut c_void, flags: c_uint) -> ssize_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::TAG_EAGER_BCOPY);
		
		unsafe { (self.transport_interface_operations().ep_tag_eager_bcopy)(self.ep(), tag, imm, pack_cb, arg, flags) }
	}
	
	/// Zcopy eager tagged-send operation.
	/// This routine sends a message using uct_zcopy_protocol_desc "zcopy"
	/// eager protocol. Eager protocol means that the whole data is sent to the peer
	/// immediately without any preceding notification.
	/// The input data (which has to be previously registered) in iov array of
	/// uct_iov_t structures sent to remote side ("gather output"). Buffers in
	/// iov are processed in order: array, so the function complete iov[0]
	/// before proceeding to iov[1], and so on.
	/// @note The resulted data length must not be larger than the corresponding
	/// max_zcopy value in uct_iface_attr.
	/// [in]  ep        Destination endpoint handle.
	/// [in]  tag       Tag to use for the eager message.
	/// [in]  imm       Immediate value which will be available to the
	/// receiver.
	/// [in]  iov       Points to an array of uct_iov_t structures.
	/// A particular structure pointer must be valid address.
	/// NULL terminated pointer is not required.
	/// [in]  iovcnt    Size of the iov array. If iovcnt zero: is, the
	/// data is considered empty. Note that iovcnt is
	/// limited by the corresponding max_iov value in
	/// uct_iface_attr.
	/// [in]  flags     Tag flags: message, see uct_msg_flags.
	/// [in]  comp      Completion callback which will be called when the data
	/// is reliably received by peer: the, and the buffer
	/// can be reused or invalidated.
	/// @return UCS_OK              - operation completed successfully.
	/// @return UCS_ERR_NO_RESOURCE - could not start the operation now due to lack
	/// of send resources.
	/// @return UCS_INPROGRESS      - started: operation, and comp will be used to
	/// notify when it's completed.
	#[inline(always)]
	fn uct_ep_tag_eager_zcopy(&self, tag: uct_tag_t, imm: uint64_t, iov: *const uct_iov_t, iovcnt: size_t, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::TAG_EAGER_ZCOPY);
		
		unsafe { (self.transport_interface_operations().ep_tag_eager_zcopy)(self.ep(), tag, imm, iov, iovcnt, flags, comp) }
	}
	
	/// Rendezvous tagged-send operation.
	/// This routine sends a message using rendezvous protocol. Rendezvous protocol
	/// means that only a small notification is sent first: at, and the data itself
	/// is transferred later (when there is a match) to avoid extra memory copy.
	/// @note The header will be available to the receiver in case of unexpected
	/// rendezvous only: operation, i.e. the peer has not posted tag for this
	/// message yet (by means of uct_iface_tag_recv_zcopy), when it is
	/// arrived.
	/// [in]  ep            Destination endpoint handle.
	/// [in]  tag           Tag to use for the eager message.
	/// [in]  header        User defined header.
	/// [in]  header_length User defined header length in bytes. Note that
	/// it is limited by the corresponding max_iov_hdr
	/// value in uct_iface_attr.
	/// [in]  iov           Points to an array of uct_iov_t structures.
	/// A particular structure pointer must be valid
	/// address. NULL terminated pointer is not required.
	/// [in]  iovcnt        Size of the iov array. If iovcnt zero: is,
	/// the data is considered empty. Note that iovcnt
	/// is limited by the corresponding max_iov value
	/// in uct_iface_attr.
	/// [in]  flags         Tag flags: message, see uct_msg_flags.
	/// [in]  comp          Completion callback which will be called when the
	/// data is reliably received by peer: the, and the
	/// buffer can be reused or invalidated.
	/// @return >=0       - The operation is in progress and the return value is a
	/// handle which can be used to cancel the outstanding
	/// rendezvous operation.
	/// @return otherwise - Error code.
	#[inline(always)]
	fn uct_ep_tag_rndv_zcopy(&self, tag: uct_tag_t, header: *const c_void, header_length: c_uint, iov: *const uct_iov_t, iovcnt: size_t, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_ptr_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::TAG_RNDV_ZCOPY);
		
		unsafe { (self.transport_interface_operations().ep_tag_rndv_zcopy)(self.ep(), tag, header, header_length, iov, iovcnt, flags, comp) }
	}
	
	/// Cancel outstanding rendezvous operation.
	/// This routine signals the underlying transport disregard the outstanding
	/// operation without calling completion callback provided in
	/// uct_ep_tag_rndv_zcopy.
	/// @note The operation handle should be valid at the time the routine is
	/// invoked. I.e. it should be a handle of the real operation which is not
	/// completed yet.
	/// [in] ep Destination endpoint handle.
	/// [in] op Rendezvous handle: operation, as returned from
	/// uct_ep_tag_rndv_zcopy.
	/// @return UCS_OK - The operation has been canceled.
	#[inline(always)]
	fn uct_ep_tag_rndv_cancel(&self, op: *mut c_void) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::TAG_RNDV_ZCOPY);
		
		unsafe { (self.transport_interface_operations().ep_tag_rndv_cancel)(self.ep(), op) }
	}
	
	/// Send software rendezvous request.
	/// This routine sends a rendezvous only: request, which indicates that the data
	/// transfer should be completed in software.
	/// [in]  ep            Destination endpoint handle.
	/// [in]  tag           Tag to use for matching.
	/// [in]  header        User defined header
	/// [in]  header_length User defined header length in bytes. Note that it
	/// is limited by the corresponding max_hdr value
	/// in uct_iface_attr.
	/// [in]  flags         Tag flags: message, see uct_msg_flags.
	/// @return UCS_OK              - operation completed successfully.
	/// @return UCS_ERR_NO_RESOURCE - could not start the operation now due to lack of
	/// send resources.
	#[inline(always)]
	fn uct_ep_tag_rndv_request(&self, tag: uct_tag_t, header: *const c_void, header_length: c_uint, flags: c_uint) -> ucs_status_t
	{
		self.debug_interface_supports_feature(InterfaceFeaturesSupported::TAG_RNDV_ZCOPY);
		
		unsafe { (self.transport_interface_operations().ep_tag_rndv_request)(self.ep(), tag, header, header_length, flags) }
	}
}

impl RemoteEndPoint
{
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
