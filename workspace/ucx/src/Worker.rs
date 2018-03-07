// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A worker.
pub struct Worker
{
	handle: ucp_worker_h,
	worker_handle_drop_safety: Rc<WorkerHandleDropSafety>,
	_application_context_handle_drop_safety: Rc<ApplicationContextHandleDropSafety>,
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
		unsafe { ucp_worker_print_info(self.handle, stream) };
	}
}

impl HasAttributes for Worker
{
	type Attributes = WorkerAttributes;
	
	#[inline(always)]
	fn attributes(&self) -> Self::Attributes
	{
		Self::Attributes::query(self.handle)
	}
}

impl Worker
{
	/*
		Creating end points.
		
		- We need to be able to send, out-of-band, `OurRemotelyAccessibleWorkerAddress` and `OurRemotelyAccessibleMemoryAddress` objects.
		- We need to be able to have an up-to-date view of all of these in our cluster.
		- A cluster should have a name.
		- A cluster runs on a fabric.
		- We will be sharing the fabric with others, potentially.
		
		- Idea:-
			- What we want is effectively a LVQ or etcd like configuration
			- Multicast DNS or Broadcast DNS (we just send every second); uses epoll or can be blocking with a time out.
			- Each 'node' (and there may be one per thread) sends messages
			- Messages are symmetrically encrypted with a key or uses a modern libsodium packet; they may have a key identifier to make life easier.
			- Messages have a namespace - essentially, our cluster name.
			- There is a dedicated Multicast DNS thread which maintains an up-to-date map
			- We should consider a logical => physical mapping for address and memory
			
	*/
	
	/// This routine returns the address of the worker object.
	/// This address can be passed to remote instances of the UCP library in order to to connect to this worker.
	#[inline(always)]
	pub fn our_remotely_accessible_worker_address(&self) -> OurRemotelyAccessibleWorkerAddress
	{
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
	
	/// Flushes all outstanding remote memory access ('RMA') and non-blocking atomic memory operations ('AMO').
	#[inline(always)]
	pub fn flush(&self)
	{
		panic_on_error!(ucp_worker_flush, self.handle);
	}
	
	/// Gets a file descriptor (also known as `EVENT_FD`) suitable for use with `epoll`.
	#[inline(always)]
	pub fn get_file_descriptor_suitable_for_epoll(&self) -> RawFd
	{
		let mut file_descriptor = unsafe { uninitialized() };
		panic_on_error!(ucp_worker_get_efd, self.handle, &mut file_descriptor);
		file_descriptor
	}
	
	/// Returns an Err if internal logical returns `UCS_ERR_IO_ERROR`.
	#[inline(always)]
	pub fn block_waiting_for_any_event(&self) -> Result<(), ()>
	{
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
	#[inline(always)]
	pub fn block_waiting_for_a_memory_event(&self, address: *mut u8)
	{
		unsafe { ucp_worker_wait_mem(self.handle, address as *mut _) }
	}
	
	/// Wakes up (signals) a worker blocked waiting (in `block_waiting_for_any_event` or `block_waiting_for_a_memory_event`) or in `epoll`.
	#[inline(always)]
	pub fn wake_up(&self)
	{
		panic_on_error!(ucp_worker_signal, self.handle);
	}
	
	/// Returns 'true' if one should call `ucp_worker_progress()`, ie the worker can not arm because it is 'busy'.
	#[inline(always)]
	pub fn arm(&self) -> bool
	{
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
}
