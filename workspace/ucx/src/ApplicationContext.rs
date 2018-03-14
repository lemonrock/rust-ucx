// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An application context holds an UCP communication instance's global information.
/// It represents a single UCP communication instance.
/// The communication instance could be an OS process (an application) that uses UCP library.
/// This global information includes communication resources, endpoints, memory, temporary file storage, and other communication information directly associated with a specific UCP instance.
/// The context also acts as an isolation mechanism, allowing resources associated with the context to manage multiple concurrent communication instances.
/// For example, users using both MPI and OpenSHMEM sessions simultaneously can isolate their communication by allocating and using separate contexts for each of them.
/// Alternatively, users can share the communication resources (memory, network resource context, etc) between them by using the same application context.
/// A message sent or a RMA operation performed in one application context cannot be received in any other application context.
pub struct ApplicationContext<MemoryCustomization = NoNonBlockingRequestMemoryCustomization>
{
	handle: ucp_context_h,
	application_context_handle_drop_safety: Rc<ApplicationContextHandleDropSafety>,
	application_context_configuration: ApplicationContextConfiguration,
	sealing_key: SealingKey,
	opening_key: OpeningKey,
	master_their_remotely_accessible: MasterTheirRemotelyAccessible,
	phantom_data: PhantomData<MemoryCustomization>,
}

impl<MemoryCustomization: NonBlockingRequestMemoryCustomization> Debug for ApplicationContext<MemoryCustomization>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		self.debug_fmt(f)
	}
}

impl<MemoryCustomization: NonBlockingRequestMemoryCustomization> PrintInformation for ApplicationContext<MemoryCustomization>
{
	const DebugName: &'static str = "ApplicationContext";
	
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		unsafe { ucp_context_print_info(self.handle, stream) };
	}
}

impl<MemoryCustomization: NonBlockingRequestMemoryCustomization> HasAttributes for ApplicationContext<MemoryCustomization>
{
	type Attributes = ApplicationContextAttributes;
	
	#[inline(always)]
	fn attributes(&self) -> Self::Attributes
	{
		Self::Attributes::query(self.handle)
	}
}

impl<MemoryCustomization: NonBlockingRequestMemoryCustomization> ApplicationContext<MemoryCustomization>
{
	/// Makes a region of memory remotely accessible (also known as 'mapping', 'registering' or 'pinning').
	/// Panics on any error, including out-of-memory.
	/// Remotely accessible memory is *not* needed for sending or receiving tagged messages.
	///
	/// If passing a `memory_address` of `AllocateAndRegister` or `AllocateAndRegisterWithAddressAddress`, the resultant allocated address can be found by calling `OurRemotelyAccessibleMemory.attributes().address()` .
	///
	/// `length` must not be zero.
	#[inline(always)]
	pub fn register_memory_as_remotely_accessible(&self, memory_address: OurLocalMemoryAddressToMakeRemotelyAccessible, length: usize, non_blocking: bool) -> OurRemotelyAccessibleMemory
	{
		assert_ne!(length, 0, "length is zero");
		
		let (address, mut flags) = memory_address.address_and_flags();
		
		if non_blocking
		{
			flags |= _bindgen_ty_2::NONBLOCK;
		}
		
		let parameters = ucp_mem_map_params_t
		{
			field_mask: (ucp_mem_map_params_field::ADDRESS | ucp_mem_map_params_field::LENGTH | ucp_mem_map_params_field::FLAGS).0 as u64,
			address: address as *mut c_void,
			length,
			flags: flags.0 as _,
		};
		
		let mut handle = unsafe { uninitialized() };
		panic_on_error!(ucp_mem_map, self.handle, &parameters, &mut handle);
		OurRemotelyAccessibleMemory
		{
			handle,
			our_remotely_accessible_memory_handle_drop_safety: Rc::new(OurRemotelyAccessibleMemoryHandleDropSafety(handle, self.application_context_handle_drop_safety.clone())),
			application_context_handle: self.handle,
		}
	}
	
	/// Creates a new worker for a hyper thread.
	#[inline(always)]
	pub fn new_worker_for_hyper_thread(&self, our_hyper_thread_index: ZeroBasedHyperThreadIndex) -> Worker
	{
		let parameters = ucp_worker_params_t
		{
			field_mask: (ucp_worker_params_field::THREAD_MODE | ucp_worker_params_field::CPU_MASK | ucp_worker_params_field::EVENTS | ucp_worker_params_field::USER_DATA).0 as u64,
			thread_mode: WorkerThreadMode::OnlyEverUsedFromThisThread.as_ucs_thread_mode_t(),
			cpu_mask: ucs_cpu_set_t::create_for_hyper_thread(our_hyper_thread_index),
			events: self.application_context_configuration.wake_up_events().0,
			user_data: null_mut(),
			event_fd: 0,
		};

		let mut handle = unsafe { uninitialized() };
		panic_on_error!(ucp_worker_create, self.handle, &parameters, &mut handle);

		Worker
		{
			handle,
			worker_handle_drop_safety: Rc::new(WorkerHandleDropSafety(handle, self.application_context_handle_drop_safety.clone())),
		}
	}
	
	/// Get remotely accessible worker end point address.
	/// Whilst there can be only multiple remote workers, we only need to get one to be able to access remote memory regions.
	#[inline(always)]
	pub fn get_remotely_accessible_worker_end_point_address(&self, application_context_name: &ApplicationContextName, worker_name: &WorkerName) -> Option<Rc<TheirRemotelyAccessibleWorkerEndPointAddress>>
	{
		if let Some(remote_application_context) = self.master_their_remotely_accessible.read().get(application_context_name)
		{
			remote_application_context.get_remotely_accessible_worker_end_point_address(worker_name).map(|reference| reference.clone())
		}
		else
		{
			None
		}
	}
	
	/// Set remotely accessible worker end point address.
	#[inline(always)]
	pub fn set_remotely_accessible_worker_address(&self, application_context_name: ApplicationContextName, worker_name: WorkerName, worker_address: TheirRemotelyAccessibleWorkerEndPointAddress)
	{
		self.master_their_remotely_accessible.update::<(), _>(|master|
		{
			master.get_mut_or_create(application_context_name).set_remotely_accessible_worker_end_point_address(worker_name, worker_address);
			Ok(())
		}).expect("Was OK")
	}
	
	/// Get remotely accessible server end point address.
	#[inline(always)]
	pub fn get_remotely_accessible_server_end_point_address(&self, application_context_name: &ApplicationContextName, worker_and_server_name: &(WorkerName, ServerName)) -> Option<Rc<TheirRemotelyAccessibleServerEndPointAddress>>
	{
		if let Some(remote_application_context) = self.master_their_remotely_accessible.read().get(application_context_name)
		{
			remote_application_context.get_remotely_accessible_server_end_point_address(worker_and_server_name).map(|reference| reference.clone())
		}
		else
		{
			None
		}
	}
	
	/// Set remotely accessible server end point address.
	#[inline(always)]
	pub fn set_remotely_accessible_server_end_point_address(&self, application_context_name: ApplicationContextName, worker_and_server_name: (WorkerName, ServerName), server_address: TheirRemotelyAccessibleServerEndPointAddress)
	{
		self.master_their_remotely_accessible.update::<(), _>(|master|
		{
			master.get_mut_or_create(application_context_name).set_remotely_accessible_server_end_point_address(worker_and_server_name, server_address);
			Ok(())
		}).expect("Was OK")
	}
	
	/// Get remotely accessible memory address.
	#[inline(always)]
	pub fn get_remotely_accessible_memory_address(&self, application_context_name: &ApplicationContextName, memory_name: &MemoryName) -> Option<Rc<TheirRemotelyAccessibleMemoryAddress>>
	{
		if let Some(remote_application_context) = self.master_their_remotely_accessible.read().get(application_context_name)
		{
			remote_application_context.get_remotely_accessible_memory_address(memory_name).map(|reference| reference.clone())
		}
		else
		{
			None
		}
	}
	
	/// Set remotely accessible memory address.
	#[inline(always)]
	pub fn set_remotely_accessible_memory_address(&self, application_context_name: ApplicationContextName, memory_name: MemoryName, memory_address: TheirRemotelyAccessibleMemoryAddress)
	{
		self.master_their_remotely_accessible.update::<(), _>(|master|
		{
			master.get_mut_or_create(application_context_name).set_remotely_accessible_memory_address(memory_name, memory_address);
			Ok(())
		}).expect("Was OK")
	}
}
