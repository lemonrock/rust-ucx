// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Their remotely accessible memory.
///
/// Use this to perform remote memory load() and store(), and atomic operations such as fetch_and_add() and compare_and_swap().
///
/// Dereferences to its parent end point, so one can call `non_blocking_flush()`, etc.
#[derive(Debug)]
pub struct TheirRemotelyAccessibleMemory<E: EndPointPeerFailureErrorHandler, A = DirectLocalToRemoteAddressTranslation>
where A: LocalToRemoteAddressTranslation
{
	handle: ucp_rkey_h,
	parent_end_point: Rc<TheirRemotelyAccessibleEndPoint<E, TheirRemotelyAccessibleWorkerEndPointAddress>>,
	local_to_remote_address_translation: A,
}

impl<E: EndPointPeerFailureErrorHandler, A: LocalToRemoteAddressTranslation> Drop for TheirRemotelyAccessibleMemory<E, A>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_rkey_destroy(self.handle) }
	}
}

impl<E: EndPointPeerFailureErrorHandler, A: LocalToRemoteAddressTranslation> Deref for TheirRemotelyAccessibleMemory<E, A>
{
	type Target = Rc<TheirRemotelyAccessibleEndPoint<E, TheirRemotelyAccessibleWorkerEndPointAddress>>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.parent_end_point
	}
}

impl<E: EndPointPeerFailureErrorHandler, A: LocalToRemoteAddressTranslation> TheirRemotelyAccessibleMemory<E, A>
{
	/// Returns a local pointer which can be used for all atomic memory operations.
	///
	/// Will only work for `mmap`, `shmem`, `xpmem`, and `knmem` memory domains, ie memory on the same machine.
	#[inline(always)]
	pub fn local_pointer_if_remote_memory_is_shared_memory(&self, remote_address: *mut u8) -> *mut u8
	{
		let mut local_address = unsafe { uninitialized() };
		panic_on_error!(ucp_rkey_ptr, self.handle, remote_address as u64, &mut local_address);
		local_address as *mut u8
	}
	
	/// Blocking remote store (put) operation.
	///
	/// This routine loads a contiguous block of data of `length` bytes from the remote address and puts into the local address.
	///
	/// Should be thought of as sequentially consistent.
	///
	/// The local memory is safe to use immediately afterwards.
	#[inline(always)]
	pub fn blocking_store(&self, local_source_address: NonNull<u8>, length_in_bytes: usize) -> Result<(), ErrorCode>
	{
		let local_address = local_source_address.as_ptr()  as *mut c_void;
		let remote_address = self.remote_address(local_source_address);
		
		let status = unsafe { ucp_put(self.end_point_handle(), local_address, length_in_bytes, remote_address, self.debug_assert_handle_is_valid()) };
		Self::parse_status_for_blocking(status, ())
	}
	
	/// Non-blocking remote store (put) operation.
	///
	/// This routine loads a contiguous block of data of `length` bytes from the remote address and puts into the local address.
	///
	/// The local memory is ***not*** safe to use immediately afterwards if 'InProgress' is returned; in this case, flush either the end point or the worker.
	#[inline(always)]
	pub fn non_blocking_store(&self, local_source_address: NonNull<u8>, length_in_bytes: usize) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ErrorCode>
	{
		let local_address = local_source_address.as_ptr()  as *mut c_void;
		let remote_address = self.remote_address(local_source_address);
		
		let status = unsafe { ucp_put_nbi(self.end_point_handle(), local_address, length_in_bytes, remote_address, self.debug_assert_handle_is_valid()) };
		Self::parse_status_for_non_blocking(status)
	}
	
	/// Blocking remote load (get) operation.
	///
	/// This routine loads a contiguous block of data of `length` bytes from the remote address and puts into the local address.
	///
	/// Should be thought of as sequentially consistent.
	///
	/// The local memory is safe to use immediately afterwards.
	#[inline(always)]
	pub fn blocking_load(&self, local_destination_address: NonNull<u8>, length_in_bytes: usize) -> Result<(), ErrorCode>
	{
		let local_address = local_destination_address.as_ptr()  as *mut c_void;
		let remote_address = self.remote_address(local_destination_address);
		
		let status = unsafe { ucp_get(self.end_point_handle(), local_address, length_in_bytes, remote_address, self.debug_assert_handle_is_valid()) };
		Self::parse_status_for_blocking(status, ())
	}
	
	/// Non-blocking remote load (get) operation.
	///
	/// This routine loads a contiguous block of data of `length` bytes from the remote address and puts into the local address.
	///
	/// The local memory is ***not*** safe to use immediately afterwards if 'InProgress' is returned; in this case, flush either the end point or the worker.
	#[inline(always)]
	pub fn non_blocking_load(&self, local_source_address: NonNull<u8>, length_in_bytes: usize) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ErrorCode>
	{
		let local_address = local_source_address.as_ptr()  as *mut c_void;
		let remote_address = self.remote_address(local_source_address);
		
		let status = unsafe { ucp_get_nbi(self.end_point_handle(), local_address, length_in_bytes, remote_address, self.debug_assert_handle_is_valid()) };
		Self::parse_status_for_non_blocking(status)
	}
	
	/// ?Blocking operation that atomically adds a u32 increment to a remote memory location.
	///
	/// The remote address must be 4-byte (32-bit) aligned.
	#[inline(always)]
	pub fn blocking_atomic_add_u32_increment(&self, aligned_remote_address: u64, increment: u32) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ErrorCode>
	{
		debug_assert_eq!(aligned_remote_address % 4, 0, "aligned_remote_address '{}' is not 4-byte (32-bit) aligned", aligned_remote_address);
		
		let status = unsafe { ucp_atomic_add32(self.end_point_handle(), increment, aligned_remote_address, self.debug_assert_handle_is_valid()) };
		Self::parse_status_for_non_blocking(status)
	}
	
	/// ?Blocking operation that atomically adds a u64 increment to a remote memory location.
	///
	/// The remote address must be 8-byte (64-bit) aligned.
	#[inline(always)]
	pub fn blocking_atomic_add_u64_increment(&self, aligned_remote_address: u64, increment: u64) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ErrorCode>
	{
		debug_assert_eq!(aligned_remote_address % 8, 0, "aligned_remote_address '{}' is not 8-byte (64-bit) aligned", aligned_remote_address);
		
		let status = unsafe { ucp_atomic_add64(self.end_point_handle(), increment, aligned_remote_address, self.debug_assert_handle_is_valid()) };
		Self::parse_status_for_non_blocking(status)
	}
	
	/// Blocking operation that atomically adds a u32 increment to a remote memory location and returns the previous value.
	///
	/// Should be thought of as sequentially consistent.
	///
	/// The remote address must be 4-byte (32-bit) aligned.
	#[inline(always)]
	pub fn blocking_atomic_fetch_and_add_u32_increment(&self, aligned_remote_address: u64, increment: u32) -> Result<u32, ErrorCode>
	{
		debug_assert_eq!(aligned_remote_address % 4, 0, "aligned_remote_address '{}' is not 4-byte (32-bit) aligned", aligned_remote_address);
		
		let mut previous_value = unsafe { uninitialized() };
		
		let status = unsafe { ucp_atomic_fadd32(self.end_point_handle(), increment, aligned_remote_address, self.debug_assert_handle_is_valid(), &mut previous_value) };
		Self::parse_status_for_blocking(status, previous_value)
	}
	
	/// Blocking operation that atomically adds a u64 increment to a remote memory location and returns the previous value.
	///
	/// Should be thought of as sequentially consistent.
	///
	/// The remote address must be 8-byte (64-bit) aligned.
	#[inline(always)]
	pub fn blocking_atomic_fetch_and_add_u64_increment(&self, aligned_remote_address: u64, increment: u64) -> Result<u64, ErrorCode>
	{
		debug_assert_eq!(aligned_remote_address % 8, 0, "aligned_remote_address '{}' is not 8-byte (64-bit) aligned", aligned_remote_address);
		
		let mut previous_value = unsafe { uninitialized() };
		
		let status = unsafe { ucp_atomic_fadd64(self.end_point_handle(), increment, aligned_remote_address, self.debug_assert_handle_is_valid(), &mut previous_value) };
		Self::parse_status_for_blocking(status, previous_value)
	}
	
	/// Blocking operation that atomically swaps a u32 value to a remote memory location and returns the previous value.
	///
	/// Should be thought of as sequentially consistent.
	///
	/// The remote address must be 4-byte (32-bit) aligned.
	#[inline(always)]
	pub fn blocking_atomic_swap_u32_value(&self, aligned_remote_address: u64, value_to_swap_for: u32) -> Result<u32, ErrorCode>
	{
		debug_assert_eq!(aligned_remote_address % 4, 0, "aligned_remote_address '{}' is not 4-byte (32-bit) aligned", aligned_remote_address);
		
		let mut previous_value = unsafe { uninitialized() };
		
		let status = unsafe { ucp_atomic_swap32(self.end_point_handle(), value_to_swap_for, aligned_remote_address, self.debug_assert_handle_is_valid(), &mut previous_value) };
		Self::parse_status_for_blocking(status, previous_value)
	}
	
	/// Blocking operation that atomically swaps a u64 value to a remote memory location and returns the previous value.
	///
	/// Should be thought of as sequentially consistent.
	///
	/// The remote address must be 8-byte (64-bit) aligned.
	#[inline(always)]
	pub fn blocking_atomic_swap_u64_value(&self, aligned_remote_address: u64, value_to_swap_for: u64) -> Result<u64, ErrorCode>
	{
		debug_assert_eq!(aligned_remote_address % 8, 0, "aligned_remote_address '{}' is not 8-byte (64-bit) aligned", aligned_remote_address);
		
		let mut previous_value = unsafe { uninitialized() };
		
		let status = unsafe { ucp_atomic_swap64(self.end_point_handle(), value_to_swap_for, aligned_remote_address, self.debug_assert_handle_is_valid(), &mut previous_value) };
		Self::parse_status_for_blocking(status, previous_value)
	}
	
	/// Blocking operation that atomically compares and swaps a u32 value to a remote memory location and returns the previous value (which, if successful, will be the same as the `value_to_expect`).
	///
	/// Should be thought of as sequentially consistent.
	///
	/// The remote address must be 4-byte (32-bit) aligned.
	#[inline(always)]
	pub fn blocking_atomic_compare_and_swap_u32_value(&self, aligned_remote_address: u64, value_to_expect: u32, value_to_swap_for: u32) -> Result<u32, ErrorCode>
	{
		debug_assert_eq!(aligned_remote_address % 4, 0, "aligned_remote_address '{}' is not 4-byte (32-bit) aligned", aligned_remote_address);
		
		let mut previous_value = unsafe { uninitialized() };
		
		let status = unsafe { ucp_atomic_cswap32(self.end_point_handle(), value_to_expect, value_to_swap_for, aligned_remote_address, self.debug_assert_handle_is_valid(), &mut previous_value) };
		Self::parse_status_for_blocking(status, previous_value)
	}
	
	/// Blocking operation that atomically compares and swaps a u32 value to a remote memory location and returns the previous value (which, if successful, will be the same as the `value_to_expect`).
	///
	/// Should be thought of as sequentially consistent.
	///
	/// The remote address must be 8-byte (64-bit) aligned.
	#[inline(always)]
	pub fn blocking_atomic_compare_and_swap_u64_value(&self, aligned_remote_address: u64, value_to_expect: u64, value_to_swap_for: u64) -> Result<u64, ErrorCode>
	{
		debug_assert_eq!(aligned_remote_address % 8, 0, "aligned_remote_address '{}' is not 8-byte (64-bit) aligned", aligned_remote_address);
		
		let mut previous_value = unsafe { uninitialized() };
		
		let status = unsafe { ucp_atomic_cswap64(self.end_point_handle(), value_to_expect, value_to_swap_for, aligned_remote_address, self.debug_assert_handle_is_valid(), &mut previous_value) };
		Self::parse_status_for_blocking(status, previous_value)
	}
	
	#[inline(always)]
	fn remote_address(&self, local_address: NonNull<u8>) -> u64
	{
		self.local_to_remote_address_translation.from_local_address_to_remote_address(local_address)
	}
	
	#[inline(always)]
	fn end_point_handle(&self) -> ucp_ep_h
	{
		self.parent_end_point.debug_assert_handle_is_valid()
	}
	
	#[inline(always)]
	fn parse_status_for_blocking<R>(status: ucs_status_t, result: R) -> Result<R, ErrorCode>
	{
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(result),
			
			Error(error_code) => Err(error_code),
			
			unexpected @ _ => panic!("Unexpected status '{:?}'", unexpected)
		}
	}
	
	#[inline(always)]
	fn parse_status_for_non_blocking(status: ucs_status_t) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ErrorCode>
	{
		use self::Status::*;
		use self::NonBlockingRequestCompletedOrInProgress::*;
		
		match status.parse()
		{
			IsOk => Ok(Completed(())),
			
			OperationInProgress => Ok(InProgress(())),
			
			Error(error_code) => Err(error_code),
			
			unexpected @ _ => panic!("Unexpected status '{:?}'", unexpected)
		}
	}
	
	#[inline(always)]
	fn debug_assert_handle_is_valid(&self) -> ucp_rkey_h
	{
		let handle = self.handle;
		debug_assert!(!handle.is_null(), "handle is null");
		handle
	}
}
