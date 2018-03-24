// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A convenience to work with remote memory.
///
/// Dereferences to `RemoteEndPoint` to provide access to `flush()`, `fence()`, `progress_thread_unsafe()`, etc.
///
/// Note that InfiniBand memory domains do not require holding onto the UnpackedMemoryKey, but nearly all other kinds do, as they pack a heap-allocated pointed into the `uct_rkey_t` type of `remote_key_descriptor`.
pub struct RemoteMemoryAccessRemoteEndPoint<'remote_end_point>
{
	remote_end_point: &'remote_end_point RemoteEndPoint,
	remote_key_descriptor: uct_rkey_t,
	_unpacked_memory_key_handle_drop_safety: &'remote_end_point UnpackedMemoryKey,
	
	supports_put_short: bool,
	maximum_put_short_length: usize,
	
	supports_put_buffered_copy: bool,
	maximum_put_buffered_copy_length: usize,
	
	supports_put_zero_copy: bool,
	maximum_put_zero_copy_length: usize,
	minimum_put_zero_copy_length: usize,
	maximum_put_zero_copy_from_local_memory_items: usize,
	optimal_alignment_for_from_local_memory_for_put_zero_copy: usize,
	
	supports_get_short: bool,
	maximum_get_short_length: usize,
	
	supports_get_buffered_copy: bool,
	maximum_get_buffered_copy_length: usize,
	
	supports_get_zero_copy: bool,
	maximum_get_zero_copy_length: usize,
	minimum_get_zero_copy_length: usize,
	maximum_get_zero_copy_to_local_memory_items: usize,
	optimal_alignment_for_to_local_memory_for_get_for_get_zero_copy: usize,
	
	supports_atomic_u32_add: bool,
	supports_atomic_u32_fetch_and_add: bool,
	supports_atomic_u32_swap: bool,
	supports_atomic_u32_compare_and_swap: bool,
	
	supports_atomic_u64_add: bool,
	supports_atomic_u64_fetch_and_add: bool,
	supports_atomic_u64_swap: bool,
	supports_atomic_u64_compare_and_swap: bool,
}

impl<'remote_end_point> Deref for RemoteMemoryAccessRemoteEndPoint<'remote_end_point>
{
	type Target = RemoteEndPoint;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.remote_end_point
	}
}

impl<'remote_end_point> RemoteMemoryAccessRemoteEndPoint<'remote_end_point>
{
	const UpperLimit: usize = ::std::u32::MAX as usize;
	
	const SizeOrStatusLowerLimit: isize = ::std::i8::MIN as isize;
	
	#[inline(always)]
	fn new(remote_end_point: &'remote_end_point RemoteEndPoint, unpacked_memory_key: &'remote_end_point UnpackedMemoryKey, attributes: &CommunicationInterfaceContextAttributes) -> Self
	{
		let put_constraints = attributes.put_constraints();
		let get_constraints = attributes.get_constraints();
		
		Self
		{
			remote_end_point,
			remote_key_descriptor: unpacked_memory_key.remote_key_descriptor(),
			_unpacked_memory_key_handle_drop_safety: unpacked_memory_key,
			
			supports_put_short: attributes.supports_all_of(InterfaceFeaturesSupported::PUT_SHORT),
			maximum_put_short_length: put_constraints.max_short,
			
			supports_put_buffered_copy: attributes.supports_all_of(InterfaceFeaturesSupported::PUT_BCOPY),
			maximum_put_buffered_copy_length: put_constraints.max_bcopy,
			
			supports_put_zero_copy: attributes.supports_all_of(InterfaceFeaturesSupported::PUT_ZCOPY),
			maximum_put_zero_copy_length: put_constraints.max_zcopy,
			minimum_put_zero_copy_length: put_constraints.min_zcopy,
			maximum_put_zero_copy_from_local_memory_items: put_constraints.max_iov,
			optimal_alignment_for_from_local_memory_for_put_zero_copy: put_constraints.opt_zcopy_align,
			
			supports_get_short: attributes.supports_all_of(InterfaceFeaturesSupported::GET_SHORT),
			maximum_get_short_length: get_constraints.max_short,
			
			supports_get_buffered_copy: attributes.supports_all_of(InterfaceFeaturesSupported::GET_BCOPY),
			maximum_get_buffered_copy_length: get_constraints.max_bcopy,
			
			supports_get_zero_copy: attributes.supports_all_of(InterfaceFeaturesSupported::GET_ZCOPY),
			maximum_get_zero_copy_length: get_constraints.max_zcopy,
			minimum_get_zero_copy_length: get_constraints.min_zcopy,
			maximum_get_zero_copy_to_local_memory_items: get_constraints.max_iov,
			optimal_alignment_for_to_local_memory_for_get_for_get_zero_copy: get_constraints.opt_zcopy_align,
			
			supports_atomic_u32_add: attributes.supports_all_of(InterfaceFeaturesSupported::ATOMIC_ADD32),
			supports_atomic_u32_fetch_and_add: attributes.supports_all_of(InterfaceFeaturesSupported::ATOMIC_FADD32),
			supports_atomic_u32_swap: attributes.supports_all_of(InterfaceFeaturesSupported::ATOMIC_SWAP32),
			supports_atomic_u32_compare_and_swap: attributes.supports_all_of(InterfaceFeaturesSupported::ATOMIC_CSWAP32),
			
			supports_atomic_u64_add: attributes.supports_all_of(InterfaceFeaturesSupported::ATOMIC_ADD64),
			supports_atomic_u64_fetch_and_add: attributes.supports_all_of(InterfaceFeaturesSupported::ATOMIC_FADD64),
			supports_atomic_u64_swap: attributes.supports_all_of(InterfaceFeaturesSupported::ATOMIC_SWAP64),
			supports_atomic_u64_compare_and_swap: attributes.supports_all_of(InterfaceFeaturesSupported::ATOMIC_CSWAP64),
		}
	}
	
	/// Maximum.
	#[inline(always)]
	pub fn maximum_put_zero_copy_from_local_memory_items(&self) -> usize
	{
		self.maximum_put_zero_copy_from_local_memory_items
	}
	
	/// Maximum.
	#[inline(always)]
	pub fn maximum_put_zero_copy_header_length(&self) -> usize
	{
		self.optimal_alignment_for_from_local_memory_for_put_zero_copy
	}
	
	/// Should be a power-of-two, but this is not enforced.
	#[inline(always)]
	pub fn optimal_alignment_for_from_local_memory_for_put_zero_copy(&self) -> usize
	{
		self.optimal_alignment_for_from_local_memory_for_put_zero_copy
	}
	
	/// Is within limits.
	#[inline(always)]
	pub fn is_inclusively_within_number_of_items_and_maximum_and_minimum_size_for_put(&self, from_local_memory: &[ZeroCopyIoVector]) -> bool
	{
		let number_of_items = from_local_memory.len();
		if number_of_items == 0 || number_of_items > self.maximum_put_zero_copy_from_local_memory_items
		{
			return false;
		}
		
		let total_length = ZeroCopyIoVector::total_length(from_local_memory);
		total_length >= self.minimum_put_zero_copy_length && total_length <= self.maximum_put_zero_copy_length
	}
	
	/// Maximum.
	#[inline(always)]
	pub fn maximum_get_zero_copy_to_local_memory_items(&self) -> usize
	{
		self.maximum_get_zero_copy_to_local_memory_items
	}
	
	/// Maximum.
	#[inline(always)]
	pub fn maximum_get_zero_copy_header_length(&self) -> usize
	{
		self.optimal_alignment_for_to_local_memory_for_get_for_get_zero_copy
	}
	
	/// Should be a power-of-two, but this is not enforced.
	#[inline(always)]
	pub fn optimal_alignment_for_to_local_memory_for_get_zero_copy(&self) -> usize
	{
		self.optimal_alignment_for_to_local_memory_for_get_for_get_zero_copy
	}
	
	/// Is within limits.
	#[inline(always)]
	pub fn is_inclusively_within_number_of_items_and_maximum_and_minimum_size_for_get(&self, to_local_memory: &[ZeroCopyIoVector]) -> bool
	{
		let number_of_items = to_local_memory.len();
		if number_of_items == 0 || number_of_items > self.maximum_get_zero_copy_to_local_memory_items
		{
			return false;
		}
		
		let total_length = ZeroCopyIoVector::total_length(to_local_memory);
		total_length >= self.minimum_get_zero_copy_length && total_length <= self.maximum_get_zero_copy_length
	}
	
	/// Puts (writes, stores) a value from `from` `to_remote_memory_address` immediately.
	///
	/// The value must be quite small (in bytes).
	///
	/// Equivalent to `uct_ep_put_short`.
	#[inline(always)]
	pub fn put_immediately(&self, from_local_memory: &[u8], to_remote_memory_address: RemoteMemoryAddress) -> Result<(), ErrorCode>
	{
		debug_assert!(self.supports_put_short, "Interface does not support put immediately ('put short')");
		debug_assert!(from_local_memory.len() <= Self::UpperLimit, "from_local_memory length '{}' exceeds upper limit '{}'", from_local_memory.len(), Self::UpperLimit);
		debug_assert!(from_local_memory.len() <= self.maximum_put_short_length, "from_local_memory length '{}' exceeds maximum for interface '{}'", from_local_memory.len(), self.maximum_put_short_length);
		
		let status = unsafe { (self.transport_interface_operations().ep_put_short)(self.ep(), from_local_memory.as_ptr() as *const c_void, from_local_memory.len() as u32, to_remote_memory_address.0, self.remote_key_descriptor) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(()),
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
	
	/// Puts (writes, stores) a value `from_local_memory` using a buffered copy `to_remote_memory_address`.
	///
	/// Equivalent to `uct_ep_put_bcopy`.
	///
	/// bcopy == buffered copy-and-send.
	///
	/// Returns serialized (packed) length.
	#[inline(always)]
	pub fn put_using_a_buffered_copy<BCS: BufferedCopySerializer>(&self, from_local_memory: Box<BCS>, to_remote_memory_address: RemoteMemoryAddress) -> Result<usize, ErrorCode>
	{
		debug_assert!(self.supports_put_buffered_copy, "Interface does not support put using buffered copy ('put bcopy')");
		
		let size_or_status = unsafe { (self.transport_interface_operations().ep_put_bcopy)(self.ep(), buffered_copy_serialize_callback::<BCS>, Box::into_raw(from_local_memory) as *mut c_void, to_remote_memory_address.0, self.remote_key_descriptor) };
		
		if size_or_status >= 0
		{
			let size = size_or_status as usize;
			debug_assert!(size <= self.maximum_put_buffered_copy_length, "size '{}' exceeds maximum for interface '{}'", size, self.maximum_put_buffered_copy_length);
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
	
	/// Puts (writes, stores) a value `from_local_memory` using a zero-copy `to_remote_memory_address`.
	///
	/// Equivalent to `uct_ep_put_zcopy`.
	///
	/// `from_local_memory` memory must have been registered with, or allocated by, the memory domain for this communication interface for this remote end point.
	///
	/// If not complete (likely), then this method then needs `progress()` to be called on the ProgressEngine.
	///
	/// * `header` may be no longer than `self.maximum_zero_copy_header_length()`.
	/// * `from_local_memory` maximum length is `self.maximum_zero_copy_io_vec_items()`. `UCT_IB_MAX_IOV` is 8.
	///
	/// Note that `from_local_memory` buffers should ideally be aligned to `self.optimal_alignment_for_to_local_memory_for_get_for_zero_copy()`.
	/// This is usually one of:-
	///
	/// * `UCS_SYS_PCI_MAX_PAYLOAD` (for InfiniBand-like devices; 512 bytes)
	/// * `UCS_SYS_CACHE_LINE_SIZE` (for Shared Memory devices)
	/// * `1`
	#[inline(always)]
	pub fn put_using_a_zero_copy<C: CompletionHandler>(&self, from_local_memory: &[ZeroCopyIoVector], to_remote_memory_address: RemoteMemoryAddress, completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		debug_assert!(self.supports_put_zero_copy, "Interface does not support put using zero copy ('put zcopy')");
		debug_assert!(self.is_inclusively_within_number_of_items_and_maximum_and_minimum_size_for_put(from_local_memory), "from_local_memory has too few or too many items, or is too small or too large in total length");
		
		let status = unsafe { (self.transport_interface_operations().ep_put_zcopy)(self.ep(), from_local_memory.as_ptr() as * const uct_iov_t, from_local_memory.len(), to_remote_memory_address.0, self.remote_key_descriptor, completion.to_raw_pointer()) };
		
		completion.parse_status(status)
	}
	
	/// Gets (reads, loads) a value `from_remote_memory_address` `to_local_memory` immediately.
	///
	/// The value must be quite small (in bytes).
	///
	/// Equivalent to `uct_ep_get_short`.
	#[inline(always)]
	pub fn get_immediately(&self, to_remote_memory_address: RemoteMemoryAddress, to_local_memory: &mut [u8]) -> Result<(), ErrorCode>
	{
		debug_assert!(self.supports_get_short, "Interface does not support get immediately ('get short')");
		debug_assert!(to_local_memory.len() <= Self::UpperLimit, "from_local_memory length '{}' exceeds upper limit '{}'", to_local_memory.len(), Self::UpperLimit);
		debug_assert!(to_local_memory.len() <= self.maximum_get_short_length, "from_local_memory length '{}' exceeds maximum for interface '{}'", to_local_memory.len(), self.maximum_get_short_length);
		
		let status = unsafe { (self.transport_interface_operations().ep_get_short)(self.ep(), to_local_memory.as_ptr() as *const c_void, to_local_memory.len() as u32, to_remote_memory_address.0, self.remote_key_descriptor) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(()),
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
	
	/// Gets (reads, loads) a value `from_remote_memory_address` using a buffered copy `to_local_memory`.
	///
	/// Equivalent to `uct_ep_get_bcopy`.
	///
	/// bcopy == buffered copy-and-send.
	///
	/// Returns serialized (packed) length.
	#[inline(always)]
	pub fn get_using_a_buffered_copy<BCD: BufferedCopyDeserializer, C: CompletionHandler>(&self, from_remote_memory_address: RemoteMemoryAddress, to_local_memory: Box<BCD>, completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		debug_assert!(self.supports_get_buffered_copy, "Interface does not support get using buffered copy ('get bcopy')");
		
		let length = to_local_memory.length();
		debug_assert_ne!(length, 0, "to_local_memory.length() can not be zero");
		debug_assert!(length <= self.maximum_get_buffered_copy_length, "to_local_memory.length() '{}' exceeds maximum for interface '{}'", length, self.maximum_get_buffered_copy_length);
		
		let status = unsafe { (self.transport_interface_operations().ep_get_bcopy)(self.ep(), buffered_copy_deserialize_callback::<BCD>, Box::into_raw(to_local_memory) as *mut c_void, length, from_remote_memory_address.0, self.remote_key_descriptor, completion.to_raw_pointer()) };
		
		completion.parse_status(status)
	}
	
	/// Gets (reads, loads) a value `from_remote_memory_address` using a zero-copy `to_local_memory`.
	///
	/// Equivalent to `uct_ep_get_zcopy`.
	///
	/// `to_local_memory` memory must have been registered with, or allocated by, the memory domain for this communication interface for this remote end point.
	///
	/// If not complete (likely), then this method then needs `progress()` to be called on the ProgressEngine.
	///
	/// * `header` may be no longer than `self.maximum_zero_copy_header_length()`.
	/// * `to_local_memory` maximum length is `self.maximum_zero_copy_io_vec_items()`. `UCT_IB_MAX_IOV` is 8.
	///
	/// Note that `to_local_memory` buffers should ideally be aligned to `self.optimal_alignment_for_to_local_memory_for_get_for_zero_copy()`.
	/// This is usually one of:-
	///
	/// * `UCS_SYS_PCI_MAX_PAYLOAD` (for InfiniBand-like devices; 512 bytes)
	/// * `UCS_SYS_CACHE_LINE_SIZE` (for Shared Memory devices)
	/// * `1`
	#[inline(always)]
	pub fn get_using_a_zero_copy<C: CompletionHandler>(&self, from_remote_memory_address: RemoteMemoryAddress, to_local_memory: &[ZeroCopyIoVector], completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		debug_assert!(self.supports_get_zero_copy, "Interface does not support get using zero copy ('get zcopy')");
		debug_assert!(self.is_inclusively_within_number_of_items_and_maximum_and_minimum_size_for_get(to_local_memory), "to_local_memory has too few or too many items, or is too small or too large in total length");
		
		let status = unsafe { (self.transport_interface_operations().ep_get_zcopy)(self.ep(), to_local_memory.as_ptr() as * const uct_iov_t, to_local_memory.len(), from_remote_memory_address.0, self.remote_key_descriptor, completion.to_raw_pointer()) };
		
		completion.parse_status(status)
	}
	
	/// Atomic add of `increment` `at_remote_memory_address` for u32.
	///
	/// Equivalent to `uct_ep_atomic_add32`.
	///
	/// `at_remote_memory_address` must be 32-bit aligned.
	#[inline(always)]
	pub fn atomic_add_u32(&self, increment: u32, at_remote_memory_address: RemoteMemoryAddress) -> Result<(), ErrorCode>
	{
		debug_assert!(self.supports_atomic_u32_add, "Interface does not support atomic add for u32");
		at_remote_memory_address.debug_assert_is_32_bit_aligned();
		
		let status = unsafe { (self.transport_interface_operations().ep_atomic_add32)(self.ep(), increment, at_remote_memory_address.0, self.remote_key_descriptor) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(()),
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
	
	/// Atomic add of `increment` `at_remote_memory_address` for u64.
	///
	/// Equivalent to `uct_ep_atomic_add64`.
	///
	/// `at_remote_memory_address` must be 64-bit aligned.
	#[inline(always)]
	pub fn atomic_add_u64(&self, increment: u64, at_remote_memory_address: RemoteMemoryAddress) -> Result<(), ErrorCode>
	{
		debug_assert!(self.supports_atomic_u64_add, "Interface does not support atomic add for u64");
		at_remote_memory_address.debug_assert_is_64_bit_aligned();
		
		let status = unsafe { (self.transport_interface_operations().ep_atomic_add64)(self.ep(), increment, at_remote_memory_address.0, self.remote_key_descriptor) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(()),
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
	
	/// Atomic fetch-and-add of `increment` `at_remote_memory_address` for u32 returning the previous value `previous_value`.
	///
	/// Equivalent to `uct_ep_atomic_fadd32`.
	///
	/// * `at_remote_memory_address` must be 32-bit aligned.
	/// * `previous_value` does not need to be initialized, but must be valid until the completion handler is called.
	#[inline(always)]
	pub fn atomic_fetch_and_add_u32<C: CompletionHandler>(&self, increment: u32, at_remote_memory_address: RemoteMemoryAddress, previous_value: &mut u32, completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		debug_assert!(self.supports_atomic_u32_fetch_and_add, "Interface does not support atomic fetch-and-add for u32");
		at_remote_memory_address.debug_assert_is_32_bit_aligned();
		
		let status = unsafe { (self.transport_interface_operations().ep_atomic_fadd32)(self.ep(), increment, at_remote_memory_address.0, self.remote_key_descriptor, previous_value, completion.to_raw_pointer()) };
		
		completion.parse_status(status)
	}
	
	/// Atomic fetch-and-add of `increment` `at_remote_memory_address` for u64 returning the previous value `previous_value`.
	///
	/// Equivalent to `uct_ep_atomic_fadd64`.
	///
	/// * `at_remote_memory_address` must be 64-bit aligned.
	/// * `previous_value` does not need to be initialized, but must be valid until the completion handler is called.
	#[inline(always)]
	pub fn atomic_fetch_and_add_u64<C: CompletionHandler>(&self, increment: u64, at_remote_memory_address: RemoteMemoryAddress, previous_value: &mut u64, completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		debug_assert!(self.supports_atomic_u64_fetch_and_add, "Interface does not support atomic fetch-and-add for u64");
		at_remote_memory_address.debug_assert_is_64_bit_aligned();
		
		let status = unsafe { (self.transport_interface_operations().ep_atomic_fadd64)(self.ep(), increment, at_remote_memory_address.0, self.remote_key_descriptor, previous_value, completion.to_raw_pointer()) };
		
		completion.parse_status(status)
	}
	
	/// Atomic swap of `replacement` `at_remote_memory_address` for u32 returning the previous value `previous_value`.
	///
	/// Equivalent to `uct_ep_atomic_swap32`.
	///
	/// * `at_remote_memory_address` must be 32-bit aligned.
	/// * `previous_value` does not need to be initialized, but must be valid until the completion handler is called.
	#[inline(always)]
	pub fn atomic_swap_u32<C: CompletionHandler>(&self, replacement: u32, at_remote_memory_address: RemoteMemoryAddress, previous_value: &mut u32, completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		debug_assert!(self.supports_atomic_u32_swap, "Interface does not support atomic swap for u32");
		at_remote_memory_address.debug_assert_is_32_bit_aligned();
		
		let status = unsafe { (self.transport_interface_operations().ep_atomic_swap32)(self.ep(), replacement, at_remote_memory_address.0, self.remote_key_descriptor, previous_value, completion.to_raw_pointer()) };
		
		completion.parse_status(status)
	}
	
	/// Atomic swap of `replacement` `at_remote_memory_address` for u64 returning the previous value `previous_value`.
	///
	/// Equivalent to `uct_ep_atomic_swap64`.
	///
	/// * `at_remote_memory_address` must be 64-bit aligned.
	/// * `previous_value` does not need to be initialized, but must be valid until the completion handler is called.
	#[inline(always)]
	pub fn atomic_swap_u64<C: CompletionHandler>(&self, replacement: u64, at_remote_memory_address: RemoteMemoryAddress, previous_value: &mut u64, completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		debug_assert!(self.supports_atomic_u64_swap, "Interface does not support atomic swap for u64");
		at_remote_memory_address.debug_assert_is_64_bit_aligned();
		
		let status = unsafe { (self.transport_interface_operations().ep_atomic_swap64)(self.ep(), replacement, at_remote_memory_address.0, self.remote_key_descriptor, previous_value, completion.to_raw_pointer()) };
		
		completion.parse_status(status)
	}
	
	/// Atomic compare-and-swap of `replacement` `at_remote_memory_address` for u32 if it matches `was` returning either, if successful, the value of `replacement`, or, if unsuccessful, the actual value `at_remote_memory_address` in `is_now`.
	///
	/// Equivalent to `uct_ep_atomic_cswap32`.
	///
	/// * `at_remote_memory_address` must be 32-bit aligned.
	/// * `is_now` does not need to be initialized, but must be valid until the completion handler is called.
	#[inline(always)]
	pub fn atomic_compare_and_swap_u32<C: CompletionHandler>(&self, was: u32, replacement: u32, at_remote_memory_address: RemoteMemoryAddress, is_now: &mut u32, completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		debug_assert!(self.supports_atomic_u32_compare_and_swap, "Interface does not support atomic compare-and-swap for u32");
		at_remote_memory_address.debug_assert_is_32_bit_aligned();
		
		let status = unsafe { (self.transport_interface_operations().ep_atomic_cswap32)(self.ep(), was, replacement, at_remote_memory_address.0, self.remote_key_descriptor, is_now, completion.to_raw_pointer()) };
		
		completion.parse_status(status)
	}
	
	/// Atomic compare-and-swap of `replacement` `at_remote_memory_address` for u64 if it matches `was` returning either, if successful, the value of `replacement`, or, if unsuccessful, the actual value `at_remote_memory_address` in `is_now`.
	///
	/// Equivalent to `uct_ep_atomic_cswap64`.
	///
	/// * `at_remote_memory_address` must be 64-bit aligned.
	/// * `is_now` does not need to be initialized, but must be valid until the completion handler is called.
	#[inline(always)]
	pub fn atomic_compare_and_swap_u64<C: CompletionHandler>(&self, was: u64, replacement: u64, at_remote_memory_address: RemoteMemoryAddress, is_now: &mut u64, completion: &Completion<C>) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		debug_assert!(self.supports_atomic_u64_compare_and_swap, "Interface does not support atomic compare-and-swap for u64");
		at_remote_memory_address.debug_assert_is_64_bit_aligned();
		
		let status = unsafe { (self.transport_interface_operations().ep_atomic_cswap64)(self.ep(), was, replacement, at_remote_memory_address.0, self.remote_key_descriptor, is_now, completion.to_raw_pointer()) };
		
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
