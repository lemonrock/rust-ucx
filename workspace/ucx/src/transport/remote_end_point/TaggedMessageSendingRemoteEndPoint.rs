// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Logic separation for sending tagged messages with a particular tag value.
///
/// Dereferences to `RemoteEndPoint` to provide access to `flush()`, `fence()`, `progress_thread_unsafe()`, etc.
#[derive(Debug, Clone)]
pub struct TaggedMessageSendingRemoteEndPoint<'remote_end_point>
{
	remote_end_point: &'remote_end_point RemoteEndPoint,
	tag_value: TagValue,
	
	supports_eager_short: bool,
	maximum_eager_short_length: usize,
	
	supports_eager_buffered_copy: bool,
	maximum_eager_buffered_copy_length: usize,
	
	supports_eager_zero_copy: bool,
	maximum_eager_zero_copy_length: usize,
	maximum_eager_zero_copy_from_local_memory_items: usize,
	
	supports_rendezvous_zero_copy: bool,
	maximum_rendezvous_header_length: usize,
	maximum_rendezvous_zero_copy_length: usize,
	maximum_rendezvous_zero_copy_from_local_memory_items: usize,
}

impl<'remote_end_point> Deref for TaggedMessageSendingRemoteEndPoint<'remote_end_point>
{
	type Target = RemoteEndPoint;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.remote_end_point
	}
}

impl<'remote_end_point> TaggedMessageSendingRemoteEndPoint<'remote_end_point>
{
	const UpperLimit: usize = ::std::u32::MAX as usize;
	
	const SizeOrStatusLowerLimit: isize = ::std::i8::MIN as isize;
	
	const SignalledMessagesAreNotSupportedFlags: uct_msg_flags = uct_msg_flags(0);
	
	#[inline(always)]
	fn new(remote_end_point: &'remote_end_point RemoteEndPoint, tag_value: TagValue, attributes: &CommunicationInterfaceContextAttributes) -> Self
	{
		let tagged_message_constraints = attributes.tagged_message_constraints();
		
		Self
		{
			remote_end_point,
			tag_value,
			
			supports_eager_short: attributes.supports_all_of(InterfaceFeaturesSupported::TAG_EAGER_SHORT),
			maximum_eager_short_length: tagged_message_constraints.eager.max_short,
			
			supports_eager_buffered_copy: attributes.supports_all_of(InterfaceFeaturesSupported::TAG_EAGER_BCOPY),
			maximum_eager_buffered_copy_length: tagged_message_constraints.eager.max_bcopy,
			
			supports_eager_zero_copy: attributes.supports_all_of(InterfaceFeaturesSupported::TAG_EAGER_ZCOPY),
			maximum_eager_zero_copy_length: tagged_message_constraints.eager.max_zcopy,
			maximum_eager_zero_copy_from_local_memory_items: tagged_message_constraints.eager.max_iov,
			
			supports_rendezvous_zero_copy: attributes.supports_all_of(InterfaceFeaturesSupported::TAG_RNDV_ZCOPY),
			maximum_rendezvous_header_length: tagged_message_constraints.rndv.max_hdr,
			maximum_rendezvous_zero_copy_length: tagged_message_constraints.rndv.max_zcopy,
			maximum_rendezvous_zero_copy_from_local_memory_items: tagged_message_constraints.rndv.max_iov,
		}
	}
	
	/// Maximum.
	#[inline(always)]
	pub fn maximum_eager_zero_copy_from_local_memory_items(&self) -> usize
	{
		self.maximum_eager_zero_copy_from_local_memory_items
	}
	
	/// Maximum.
	#[inline(always)]
	pub fn maximum_rendezvous_zero_copy_from_local_memory_items(&self) -> usize
	{
		self.maximum_rendezvous_zero_copy_from_local_memory_items
	}
	
	/// Is within limits.
	///
	/// Unlike active messages and remote memory access operations, it is permissible `from_local_memory.is_empty()` to be true, ie there are nto items.
	#[inline(always)]
	pub fn is_less_than_number_of_items_and_maximum_size_for_eager(&self, from_local_memory: &[ZeroCopyIoVector]) -> bool
	{
		let number_of_items = from_local_memory.len();
		if number_of_items > self.maximum_eager_zero_copy_from_local_memory_items
		{
			return false;
		}
		
		let total_length = ZeroCopyIoVector::total_length(from_local_memory);
		total_length <= self.maximum_eager_zero_copy_length
	}
	
	/// Is within limits.
	#[inline(always)]
	pub fn is_inclusively_within_number_of_items_and_maximum_and_minimum_size_for_rendezvous(&self, from_local_memory: &[ZeroCopyIoVector]) -> bool
	{
		let number_of_items = from_local_memory.len();
		if number_of_items > self.maximum_rendezvous_zero_copy_from_local_memory_items
		{
			return false;
		}
		
		let total_length = ZeroCopyIoVector::total_length(from_local_memory);
		total_length <= self.maximum_rendezvous_zero_copy_length
	}
	
	/// Send an immediate ('short') tagged message using the eager protocol.
	///
	/// Equivalent to `uct_ep_tag_eager_short`.
	///
	/// This routine sends a tagged message using the eager protocol.
	///
	/// The eager protocol sends all of the message's data to the peer without any preceding notification.
	///
	/// Tagged messages are received using `tagged_message_receive_zero_copy` and the `unexpected_tagged_message_handler` provided on construction of `CommunicationInterfaceContext`.
	///
	/// Internally, an `immediate` value is also sent and is always zero (0).
	///
	/// May return an error of `NoResourcesAreAvailableToInitiateTheOperation`, in which case the operation can be tried again.
	#[inline(always)]
	pub fn send_an_immediate_eager_tagged_message(&self, immediate_value: u64, from_local_memory: &[u8]) -> Result<(), ErrorCode>
	{
		debug_assert!(self.supports_eager_short, "Interface does not support immediate ('short') eager tagged messages");
		debug_assert!(from_local_memory.len() <= Self::UpperLimit, "from_local_memory length '{}' exceeds upper limit '{}'", from_local_memory.len(), Self::UpperLimit);
		debug_assert!(from_local_memory.len() <= self.maximum_eager_short_length, "from_local_memory length '{}' exceeds maximum for interface '{}'", from_local_memory.len(), self.maximum_eager_short_length);
		
		assert_eq!(immediate_value, 0, "Values other than zero (0) are not supported for immediate value at this time for eager immediate tagged messages");
		
		let status = unsafe { (self.transport_interface_operations().ep_tag_eager_short)(self.ep(), self.tag_value.0, from_local_memory.as_ptr() as *const c_void, from_local_memory.len()) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(()),
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
	
	/// Send a buffered copy ('bcopy') tagged message using the eager protocol.
	///
	/// Equivalent to `uct_ep_tag_eager_bcopy`.
	///
	/// bcopy == buffered copy-and-send.
	///
	/// The eager protocol sends all of the message's data to the peer without any preceding notification.
	///
	/// Tagged messages are received using `tagged_message_receive_zero_copy` and the `unexpected_tagged_message_handler` provided on construction of `CommunicationInterfaceContext`.
	///
	/// Returns serialized (packed) length.
	///
	/// Sending signalled messages (using uct_msg_flags) is not supported (only shared memory and loopback devices support it in any event).
	///
	/// * `from_local_memory`: Will be specified a buffer that must not exceed `maximum_buffered_copy_length`.
	///
	/// May return an error of `NoResourcesAreAvailableToInitiateTheOperation`, in which case the operation can be tried again.
	#[inline(always)]
	pub fn send_a_buffered_copy_eager_tagged_message<BCS: BufferedCopySerializer>(&self, immediate_value: u64, from_local_memory: Box<BCS>) -> Result<usize, ErrorCode>
	{
		debug_assert!(self.supports_eager_buffered_copy, "Interface does not support buffered copy ('bcopy') eager tagged messages");
		
		let size_or_status = unsafe { (self.transport_interface_operations().ep_tag_eager_bcopy)(self.ep(), self.tag_value.0, immediate_value, buffered_copy_serialize_callback::<BCS>, Box::into_raw(from_local_memory) as *mut c_void, Self::SignalledMessagesAreNotSupportedFlags.0) };
		
		if size_or_status >= 0
		{
			let size = size_or_status as usize;
			debug_assert!(size <= self.maximum_eager_buffered_copy_length, "size '{}' exceeds maximum for interface '{}'", size, self.maximum_eager_buffered_copy_length);
			Ok(size)
		}
		else
		{
			use self::Status::*;
			
			debug_assert!(size_or_status >= Self::SizeOrStatusLowerLimit, "size_or_status is out-of-range");
			let status: ucs_status_t = unsafe { transmute(size_or_status as i8) };
			
			match status.parse()
			{
				Error(error_code) => Err(error_code),
				
				unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
			}
		}
	}
	
	/// Send an tagged message while avoiding local memory copy, ie by 'zero copy', using the eager protocol.
	///
	/// Equivalent to `uct_ep_tag_eager_zcopy`.
	///
	/// The eager protocol sends all of the message's data to the peer without any preceding notification.
	///
	/// Tagged messages are received using `tagged_message_receive_zero_copy` and the `unexpected_tagged_message_handler` provided on construction of `CommunicationInterfaceContext`.
	///
	/// `from_local_memory` memory must have been registered with, or allocated by, the memory domain for this communication interface for this remote end point.
	///
	/// If not complete (likely), then this method then needs `progress()` to be called on the ProgressEngine.
	///
	/// * `from_local_memory` maximum length is `self.maximum_zero_copy_from_local_memory_items()`. `UCT_IB_MAX_IOV` is 8.
	///
	/// Sending signalled messages (using uct_msg_flags) is not supported (only shared memory and loopback devices support it in any event).
	///
	/// May return an error of `NoResourcesAreAvailableToInitiateTheOperation`, in which case the operation can be tried again (this can be discovered in the completion handler currently, which is maybe not ideal).
	#[inline(always)]
	pub fn send_a_zero_copy_eager_tagged_message<C: CompletionHandler>(&self, immediate_value: u64, from_local_memory: &[ZeroCopyIoVector], completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		debug_assert!(self.supports_eager_zero_copy, "Interface does not support zero copy ('zcopy') eager tagged messages");
		debug_assert!(self.is_less_than_number_of_items_and_maximum_size_for_eager(from_local_memory), "from_local_memory has too many items, or is too large in total length");
		
		let status = unsafe { (self.transport_interface_operations().ep_tag_eager_zcopy)(self.ep(), self.tag_value.0, immediate_value, from_local_memory.as_ptr() as * const uct_iov_t, from_local_memory.len(), Self::SignalledMessagesAreNotSupportedFlags.0, completion.to_raw_pointer()) };
		
		completion.parse_status(status)
	}
	
	/// Send a rendezvous notification (request) tagged message.
	///
	/// Equivalent to `uct_ep_tag_rndv_request`.
	///
	/// The rendezvous protocol sends only a small notification message containing just a `header` and no actual payload data.
	/// If a receiving peer does not want a particular tagged message, then this saves substantial bandwidth.
	/// The protocol allows a send to optionally specify where the message payload data can be obtain from.
	///
	/// This operation sends a `notification request`, that **does not specify*** any protocol-aware details of how to obtain the message data.
	///
	/// Tagged messages are received using `tagged_message_receive_zero_copy` and the `unexpected_tagged_message_handler` provided on construction of `CommunicationInterfaceContext`.
	///
	/// It is up to the sender and received to agree a structure for the `header` that might contain this.
	///
	/// A typical use case is where a sender wants to tell a receiver about a data structure, but only knows its metadata. The actual payload, or bulk of the content, is stored on a storage server separate to both the sender and receiver. The receiver can then opt whether to download the payload, depending on what's in the metadata.
	///
	/// May return an error of `NoResourcesAreAvailableToInitiateTheOperation`, in which case the operation can be tried again (this can be discovered in the completion handler currently, which is maybe not ideal).
	#[inline(always)]
	pub fn send_a_rendezvous_notification_tagged_message(&self, header: &[u8]) -> Result<(), ErrorCode>
	{
		debug_assert!(header.len() <= Self::UpperLimit, "header length '{}' exceeds upper limit '{}'", header.len(), Self::UpperLimit);
		debug_assert!(header.len() <= self.maximum_rendezvous_header_length, "header length '{}' exceeds maximum for interface '{}'", header.len(), self.maximum_rendezvous_header_length);
		
		let status = unsafe { (self.transport_interface_operations().ep_tag_rndv_request)(self.ep(), self.tag_value.0, header.as_ptr() as *const c_void, header.len() as u32, Self::SignalledMessagesAreNotSupportedFlags.0) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(()),
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
	
	/// Send an active message while avoiding local memory copy, ie by 'zero copy', using the rendezvous protocol.
	///
	/// Equivalent to `uct_ep_tag_rndv_zcopy`.
	///
	/// The rendezvous protocol sends only a small notification message containing just a `header` and no actual payload data.
	/// If a receiving peer does not want a particular tagged message, then this saves substantial bandwidth.
	/// The protocol allows a send to optionally specify where the message payload data can be obtain from.
	///
	/// This operation **does specify*** protocol-aware details of how to obtain the message data.
	///
	/// Tagged messages are received using `tagged_message_receive_zero_copy` and the `unexpected_tagged_message_handler` provided on construction of `CommunicationInterfaceContext`.
	///
	/// `header` and `from_local_memory` memory must have been registered with, or allocated by, the memory domain for this communication interface for this remote end point.
	///
	/// This method then needs `progress()` to be called on the ProgressEngine. It also returns a cancellation handle, which can be used to cancel this operation (cancellation does not occur if this returned value is dropped). The cancellation handle is invalid once the operation has completed.
	///
	/// * `header` may be no longer than `self.maximum_zero_copy_header_length()`.
	/// * `from_local_memory` maximum length is `self.maximum_zero_copy_from_local_memory_items()`. `UCT_IB_MAX_IOV` is 8, and one IoVector seems to be used internally for the header, implying 7 is the normal maximum.
	///
	/// Sending signalled messages (using uct_msg_flags) is not supported (only shared memory and loopback devices support it in any event).
	///
	/// Only supported by the InfiniBand variants for reliably and dynamically connected.
	///
	/// May return an error of `NoResourcesAreAvailableToInitiateTheOperation`, in which case the operation can be tried again (this can be discovered in the completion handler currently, which is maybe not ideal).
	#[inline(always)]
	pub fn send_a_rendezvous_zero_copy_tagged_message<'completion, C: CompletionHandler>(&'remote_end_point self, header: &[u8], from_local_memory: &[ZeroCopyIoVector], completion: &'completion Completion<C>) -> Result<RendezvousCancellation<'remote_end_point, 'completion, C>, ()>
	{
		debug_assert!(self.supports_rendezvous_zero_copy, "Interface does not support zero copy ('zcopy') rendezvous tagged messages");
		debug_assert!(header.len() <= Self::UpperLimit, "header length '{}' exceeds upper limit '{}'", header.len(), Self::UpperLimit);
		debug_assert!(header.len() <= self.maximum_rendezvous_header_length, "header length '{}' exceeds maximum for interface '{}'", header.len(), self.maximum_rendezvous_header_length);
		debug_assert!(self.is_inclusively_within_number_of_items_and_maximum_and_minimum_size_for_rendezvous(from_local_memory), "from_local_memory has too many items, or is too large in total length");
		
		let status_pointer = unsafe { (self.transport_interface_operations().ep_tag_rndv_zcopy)(self.ep(), self.tag_value.0, header.as_ptr() as *const c_void, header.len() as u32, from_local_memory.as_ptr() as * const uct_iov_t, from_local_memory.len(), Self::SignalledMessagesAreNotSupportedFlags.0, completion.to_raw_pointer()) };
		
		let status_pointer_isize = status_pointer as isize;
		
		if status_pointer_isize >= 0
		{
			let cancellation_handle = status_pointer;
			Ok
			(
				RendezvousCancellation
				{
					cancellation_handle,
					remote_end_point: self,
					completion,
				}
			)
		}
		else
		{
			use self::Status::*;
			
			debug_assert!(status_pointer_isize >= Self::SizeOrStatusLowerLimit, "status_pointer is out-of-range for a status");
			let status: ucs_status_t = unsafe { transmute(status_pointer as i8) };
			
			match status.parse()
			{
				Error(error_code) =>
				{
					completion.completed_with_error(error_code);
					Err(())
				}
				
				unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
			}
		}
	}
}
