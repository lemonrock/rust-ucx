// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Logic separation for sending active messages with a particular identifier.
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
	maximum_zero_copy_header_length: usize,
	maximum_zero_copy_io_vec_items: usize,
	optimal_alignment_for_io_vec_buffers_for_zero_copy: usize,
}

impl<'remote_end_point> ActiveMessageSendingRemoteEndPoint<'remote_end_point>
{
	const UpperLimit: usize = ::std::u32::MAX as usize;
	
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
			maximum_zero_copy_header_length: active_message_constraints.max_hdr,
			maximum_zero_copy_io_vec_items: active_message_constraints.max_iov,
			optimal_alignment_for_io_vec_buffers_for_zero_copy: active_message_constraints.opt_zcopy_align,
		}
	}
	
	/// Maximum.
	#[inline(always)]
	pub fn maximum_zero_copy_io_vec_items(&self) -> usize
	{
		self.maximum_zero_copy_io_vec_items
	}
	
	/// Maximum.
	#[inline(always)]
	pub fn maximum_zero_copy_header_length(&self) -> usize
	{
		self.optimal_alignment_for_io_vec_buffers_for_zero_copy
	}
	
	/// Should be a power-of-two, but this is not enforced.
	#[inline(always)]
	pub fn optimal_alignment_for_io_vec_buffers_for_zero_copy(&self) -> usize
	{
		self.optimal_alignment_for_io_vec_buffers_for_zero_copy
	}
	
	/// Is within limits.
	#[inline(always)]
	pub fn is_inclusively_within_number_of_items_and_maximum_and_minimum_size_and(&self, zero_copy_io_vectors: &[ZeroCopyIoVector]) -> bool
	{
		let number_of_items = zero_copy_io_vectors.len();
		if number_of_items == 0 || number_of_items > self.maximum_zero_copy_io_vec_items
		{
			return false;
		}
		
		let total_length = ZeroCopyIoVector::total_length(zero_copy_io_vectors);
		total_length >= self.minimum_zero_copy_length && total_length <= self.maximum_zero_copy_length
	}
	
	/// Send an immediate ('short') active message.
	///
	/// Equivalent to `uct_ep_am_short`.
	#[inline(always)]
	pub fn send_an_immediate_active_message(&self, header: ActiveMessageImmediateHeader, payload: &[u8]) -> Result<(), ErrorCode>
	{
		debug_assert!(self.supports_short, "Does not support immediate (short) active messages");
		debug_assert!(payload.len() <= Self::UpperLimit, "payload length '{}' exceeds upper limit '{}'", payload.len(), Self::UpperLimit);
		debug_assert!(size_of::<ActiveMessageImmediateHeader>() + payload.len() <= self.maximum_short_length, "header length '{}' + payload length '{}' exceeds maximum for interface '{}'", size_of::<ActiveMessageImmediateHeader>(), payload.len(), self.maximum_short_length);
		
		let status = unsafe { (self.transport_interface_operations().ep_am_short)(self.ep(), self.active_message_identifier.0, header, payload.as_ptr() as *const c_void, payload.len() as u32) };
		
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
	/// bcopy == buffered copy-and-send
	///
	/// Sending signalled messages (using uct_msg_flags) is not supported (only shared memory and loopback devices support it in any event).
	///
	/// * `buffered_copy_serializer`: Will be specified a buffer that, including header, must not exceed `maximum_buffered_copy_length`.
	#[inline(always)]
	pub fn send_a_buffered_copy_active_message<BCS: BufferedCopySerializer>(&self, buffered_copy_serializer: Box<BCS>) -> Result<usize, ErrorCode>
	{
		debug_assert!(self.supports_buffered_copy, "Does not support buffered copy (bcopy) active messages");
		
		unsafe extern "C" fn send_a_buffered_copy_active_message_callback<BCS: BufferedCopySerializer>(dest: *mut c_void, arg: *mut c_void) -> usize
		{
			debug_assert!(!dest.is_null(), "dest is null");
			debug_assert!(!arg.is_null(), "arg is null");
			
			let buffered_copy_serializer = Box::from_raw(arg as *mut BCS);
			buffered_copy_serializer.serialize(NonNull::new_unchecked(dest as *mut u8))
		}
		
		let size_or_status = unsafe { (self.transport_interface_operations().ep_am_bcopy)(self.ep(), self.active_message_identifier.0, send_a_buffered_copy_active_message_callback::<BCS>, Box::into_raw(buffered_copy_serializer) as *mut c_void, Self::SignalledMessagesAreNotSupportedFlags.0) };
		
		if size_or_status >= 0
		{
			let size = size_or_status as usize;
			debug_assert!(size <= self.maximum_buffered_copy_length, "size '{}' exceeds maximum_buffered_copy_length '{}'", size, self.maximum_buffered_copy_length);
			Ok(size)
		}
		else
		{
			use self::Status::*;
			
			debug_assert!(size_or_status >= ::std::i8::MIN as isize, "size_or_status is out-of-range");
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
	/// `header` and `zero_copy_io_vectors` memory must have been registered with, or allocated by, the memory domain for this communication interface for this remote end point.
	///
	///
	///
	///
	/// ***The lifetime requirements of `header` and `zero_copy_io_vectors` is not known.***
	///
	/// If not complete (likely), then this method then needs `progress()` to be called on the ProgressEngine.
	///
	///
	///
	/// * `header` may be no longer than `self.maximum_zero_copy_header_length()`.
	/// * `zero_copy_io_vectors` maximum length is `self.maximum_zero_copy_io_vec_items()`. `UCT_IB_MAX_IOV` is 8, and one IoVector seems to be used internally for the header, implying 7 is the normal maximum.
	///
	/// Sending signalled messages (using uct_msg_flags) is not supported (only shared memory and loopback devices support it in any event).
	///
	/// Note that `zero_copy_io_vectors` buffers should ideally be aligned to `self.optimal_alignment_for_io_vec_buffers_for_zero_copy()`.
	/// This is usually one of:-
	///
	/// * `UCS_SYS_PCI_MAX_PAYLOAD` (for InfiniBand-like devices; 512 bytes)
	/// * `UCS_SYS_CACHE_LINE_SIZE` (for Shared Memory devices)
	/// * `1`
	#[inline(always)]
	pub fn send_a_zero_copy_active_message<C: CompletionHandler>(&self, header: &[u8], zero_copy_io_vectors: &[ZeroCopyIoVector], completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		debug_assert!(self.supports_zero_copy, "Does not support zero copy (zcopy) active messages");
		debug_assert!(header.len() <= Self::UpperLimit, "header length '{}' exceeds upper limit '{}'", header.len(), Self::UpperLimit);
		debug_assert!(header.len() <= self.maximum_zero_copy_header_length, "header length '{}' exceeds maximum for interface '{}'", header.len(), self.maximum_zero_copy_header_length);
		debug_assert!(self.is_inclusively_within_number_of_items_and_maximum_and_minimum_size_and(zero_copy_io_vectors), "zero_copy_io_vectors has too few or too many items, or is too small or too large in total length");
		
		let status = unsafe { (self.transport_interface_operations().ep_am_zcopy)(self.ep(), self.active_message_identifier.0, header.as_ptr() as *const c_void, header.len() as u32, zero_copy_io_vectors.as_ptr() as * const uct_iov_t, zero_copy_io_vectors.len(), Self::SignalledMessagesAreNotSupportedFlags.0, completion.to_raw_pointer()) };
		
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
