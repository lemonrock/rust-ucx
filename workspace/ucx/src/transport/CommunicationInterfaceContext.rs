// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct CommunicationInterfaceContext(UnsafeCell<uct_iface>);

/// Tagged Messages (TM).
impl CommunicationInterfaceContext
{
	/// Post a tag to a transport interface.
	///
	/// Equivalent to `uct_iface_tag_recv_zcopy`.
	///
	/// This routine posts a tag to be matched on a transport interface.
	/// When a message with the corresponding tag arrives it is stored in the user buffer (described by `pointer_to_an_array_of_io_vec_structures` and `array_of_io_vec_structures_length`) directly.
	/// The operation completion is reported using callbacks on the `ctx` structure.
	///
	/// * `tag`: Tag to expect.
	/// * `tag_mask`: Mask which specifies what bits of the tag to compare.
	/// * `io_vec`: May be empty. Maximum length is uct_iface_attr.max_iov`.
	/// * `ctx`: Context associated with this particular tag. The `priv_` field in this structure is used to track the state internally.
	///
	/// Returns:-
	///
	/// * `UCS_OK`: The tag is posted to the transport.
	/// * `UCS_ERR_NO_RESOURCE`: Could not start the operation due to lack of resources.
	/// * `UCS_ERR_EXCEEDS_LIMIT`: No more room for tags in the transport.
	#[inline(always)]
	pub(crate) fn tagged_message_receive_zero_copy(&self, tag_matcher: TagMatcher, io_vec: &[uct_iov_t], ctx: &mut uct_tag_context) -> ucs_status_t
	{
		let tag = tag_matcher.value.0;
		let tag_mask = tag_matcher.bit_mask.0;
		
		unsafe { self.transport_interface_operations().iface_tag_recv_zcopy.unwrap()(self.0.get(), tag, tag_mask, io_vec.as_ptr(), io_vec.len(), ctx) }
	}
	
	/// Cancel a posted tag.
	///
	/// Equivalent to `uct_iface_tag_recv_cancel`.
	///
	/// This routine cancels tag: a, which was previously posted by `tagged_message_receive_zcopy`.
	/// The tag would be either matched canceled: or, in a time: bounded, regardless of the peer actions.
	/// The original completion callback of the tag would be called with the status if force is not set.
	///
	/// The tag would be either matched or canceled in a bounded time, regardless of the peer actions.
	/// The original completion callback of the tag would be called with the status if `force` is not set (`false`).
	///
	/// * `ctx`: Tag context which was used for posting the tag. If `force` is `false`, `ctx->completed_cb` will be called with either `UCS_OK` which means the tag was matched and data received despite the cancel request, or `UCS_ERR_CANCELED` which means the tag was successfully cancelled before it was matched.
	/// * `force`: Whether to report completions to `ctx->completed_cb`.
	///
	/// Returns:-
	/// * `UCS_OK`: If the tag is cancelled in the transport.
	#[inline(always)]
	pub(crate) fn tagged_message_receive_cancel(&self, ctx: &mut uct_tag_context, force: bool) -> ucs_status_t
	{
		unsafe { self.transport_interface_operations().iface_tag_recv_cancel.unwrap()(self.0.get(), ctx, force.to_c_bool()) }
	}
}

/// Resources (RESOURCE).
impl CommunicationInterfaceContext
{
	/// Enable synchronous progress for the interface.
	///
	/// Equivalent to `uct_iface_progress_enable`.
	///
	/// Notify the transport that it should actively progress communications during `Worker.progress()`.
	///
	/// NOTE: This function is not thread safe with respect to `Worker.progress()` unless the flag `uct_progress_types::THREAD_SAFE` is set in `flags`.
	///
	/// NOTE: The initial state is for progressing to be disabled.
	#[inline(always)]
	pub(crate) fn enable_progressing(&self, flags: uct_progress_types)
	{
		unsafe { self.transport_interface_operations().iface_progress_enable.unwrap()(self.iface(), flags.0) }
	}
	
	/// Disable synchronous progress for the interface.
	///
	/// Equivalent to `uct_iface_progress_disable`.
	///
	/// Notify the transport that it should not progress its communications during `Worker.progress()`.
	///
	/// This may improve the latency of other transports.
	///
	/// NOTE: This function is not thread safe with respect to `Worker.progress()` unless the flag `uct_progress_types::THREAD_SAFE` is set in `flags`.
	///
	/// NOTE: The initial state is for progressing to be disabled.
	#[inline(always)]
	pub(crate) fn disable_progressing(&self, flags: uct_progress_types)
	{
		unsafe { self.transport_interface_operations().iface_progress_disable.unwrap()(self.iface(), flags.0) }
	}
	
	/// Perform a progress on an interface.
	///
	/// Returns a number of events.
	///
	/// Equivalent to `uct_iface_progress`.
	#[inline(always)]
	pub(crate) fn progress(&self) -> u32
	{
		unsafe { self.transport_interface_operations().iface_progress.unwrap()(self.iface()) }
	}
	
	/// Flush outstanding communication operations on an interface.
	///
	/// Equivalent to `uct_iface_flush`.
	///
	/// Flushes all outstanding communications issued on the interface prior to this call.
	/// The operations are completed at the origin or at the target as well. The exact completion semantic depends on flags parameter; currently, only `uct_flush_flags::LOCAL` is supported. This flag guarantees that the data transfer is completed but the target buffer may not be updated yet.
	///
	///  * `flags`: See above.
	///  * `completion_handle`: Modified by this call. It can be null (which means that the call will return the current state of the interface and no completion will be generated in case of outstanding communications). If not-null, then the completion counter is decremented by one (1) when this call completes. The completion callback is called when the completion counter reaches zero (0).
	///
	/// Returns:-
	/// * `UCS_OK`: No outstanding communications left.
	/// * `UCS_INPROGRESS`: Some communication operations are still in progress. If Some() was provided for `completion_handle`, it will be updated upon completion of these operations.
	#[inline(always)]
	pub(crate) fn flush(&self, flags: uct_flush_flags, completion_handle: Option<&mut uct_completion>) -> ucs_status_t
	{
		debug_assert_eq!(flags, uct_flush_flags::LOCAL, "Only LOCAL is supported currently");
		
		unsafe { self.transport_interface_operations().iface_flush.unwrap()(self.iface(), flags.0, completion_handle.mutable_reference()) }
	}
	
	/// Ensures ordering of outstanding communications on the interface.
	///
	/// Equivalent to `uct_iface_fence`.
	///
	/// Operations issued on the interface prior to this call are guaranteed to be completed before any subsequent communication operations to the same interface which follow the call to `fence`.
	///
	/// Should only ever return `UCS_OK`.
	#[inline(always)]
	pub(crate) fn fence(&self) -> ucs_status_t
	{
		unsafe { self.transport_interface_operations().iface_fence.unwrap()(self.iface(), ReservedForFutureUseFlags) }
	}
}

impl CommunicationInterfaceContext
{
	/// Transport interface operations.
	#[inline(always)]
	pub(crate) fn transport_interface_operations(&self) -> &mut uct_iface_ops
	{
		&mut unsafe { &mut * self.iface()}.ops
	}
	
	#[inline(always)]
	fn iface(&self) -> *mut uct_iface
	{
		self.0.get()
	}
}
