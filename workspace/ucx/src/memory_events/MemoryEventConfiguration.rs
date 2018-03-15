// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


bitflags!
{
	/// Memory event types.
	pub struct MemoryEventConfiguration: u32
	{
		/// Native mmap event.
		const mmap = 1 << 0;
		
		/// Native munmap event.
		const munmap = 1 << 1;
		
		/// Native mremap event.
		const mremap = 1 << 2;
		
		/// Native shmat event.
		const shmat = 1 << 3;
		
		/// Native shmdt event.
		const shmdt = 1 << 4;
		
		/// Native sbrk event.
		const sbrk = 1 << 5;
		
		/// Aggregate virtual machine mmap event.
		const virtual_machine_mmap = 1 << 16;
		
		/// Aggregate virtual machine munmap event.
		const virtual_machine_munmap = 1 << 17;
		
		/// Unaccessible malloc event (currently only for CUDA).
		const gpu_alloc = 1 << 20;
		
		/// Unaccessible free event (currently only for CUDA).
		const gpu_free = 1 << 21;
	}
}

impl MemoryEventConfiguration
{
	/// Adds a memory events handler.
	#[inline(always)]
	pub fn add_memory_events_handlers<EventHandler: MemoryEventHandler>(&self, priority: MemoryEventHandlerPriority, memory_event_handler: &'static EventHandler, do_not_install_memory_event_hooks: bool) -> Result<(), ErrorCode>
	{
		let mut events = ucm_event_type_t(self.bits);
		if do_not_install_memory_event_hooks
		{
			events |= ucm_event_type_t::FLAG_NO_INSTALL
		}
		let priority = priority.to_priority();
		let callback = Some(EventHandler::callback as unsafe extern "C" fn(event_type: ucm_event_type_t, event: *mut ucm_event_t, arg: *mut c_void));
		let arg = memory_event_handler as *const EventHandler as *mut EventHandler as *mut c_void;
		
		let status = unsafe { ucm_set_event_handler(events.0 as i32, priority, callback, arg) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(()),
			
			Error(error_code) => Err(error_code),
			
			_ => panic!("Unexpected status '{:?}'", status)
		}
	}
	
	/// Removes a memory events handler.
	#[inline(always)]
	pub fn remove_memory_events_handlers<EventHandler: MemoryEventHandler>(&self, memory_event_handler: &'static EventHandler)
	{
		let events = ucm_event_type_t(self.bits);
		let callback = Some(EventHandler::callback as unsafe extern "C" fn(event_type: ucm_event_type_t, event: *mut ucm_event_t, arg: *mut c_void));
		let arg = memory_event_handler as *const EventHandler as *mut EventHandler as *mut c_void;
		
		unsafe { ucm_unset_event_handler(events.0 as i32, callback, arg) };
	}
	
	/// Adds memory events to the external events list.
 	///
	/// When the event is set to be external, it means that user is responsible for handling it.
	/// So, setting a handler for external event will not trigger installing of UCM memory hooks (if they were not installed before).
	/// In this case the corresponding UCM function needs to be invoked to trigger event handlers.
	///
	/// To take an effect, the event should be set external prior to adding event handlers for it.
	///
	/// An usage example is when the user disables UCM memory hooks (because the user has its own hooks; an example is Open MPI), but it wants to use some UCM based functionality, eg use of the IB registration cache.
	/// (The IB registration cache needs to be notified about `virtual_machine_munmap` events, therefore one adds a specific handler for it. In this case user needs to declare the `virtual_machine_munmap` event as external and explicitly call `virtual_machine_munmap()` when some memory release operation occurs).
	#[inline(always)]
	pub fn set_to_be_external(&self)
	{
		unsafe { ucm_set_external_event(self.bits as i32) }
	}
	
	/// Remove memory events from the external events list.
	///
	// When the event is removed from the external events list, any subsequent call to `add_memory_events_handlers()` for that event will trigger installing of UCM memory hooks (if they are enabled and were not installed before).
	#[inline(always)]
	pub fn unset_being_external(&self)
	{
		unsafe { ucm_unset_external_event(self.bits as i32) }
	}
}
