// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Logic separation for sending active messages with a particular identifier.
///
/// Dereferences to `RemoteEndPoint` to provide access to `flush()`, `fence()`, `progress_thread_unsafe()`, etc.
#[derive(Debug, Clone)]
pub struct ActiveMessageSendingRemoteEndPoint<'remote_end_point>
{
	remote_end_point: &'remote_end_point RemoteEndPoint,
	active_message_identifier: ActiveMessageIdentifier,
	
	supports_short: bool,
	maximum_short_length: usize,
	
	supports_buffered_copy: bool,
	maximum_buffered_copy_length: usize,
	
	supports_zero_copy: bool,
	maximum_zero_copy_length: usize,
	minimum_zero_copy_length: usize,
	maximum_zero_copy_from_local_memory_items: usize,
	optimal_alignment_for_from_local_memory_for_zero_copy: usize,
	maximum_zero_copy_header_length: usize,
}

impl<'remote_end_point> Deref for ActiveMessageSendingRemoteEndPoint<'remote_end_point>
{
	type Target = RemoteEndPoint;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.remote_end_point
	}
}

impl<'remote_end_point> ActiveMessageSendingRemoteEndPoint<'remote_end_point>
{
	const UpperLimit: usize = ::std::u32::MAX as usize;
	
	const SizeOrStatusLowerLimit: isize = ::std::i8::MIN as isize;
	
	const SignalledMessagesAreNotSupportedFlags: uct_msg_flags = uct_msg_flags(0);
	
	#[inline(always)]
	fn new(remote_end_point: &'remote_end_point RemoteEndPoint, active_message_identifier: ActiveMessageIdentifier, attributes: &CommunicationInterfaceContextAttributes) -> Self
	{
		let active_message_constraints = attributes.active_message_constraints();
		
		Self
		{
			remote_end_point,
			active_message_identifier,
			
			supports_short: attributes.supports_all_of(InterfaceFeaturesSupported::AM_SHORT),
			maximum_short_length: active_message_constraints.max_short,
			
			supports_buffered_copy: attributes.supports_all_of(InterfaceFeaturesSupported::AM_BCOPY),
			maximum_buffered_copy_length: active_message_constraints.max_bcopy,
			
			supports_zero_copy: attributes.supports_all_of(InterfaceFeaturesSupported::AM_ZCOPY),
			maximum_zero_copy_length: active_message_constraints.max_zcopy,
			minimum_zero_copy_length: active_message_constraints.min_zcopy,
			maximum_zero_copy_from_local_memory_items: active_message_constraints.max_iov,
			optimal_alignment_for_from_local_memory_for_zero_copy: active_message_constraints.opt_zcopy_align,
			maximum_zero_copy_header_length: active_message_constraints.max_hdr,
		}
	}
	
	/// Maximum.
	#[inline(always)]
	pub fn maximum_zero_copy_from_local_memory_items(&self) -> usize
	{
		self.maximum_zero_copy_from_local_memory_items
	}
	
	/// Maximum.
	#[inline(always)]
	pub fn maximum_zero_copy_header_length(&self) -> usize
	{
		self.optimal_alignment_for_from_local_memory_for_zero_copy
	}
	
	/// Should be a power-of-two, but this is not enforced.
	#[inline(always)]
	pub fn optimal_alignment_for_from_local_memory_for_zero_copy(&self) -> usize
	{
		self.optimal_alignment_for_from_local_memory_for_zero_copy
	}
	
	/// Is within limits.
	#[inline(always)]
	pub fn is_inclusively_within_number_of_items_and_maximum_and_minimum_size(&self, from_local_memory: &[ZeroCopyIoVector]) -> bool
	{
		let number_of_items = from_local_memory.len();
		if number_of_items == 0 || number_of_items > self.maximum_zero_copy_from_local_memory_items
		{
			return false;
		}
		
		let total_length = ZeroCopyIoVector::total_length(from_local_memory);
		total_length >= self.minimum_zero_copy_length && total_length <= self.maximum_zero_copy_length
	}
	
	/// Send an immediate ('short') active message.
	///
	/// Equivalent to `uct_ep_am_short`.
	#[inline(always)]
	pub fn send_an_immediate_active_message(&self, header: ActiveMessageImmediateHeader, from_local_memory: &[u8]) -> Result<(), ErrorCode>
	{
		debug_assert!(self.supports_short, "Interface does not support immediate ('short') active messages");
		debug_assert!(from_local_memory.len() <= Self::UpperLimit, "from_local_memory length '{}' exceeds upper limit '{}'", from_local_memory.len(), Self::UpperLimit);
		debug_assert!(size_of::<ActiveMessageImmediateHeader>() + from_local_memory.len() <= self.maximum_short_length, "header length '{}' + from_local_memory length '{}' exceeds maximum for interface '{}'", size_of::<ActiveMessageImmediateHeader>(), from_local_memory.len(), self.maximum_short_length);
		
		let status = unsafe { (self.transport_interface_operations().ep_am_short)(self.ep(), self.active_message_identifier.0, header, from_local_memory.as_ptr() as *const c_void, from_local_memory.len() as u32) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(()),
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
	
	/// Send a buffered copy ('bcopy') active message.
	///
	/// Equivalent to `uct_ep_am_bcopy`.
	///
	/// bcopy == buffered copy-and-send.
	///
	/// Returns serialized (packed) length.
	///
	/// Sending signalled messages (using uct_msg_flags) is not supported (only shared memory and loopback devices support it in any event).
	///
	/// * `from_local_memory`: Will be specified a buffer that, including header, must not exceed `maximum_buffered_copy_length`.
	#[inline(always)]
	pub fn send_a_buffered_copy_active_message<BCS: BufferedCopySerializer>(&self, from_local_memory: Box<BCS>) -> Result<usize, ErrorCode>
	{
		debug_assert!(self.supports_buffered_copy, "Interface does not support buffered copy ('bcopy') active messages");
		
		let size_or_status = unsafe { (self.transport_interface_operations().ep_am_bcopy)(self.ep(), self.active_message_identifier.0, buffered_copy_serialize_callback::<BCS>, Box::into_raw(from_local_memory) as *mut c_void, Self::SignalledMessagesAreNotSupportedFlags.0) };
		
		if size_or_status >= 0
		{
			let size = size_or_status as usize;
			debug_assert!(size <= self.maximum_buffered_copy_length, "size '{}' exceeds maximum for interface '{}'", size, self.maximum_buffered_copy_length);
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
	
	/// Send an active message while avoiding local memory copy, ie by 'zero copy'.
	///
	/// Equivalent to `uct_ep_am_zcopy`.
	///
	/// `header` and `from_local_memory` memory must have been registered with, or allocated by, the memory domain for this communication interface for this remote end point.
	///
	/// If not complete (likely), then this method then needs `progress()` to be called on the ProgressEngine.
	///
	/// * `header` may be no longer than `self.maximum_zero_copy_header_length()`.
	/// * `from_local_memory` maximum length is `self.maximum_zero_copy_from_local_memory_items()`. `UCT_IB_MAX_IOV` is 8, and one IoVector seems to be used internally for the header, implying 7 is the normal maximum.
	///
	/// Note that `from_local_memory` buffers should ideally be aligned to `self.optimal_alignment_for_from_local_memory_for_zero_copy()`.
	/// This is usually one of:-
	///
	/// * `UCS_SYS_PCI_MAX_PAYLOAD` (for InfiniBand-like devices; 512 bytes)
	/// * `UCS_SYS_CACHE_LINE_SIZE` (for Shared Memory devices)
	/// * `1`
	///
	/// Sending signalled messages (using uct_msg_flags) is not supported (only shared memory and loopback devices support it in any event).
	#[inline(always)]
	pub fn send_a_zero_copy_active_message<C: CompletionHandler>(&self, header: &[u8], from_local_memory: &[ZeroCopyIoVector], completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		debug_assert!(self.supports_zero_copy, "Interface does not support zero copy ('zcopy') active messages");
		debug_assert!(header.len() <= Self::UpperLimit, "header length '{}' exceeds upper limit '{}'", header.len(), Self::UpperLimit);
		debug_assert!(header.len() <= self.maximum_zero_copy_header_length, "header length '{}' exceeds maximum for interface '{}'", header.len(), self.maximum_zero_copy_header_length);
		debug_assert!(self.is_inclusively_within_number_of_items_and_maximum_and_minimum_size(from_local_memory), "from_local_memory has too few or too many items, or is too small or too large in total length");
		
		let status = unsafe { (self.transport_interface_operations().ep_am_zcopy)(self.ep(), self.active_message_identifier.0, header.as_ptr() as *const c_void, header.len() as u32, from_local_memory.as_ptr() as * const uct_iov_t, from_local_memory.len(), Self::SignalledMessagesAreNotSupportedFlags.0, completion.to_raw_pointer()) };
		
		completion.parse_status(status)
	}
	
	#[inline(always)]
	fn transport_interface_operations(&self) -> &mut uct_iface_ops
	{
		self.remote_end_point.transport_interface_operations()
	}
	
	#[inline(always)]
	fn ep(&self) -> *mut uct_ep
	{
		self.remote_end_point.ep()
	}
}
