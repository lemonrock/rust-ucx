// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A special trait to allow customization of request memory used for non-blocking requests in application contexts.
pub trait NonBlockingRequestMemoryCustomization
{
	/// The size of a reserved space in a non-blocking requests.
	/// Typically applications use this space for caching their own structures in order to avoid costly memory allocations, pointer dereferences, and cache misses.
	/// For example, MPI implementations can use this memory for caching MPI descriptors.
	///
	/// Defaults to zero.
	const reserved_space_in_non_blocking_requests: usize = 0;
	
	/// Function pointer to a routine that is used for a non-blocking request initialization.
	/// This function will be called only on the very first time a request memory is initialized, and may not be called again if a request is reused.
	/// If a request should be reset before the next reuse, it can be done before calling `ucp_request_free`.
	///
	/// The size of the memory pointed to by `request` will be `reserved_space_in_non_blocking_requests`.
	///
	/// Defaults to doing nothing.
	#[allow(unused_variables)]
	unsafe extern "C" fn initialize(request: *mut c_void)
	{
	}
	
	/// Function pointer to a routine that is responsible for final cleanup of the memory associated with a non-blocking request.
	/// This routine may not be called every time a request is released.
	/// For some implementations, the cleanup call may be delayed and only invoked at `ucp_worker_cleanup`.
	/// The size of the memory pointed to by `request` will be `reserved_space_in_non_blocking_requests`.
	///
	/// Defaults to doing nothing.
	///
	#[allow(unused_variables)]
	unsafe extern "C" fn clean_up(request: *mut c_void)
	{
	}
	
	/// Provide function pointers to `initialize()` and `clean_up()`.
	/// Override if need to supply alternative.
	///
	/// Defaults to `(Some(Self::initialize), Some(Self::clean_up))`
	#[inline(always)]
	fn function_pointers() -> (ucp_request_init_callback_t, ucp_request_cleanup_callback_t)
	{
		(Some(Self::initialize), Some(Self::clean_up))
	}
}
