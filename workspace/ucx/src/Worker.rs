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
		if self.handle.is_null()
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
	/// Maximum number of end points for stack-optimized functions; nothing to do with UCX.
	pub const MaximumEndPoints: usize = 64;
	
	/// Creates an end point to connected to a their_remote_address worker.
	///
	/// Errors are not differentiated.
	/// The following may indicate that the application should not terminated:-
	/// * `NoResourcesAreAvailableToInitiateTheOperation`
	///
	/// These might indicate 'try again immediately', although it's not clear if they are ever returned from UCX for this logic:-
	/// * `DeviceIsBusy`
	/// * `NoResourcesAreAvailableToInitiateTheOperation`
	///
	/// These might indicate trying again with a different destination:-
	/// * `DestinationIsUnreachable`
	/// * `InputOutputError`
	///
	/// Other:-
	/// * `InvalidAddress` (the destination address format is invalid).
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
	pub fn new_end_point<E: EndPointPeerFailureErrorHandler>(&self, peer_failure_error_handler: E, their_remote_address: &Arc<TheirRemoteAddress>, guarantee_that_send_requests_are_always_completed_successfully_or_error: bool) -> Result<Rc<RefCell<EndPoint<E>>>, ErrorCode>
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		EndPoint::new_end_point(peer_failure_error_handler, their_remote_address, guarantee_that_send_requests_are_always_completed_successfully_or_error, self)
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
	
	/// A server listener listens for incoming client connections on a particular address.
	/// It then creates ?end points? to handle them.
	#[inline(always)]
	pub fn create_server_listener<L: ServerListenerAcceptHandler>(&self, our_listening_socket: NixSockAddr, server_listener_accept_handler: L) -> Result<Box<ServerListener<L>>, ErrorCode>
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		ServerListener::create_server_listener(our_listening_socket, server_listener_accept_handler, &self.worker_handle_drop_safety, self.handle)
	}
	
	/// This non-blocking routine returns endpoints on a worker which are ready to consume streaming data.
	/// The ready end points are put into `end_points`.
	/// On success, the `end_points` will have been overwritten with ready end points.
	///
	/// NOTE: The value of `end_points.len()` is ignored on entry.
	#[inline(always)]
	pub fn which_end_points_are_ready_to_consume_streaming_data(&self, end_points: &mut Vec<EndPointReadyToConsumeStreamingData>) -> Result<(), ErrorCode>
	{
		let maximum_end_points = end_points.capacity();
		self.ucp_stream_worker_poll(end_points.as_mut_ptr(), maximum_end_points).map(|number_of_end_points_ready| unsafe { end_points.set_len(number_of_end_points_ready) })
	}
	
	/// Identical to `which_end_points_are_ready_to_consume_streaming_data` but uses a fixed size, stack-friendly array.
	///
	/// Number of end points is returned in the result.
	#[inline(always)]
	pub fn which_end_points_are_ready_to_consume_streaming_data_optimized(&self, end_points: &mut [EndPointReadyToConsumeStreamingData; Self::MaximumEndPoints]) -> Result<usize, ErrorCode>
	{
		self.ucp_stream_worker_poll(end_points.as_mut_ptr(), Self::MaximumEndPoints)
	}
	
	#[inline(always)]
	fn ucp_stream_worker_poll(&self, end_points: *mut EndPointReadyToConsumeStreamingData, maximum_end_points: usize) -> Result<usize, ErrorCode>
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		debug_assert!(!end_points.is_null(), "end_points is null");
		
		let result = unsafe { ucp_stream_worker_poll(self.handle, end_points as *mut _, maximum_end_points, ReservedForFutureUseFlags) };
		if result >= 0
		{
			let count = result as usize;
			debug_assert!(count <= Self::MaximumEndPoints);
			Ok(count)
		}
		else
		{
			debug_assert!(result >= (::std::i8::MIN as isize), "result is out-of-range");
			let status: ucs_status_t = unsafe { transmute(result as i8) };
			Err(status.error_code_or_panic())
		}
	}
	
	#[inline(always)]
	pub(crate) fn parse_status_pointer<'worker>(&'worker self, status_pointer: ucs_status_ptr_t) -> Result<Option<WorkerWithNonBlockingRequest<'worker>>, ErrorCode>
	{
		debug_assert!(!self.handle.is_null(), "handle is null");
		
		use self::Status::*;
		use self::StatusOrNonBlockingRequest::*;
		
		match status_pointer.parse()
		{
			Status(IsOk) => Ok(None),
			
			Status(Error(error_code)) => Err(error_code),
			
			NonBlockingRequest(non_blocking_request) => Ok(Some(WorkerWithNonBlockingRequest::new(self, non_blocking_request))),
			
			unexpected @ _ => panic!("Unexpected status '{:?}'", unexpected)
		}
	}
	
	#[inline(always)]
	pub(crate) fn block_until_non_blocking_request_is_complete<'worker>(&'worker self, status_pointer: ucs_status_ptr_t) -> Result<(), ErrorCode>
	{
		match self.parse_status_pointer(status_pointer)
		{
			Ok(Some(worker_with_non_blocking_request)) => worker_with_non_blocking_request.block_until_non_blocking_request_is_complete(),
			
			Ok(None) => Ok(()),
			
			Err(error_code) => Err(error_code),
		}
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
		
		let status_pointer = unsafe { ucp_worker_flush_nb(self.handle, ReservedForFutureUseFlags, callback_when_finished_or_cancelled) };
		status_pointer.parse()
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
		
		unsafe { ucp_worker_progress(self.handle) }.from_c_bool()
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
