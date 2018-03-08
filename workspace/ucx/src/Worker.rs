// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A worker is an opaque object representing the communication context.
///
/// A worker represents an instance of a local communication resource and the progress engine associated with it.
/// A progress engine is a construct that is responsible for asynchronous and independent progress of communication directives.
/// A progress engine could be implement in hardware or software.
///
/// A worker abstracts network resources such as a host channel adapter port, network interface, or multiple resources such as
/// multiple network interfaces or communication ports.
/// It can also represent virtual communication resources that are defined across multiple devices.
///
/// Although the worker can represent multiple network resources, it is associated with a single `ApplicationContext`.
///
/// All communication functions require a context to perform the operation on the dedicated hardware resource(s) and an `EndPoint` to address the destination.
///
/// NOTE: Workers are parallel 'threading points' that an upper layer may use to optimize concurrent communications.
#[derive(Clone)]
pub struct Worker
{
	pub(crate) handle: ucp_worker_h,
	worker_handle_drop_safety: Rc<WorkerHandleDropSafety>,
}

impl Debug for Worker
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		self.debug_fmt(f)
	}
}

impl PrintInformation for Worker
{
	const DebugName: &'static str = "Worker";
	
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		if handle.is_null()
		{
			return;
		}
		
		unsafe { ucp_worker_print_info(self.handle, stream) };
	}
}

impl HasAttributes for Worker
{
	type Attributes = WorkerAttributes;
	
	#[inline(always)]
	fn attributes(&self) -> Self::Attributes
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		Self::Attributes::query(self.handle)
	}
}

impl Worker
{
	/// Creates an end point to connected to a their_remote_address worker.
	///
	/// `peer_failure_error_handler` is moved into the `EndPoint`.
	///
	/// `guarantee_that_send_requests_are_always_completed_successfully_or_error` has some advantages:-
	/// * guarantees that send requests are always completed, ie the peer has to be alive.
	/// * stops hangs
	/// * stops undefined behaviour in the event of peer failure
	///
	/// It has a number of significant impacts:-
	/// * it disables protocols and APIs which may cause a hang or undefined behavior in case of peer failure
	/// * it may affect performance
	/// * it may increase memory footprint
	#[inline(always)]
	pub fn new_end_point<E: EndPointPeerFailureErrorHandler>(&self, peer_failure_error_handler: E, their_remote_address: TheirRemoteAddress, guarantee_that_send_requests_are_always_completed_successfully_or_error: bool)
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		EndPoint::new_end_point(peer_failure_error_handler, their_remote_address, guarantee_that_send_requests_are_always_completed_successfully_or_error, self)
	}
	
	const ReservedForFutureUseFlags: u32 = 0;
	
	/// This non-blocking routine returns endpoints on a worker which are ready to consume streaming data.
	/// The ready end points are put into `end_points`.
	/// On success, the `end_points` will have been overwritten with ready end points.
	///
	/// NOTE: The value of `end_points.len()` is ignored on entry.
	#[inline(always)]
	pub fn which_end_points_are_ready_to_consume_streaming_data(&self, end_points: &mut Vec<EndPointReadToConsumeStreamingData>) -> Result<(), ErrorCode>
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		let maximum_end_points = end_points.capacity();
		
		let result = unsafe { ucp_stream_worker_poll(self.handle, end_points.as_mut_ptr() as *mut _, maximum_end_points, Self::ReservedForFutureUseFlags) };
		if result >= 0
		{
			let count = result as usize;
			debug_assert!(count <= maximum_end_points);
			unsafe { end_points.set_len(count) }
			
			Ok(())
		}
		else
		{
			let status = result as ucs_status_t;
			Err(status.error_code_or_panic())
		}
	}
	
	pub const MaximumEndPoints: usize = 64;
	
	/// Identical to `which_end_points_are_ready_to_consume_streaming_data` but uses a fixed size, stack-friendly array.
	///
	/// Number of end points is returned in the result.
	#[inline(always)]
	pub fn which_end_points_are_ready_to_consume_streaming_data_optimized(&self, end_points: &mut [EndPointReadToConsumeStreamingData; Self::MaximumEndPoints]) -> Result<usize, ErrorCode>
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		let result = unsafe { ucp_stream_worker_poll(self.handle, end_points.as_mut_ptr() as *mut _, Self::MaximumEndPoints, Self::ReservedForFutureUseFlags) };
		if result >= 0
		{
			let count = result as usize;
			debug_assert!(count <= Self::MaximumEndPoints);
			Ok(count)
		}
		else
		{
			let status = result as ucs_status_t;
			Err(status.error_code_or_panic())
		}
	}
	
	/// A server listener listens for incoming client connections on a particular address.
	/// It then creates ?end points? to handle them.
	#[inline(always)]
	pub fn create_server_listener<L: ServerListenerAcceptHandler>(&self, our_listening_socket: NixSockAddr, server_listener_accept_handler: L) -> Result<ServerListener<L>, ErrorCode>
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		let mut server_listener = Box::new
		(
			ServerListener
			{
				handle: null_mut(),
				worker_handle_drop_safety: self.worker_handle_drop_safety.clone(),
				server_listener_accept_handler,
			}
		);
		
		let (socket_address, length) = our_listening_socket.as_ffi_pair();
		
		let parameters = ucp_listener_params_t
		{
			field_mask: u64,
			sockaddr: ucs_sock_addr_t
			{
				addr: socket_address,
				addrlen: length,
			},
			accept_handler: ucp_listener_accept_handler_t
			{
				cb: Some(ServerListener::<L>::accept_callback),
				arg: (&server_listener.listener_accept_handler) as *const _ as *mut _,
			},
		};
		
		let mut handle = unsafe { uninitialized() };
		
		let status = unsafe { ucp_listener_create(self.handle, &parameters, &mut handle) };
		
		use self::Status::*;
		
		match status.parse().expect("Invalid status")
		{
			IsOk =>
			{
				server_listener.handle = handle;
				Ok(server_listener)
			}
			
			Error(error_code) => Err(error_code),
			
			unexpected @ _ => panic!("Unexpected status '{:?}'", unexpected)
		}
	}
	
	/// This routine returns the address of the worker object.
	/// This address can be passed to remote instances of the UCP library in order to to connect to this worker.
	#[inline(always)]
	pub fn our_remotely_accessible_worker_address(&self) -> OurRemotelyAccessibleWorkerAddress
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		let mut address = unsafe { uninitialized() };
		let mut length = unsafe { uninitialized() };
		panic_on_error!(ucp_worker_get_address, self.handle, &mut address, &mut length);
		
		debug_assert!(!address.is_null(), "handle is null");
		debug_assert_ne!(length, 0, "length is zero");
		
		OurRemotelyAccessibleWorkerAddress
		{
			address: unsafe { NonNull::new_unchecked(address as *mut u8) },
			length,
			worker_handle: self.handle,
			worker_handle_drop_safety: self.worker_handle_drop_safety.clone(),
		}
	}
	
	/// Blocks until a non-blocking operation is complete.
	#[inline(always)]
	pub fn block_until_non_blocking_operation_is_complete(&self, status_pointer: ucs_status_ptr_t) -> Result<(), ErrorCode>
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		NonBlockingRequest::block_until_non_blocking_operation_is_complete(self, status_pointer)
	}
	
	/// Flushes all outstanding remote memory access ('RMA') and non-blocking atomic memory operations ('AMO') on all end points.
	///
	/// Blocking.
	#[inline(always)]
	pub fn blocking_flush_all_end_points(&self)
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		panic_on_error!(ucp_worker_flush, self.handle);
	}
	
	/// Flushes all outstanding remote memory access ('RMA') and non-blocking atomic memory operations ('AMO') on all end points.
	///
	/// Non-blocking.
	///
	/// The `callback_when_finished_or_cancelled` will receive an ErrorCode(Cancelled) if the non-blocking request is cancelled.
	#[inline(always)]
	pub fn non_blocking_flush_all_end_points(&self, callback_when_finished_or_cancelled: ucp_send_callback_t) -> StatusOrNonBlockingRequest
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		const ReservedForFutureUseFlags: u32 = 0;
		
		let status_pointer = unsafe { ucp_worker_flush_nb(self.handle, ReservedForFutureUseFlags, callback_when_finished_or_cancelled) };
		status_pointer.parse().unwrap("Invalid status_pointer")
	}
	
	/// Assures ordering between non-blocking operations.
	///
	/// This routine ensures ordering of non-blocking communication operations on the UCP worker.
	/// Communication operations issued on the worker prior to this call are guaranteed to be completed before any subsequent communication operations to the same worker which follow the call to `fence()`.
	///
	/// The primary difference between `fence()` and `flush_non_blocking` is that the fence  routine does not guarantee completion of the operations on the call return but only ensures the order between communication operations.
	/// The `flush_non_blocking` operation on return guarantees that all operations are completed and corresponding memory regions were updated.
	#[inline(always)]
	pub fn fence(&self)
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		panic_on_error!(ucp_worker_fence, self.handle);
	}
	
	/// This routine explicitly progresses all communication operations on a worker.
	///
	/// Returns true if there are still outstanding events to process on the worker.
	///
	/// (A sort of alternative to poll, this method is used every now and then to progress non-blocking).
	///
	/// * Typically, request wait and test routines call `progress` to progress any outstanding operations.
	/// * Transport layers, implementing asynchronous progress using threads, require callbacks and other user code to be thread safe.
	/// * The state of communication can be advanced (progressed) by blocking routines. Nevertheless, the non-blocking routines can not be used for communication progress.
	#[inline(always)]
	pub fn progress(&self) -> bool
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		let result = panic_on_error!(ucp_worker_progress, self.handle);
		result != 0
	}
	
	/// Returns an Err if internal logical returns `UCS_ERR_IO_ERROR`.
	///
	/// Do not call this is `progress()` returned true.
	#[inline(always)]
	pub fn block_waiting_for_any_event(&self) -> Result<(), ()>
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		panic_on_error_with_clean_up!
		(
			status,
			{
				if status == ucs_status_t::UCS_ERR_IO_ERROR
				{
					return Err(())
				};
			},
			ucp_worker_wait,
			self.handle
		);
		Ok(())
	}
	
	/// Block waiting for a memory event.
	///
	/// Do not call this is `progress()` returned true.
	#[inline(always)]
	pub fn block_waiting_for_a_memory_event(&self, address: *mut u8)
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		unsafe { ucp_worker_wait_mem(self.handle, address as *mut _) }
	}
	
	/// Wakes up (signals) a worker blocked waiting (in `block_waiting_for_any_event` or `block_waiting_for_a_memory_event`) or in `epoll`.
	#[inline(always)]
	pub fn wake_up(&self)
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		panic_on_error!(ucp_worker_signal, self.handle);
	}
	
	/// Returns 'true' if one should call `ucp_worker_progress()`, ie the worker can not arm because it is 'busy'.
	#[inline(always)]
	pub fn arm(&self) -> bool
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		panic_on_error_with_clean_up!
		(
			status,
			{
				if status.is_busy()
				{
					return true
				}
			},
			ucp_worker_arm,
			self.handle
		);
		false
	}
	
	/// Gets a file descriptor (also known as `EVENT_FD`) suitable for use with `epoll`.
	#[inline(always)]
	pub fn get_file_descriptor_suitable_for_epoll(&self) -> RawFd
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		let mut file_descriptor = unsafe { uninitialized() };
		panic_on_error!(ucp_worker_get_efd, self.handle, &mut file_descriptor);
		file_descriptor
	}
}
