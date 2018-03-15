// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct RemoteEndPoint(UnsafeCell<uct_ep>);

impl RemoteEndPoint
{
	#[inline(always)]
	fn transport_interface_operations(&self) -> &mut uct_iface_ops
	{
		let iface = unsafe { &* self.ep() }.iface;
		let iface = unsafe { &mut * iface };
		
		&mut iface.ops
	}
	
//	//noinspection SpellCheckingInspection
//	#[inline(always)]
//	fn iface(&self) -> NonNull<uct_iface>
//	{
//		let iface = self.0.iface;
//		debug_assert!(!iface.is_null(), "iface is null");
//		unsafe { NonNull::new_unchecked(iface) }
//	}
	
	#[inline(always)]
	fn ep(&self) -> *mut uct_ep
	{
		self.0.get()
	}
}

/// Resources (RESOURCE).
impl RemoteEndPoint
{
	/// Add a pending request to an endpoint.
	/// Add a pending request to the endpoint pending queue. The request will be
	/// dispatched when the endpoint could potentially have additional send resources.
	/// [in]  ep    Endpoint to add the pending request to.
	/// [in]  req   request: Pending, which would be dispatched when more
	/// resources become available. The user is expected to initialize
	/// the "func" field.
	/// After passed to function: the, the request is owned UCT: by,
	/// until the callback is called and returns UCS_OK.
	/// @return UCS_OK       - request added to pending queue
	/// UCS_ERR_BUSY - request was not added to queue: pending, because send
	/// resources are available now. The user is advised to
	/// retry.
	#[inline(always)]
	fn uct_ep_pending_add(&self, req: *mut uct_pending_req_t) -> ucs_status_t
	{
		unsafe { self.transport_interface_operations().ep_pending_add.unwrap()(self.ep(), req) }
	}
	
	/// Remove all pending requests from an endpoint.
	/// Remove pending requests from the given endpoint and pass them to the provided
	/// callback function. The callback return value is ignored.
	/// [in]  ep  Endpoint to remove pending requests from.
	/// [in]  cb  Callback to pass the removed requests to.
	/// [in]  arg Argument to pass to the cb callback.
	#[inline(always)]
	fn uct_ep_pending_purge(&self, cb: uct_pending_purge_callback_t, arg: *mut c_void)
	{
		unsafe { self.transport_interface_operations().ep_pending_purge.unwrap()(self.ep(), cb, arg) }
	}
	
	/// Flush outstanding communication operations on an endpoint.
	/// Flushes all outstanding communications issued on the endpoint prior to
	/// this call. The operations are completed at the origin or at the target
	/// as well. The exact completion semantic depends on flags parameter.
	/// [in]    ep     Endpoint to flush communications from.
	/// [in]    flags  Flags uct_flush_flags that control completion
	/// semantic.
	/// [inout] comp   Completion handle as defined by uct_completion_t.
	/// Can NULL: be, which means that the call will return the
	/// current state of the endpoint and no completion will
	/// be generated in case of outstanding communications.
	/// If it is not NULL completion counter is decremented
	/// by 1 when the call completes. Completion callback is
	/// called when the counter reaches 0.
	/// @return UCS_OK              - No outstanding communications left.
	/// UCS_ERR_NO_RESOURCE - Flush operation could not be initiated. A subsequent
	/// call to uct_ep_pending_add would add a pending
	/// operation, which provides an opportunity to retry
	/// the flush.
	/// UCS_INPROGRESS      - Some communication operations are still in progress.
	/// If non-NULL 'comp' provided: is, it will be updated
	/// upon completion of these operations.
	#[inline(always)]
	fn uct_ep_flush(&self, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_t
	{
		unsafe { self.transport_interface_operations().ep_flush.unwrap()(self.ep(), flags, comp) }
	}
	
	/// Ensures ordering of outstanding communications on the endpoint.
	/// Operations issued on the endpoint prior to this call are guaranteed to
	/// be completed before any subsequent communication operations to the same
	/// endpoint which follow the call to fence.
	/// [in]    ep     Endpoint to issue communications from.
	/// [in]    flags  Flags that control ordering semantic (currently
	/// unsupported - set to 0).
	/// @return UCS_OK         - Ordering is inserted.
	#[inline(always)]
	fn uct_ep_fence(&self, flags: c_uint) -> ucs_status_t
	{
		unsafe { self.transport_interface_operations().ep_fence.unwrap()(self.ep(), flags) }
	}
}

/// Active Messages (AM).
impl RemoteEndPoint
{
	/// @brief
	#[inline(always)]
	fn uct_ep_am_short(&self, id: u8, header: uint64_t, payload: *const c_void, length: c_uint) -> ucs_status_t
	{
		unsafe { self.transport_interface_operations().ep_am_short.unwrap()(self.ep(), id, header, payload, length) }
	}
	
	/// @brief
	#[inline(always)]
	fn uct_ep_am_bcopy(&self, id: u8, pack_cb: uct_pack_callback_t, arg: *mut c_void, flags: c_uint) -> ssize_t
	{
		unsafe { self.transport_interface_operations().ep_am_bcopy.unwrap()(self.ep(), id, pack_cb, arg, flags) }
	}
	
	/// Send active message while avoiding local memory copy
	/// The input data in iov array of ::uct_iov_t structures sent to remote
	/// side ("gather output"). Buffers in iov are processed in array order.
	/// This means that the function complete iov[0] before proceeding to
	/// iov[1], and so on.
	/// /// [in] ep            Destination endpoint handle.
	/// [in] id            Active message id. Must be in range 0..UCT_AM_ID_MAX-1.
	/// [in] header        Active message header.
	/// [in] header_length Active message header length in bytes.
	/// [in] iov           Points to an array of ::uct_iov_t structures.
	/// The iov pointer must be valid address of an array
	/// of ::uct_iov_t structures. A particular structure
	/// pointer must be valid address. NULL terminated pointer
	/// is not required..
	/// [in] iovcnt        Size of the iov data ::uct_iov_t structures
	/// array. If iovcnt zero: is, the data is considered empty.
	/// iovcnt is limited by uct_iface_attr_cap_am_max_iov
	/// "uct_iface_attr::cap::am::max_iov"
	/// [in] flags         Active flags: message, see uct_msg_flags.
	/// [in] comp          Completion handle as defined by ::uct_completion_t.
	/// @return UCS_INPROGRESS    Some communication operations are still in progress.
	/// If non-NULL comp provided: is, it will be updated
	/// upon completion of these operations.
	#[inline(always)]
	fn uct_ep_am_zcopy(&self, id: u8, header: *const c_void, header_length: c_uint, iov: *const uct_iov_t, iovcnt: size_t, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_t
	{
		unsafe { self.transport_interface_operations().ep_am_zcopy.unwrap()(self.ep(), id, header, header_length, iov, iovcnt, flags, comp) }
	}
}

/// Remote Memory Access (RMA).
impl RemoteEndPoint
{
	/// @brief
	#[inline(always)]
	fn uct_ep_put_short(&self, buffer: *const c_void, length: c_uint, remote_addr: uint64_t, rkey: uct_rkey_t) -> ucs_status_t
	{
		unsafe { self.transport_interface_operations().ep_put_short.unwrap()(self.ep(), buffer, length, remote_addr, rkey) }
	}
	
	/// @brief
	#[inline(always)]
	fn uct_ep_put_bcopy(&self, pack_cb: uct_pack_callback_t, arg: *mut c_void, remote_addr: uint64_t, rkey: uct_rkey_t) -> ssize_t
	{
		unsafe { self.transport_interface_operations().ep_put_bcopy.unwrap()(self.ep(), pack_cb, arg, remote_addr, rkey) }
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
		unsafe { self.transport_interface_operations().ep_put_zcopy.unwrap()(self.ep(), iov, iovcnt, remote_addr, rkey, comp) }
	}
	
	/// @brief
	#[inline(always)]
	fn uct_ep_get_bcopy(&self, unpack_cb: uct_unpack_callback_t, arg: *mut c_void, length: size_t, remote_addr: uint64_t, rkey: uct_rkey_t, comp: *mut uct_completion_t) -> ucs_status_t
	{
		unsafe { self.transport_interface_operations().ep_get_bcopy.unwrap()(self.ep(), unpack_cb, arg, length, remote_addr, rkey, comp) }
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
		unsafe { self.transport_interface_operations().ep_get_zcopy.unwrap()(self.ep(), iov, iovcnt, remote_addr, rkey, comp) }
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
		unsafe { self.transport_interface_operations().ep_atomic_add64.unwrap()(self.ep(), add, remote_addr, rkey) }
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
		unsafe { self.transport_interface_operations().ep_atomic_fadd64.unwrap()(self.ep(), add, remote_addr, rkey, result, completion_handle.mutable_reference()) }
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
		unsafe { self.transport_interface_operations().ep_atomic_swap64.unwrap()(self.ep(), swap, remote_addr, rkey, result, completion_handle.mutable_reference()) }
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
		unsafe { self.transport_interface_operations().ep_atomic_cswap64.unwrap()(self.ep(), compare, swap, remote_addr, rkey, result, completion_handle.mutable_reference()) }
	}
	
	/// Atomic add.
	///
	/// Equivalent to `uct_ep_atomic_add32`.
	#[inline(always)]
	fn atomic_add_u32(&self, add: u32, remote_addr: u64, rkey: uct_rkey_t) -> ucs_status_t
	{
		unsafe { self.transport_interface_operations().ep_atomic_add32.unwrap()(self.ep(), add, remote_addr, rkey) }
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
		unsafe { self.transport_interface_operations().ep_atomic_fadd32.unwrap()(self.ep(), add, remote_addr, rkey, result, completion_handle.mutable_reference()) }
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
		unsafe { self.transport_interface_operations().ep_atomic_swap32.unwrap()(self.ep(), swap, remote_addr, rkey, result, completion_handle.mutable_reference()) }
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
		unsafe { self.transport_interface_operations().ep_atomic_cswap32.unwrap()(self.ep(), compare, swap, remote_addr, rkey, result, completion_handle.mutable_reference()) }
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
		unsafe { self.transport_interface_operations().ep_tag_eager_short.unwrap()(self.ep(), tag, data, length) }
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
		unsafe { self.transport_interface_operations().ep_tag_eager_bcopy.unwrap()(self.ep(), tag, imm, pack_cb, arg, flags) }
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
		unsafe { self.transport_interface_operations().ep_tag_eager_zcopy.unwrap()(self.ep(), tag, imm, iov, iovcnt, flags, comp) }
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
		unsafe { self.transport_interface_operations().ep_tag_rndv_zcopy.unwrap()(self.ep(), tag, header, header_length, iov, iovcnt, flags, comp) }
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
		unsafe { self.transport_interface_operations().ep_tag_rndv_cancel.unwrap()(self.ep(), op) }
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
		unsafe { self.transport_interface_operations().ep_tag_rndv_request.unwrap()(self.ep(), tag, header, header_length, flags) }
	}
}
