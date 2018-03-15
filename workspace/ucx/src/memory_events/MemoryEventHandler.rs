// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A trait to help with handling memory events.
///
/// Wraps access to a `MemoryEventUser`.
///
/// UCX currently provides no way for this object to be freed, so implementations must expect it to live with a `'static` lifetime.
///
/// No implementation inside this handler is permitted to call a memory allocation routine, ie anything that would result in a call to `mmap`, `munmap`, `mremap`, `shmat`, `shmdt`, `sbrk` (eg malloc), `malloc` or `free`, as this can cause infinite recursion.
pub trait MemoryEventHandler
{
	/// The type of a passed argument.
	type User: MemoryEventUser;
	
	/// This function converts the callback to an instance of T, a memory event handler
	#[inline(always)]
	fn callback_arg_to_memory_event_user<'fake>(arg: NonNull<u8>) -> &'fake Self::User;
	
	#[doc(hidden)]
	unsafe extern "C" fn callback(event_type: ucm_event_type_t, event: *mut ucm_event_t, arg: *mut c_void)
	{
		debug_assert!(!event.is_null(), "event is null");
		let mut event = NonNull::new_unchecked(event);
		let event = event.as_mut();
		
		debug_assert!(!arg.is_null(), "arg is null");
		let memory_event_user = Self::callback_arg_to_memory_event_user(NonNull::new_unchecked(arg as *mut u8));
		
		match event_type
		{
			ucm_event_type_t::MMAP => memory_event_user.use_mmap_event(event.mmap.as_mut()),
			
			ucm_event_type_t::MUNMAP => memory_event_user.use_munmap_event(event.munmap.as_mut()),
			
			ucm_event_type_t::MREMAP => memory_event_user.use_mremap_event(event.mremap.as_mut()),
			
			ucm_event_type_t::SHMAT => memory_event_user.use_shmat_event(event.shmat.as_mut()),
			
			ucm_event_type_t::SHMDT => memory_event_user.use_shmdt_event(event.shmdt.as_mut()),
			
			ucm_event_type_t::SBRK => memory_event_user.use_sbrk_event(event.sbrk.as_mut()),
			
			ucm_event_type_t::VM_MAPPED => memory_event_user.use_virtual_machine_mmap_event(transmute(event.vm_mapped.as_ref())),
			
			ucm_event_type_t::VM_UNMAPPED => memory_event_user.use_virtual_machine_munmap_event(transmute(event.vm_unmapped.as_ref())),
			
			ucm_event_type_t::MEM_TYPE_ALLOC =>
			{
				let gpu_memory_event = event.mem_type.as_ref();
				
				match gpu_memory_event.mem_type
				{
					ucm_mem_type::CUDA =>memory_event_user.use_cuda_gpu_alloc_event(transmute(gpu_memory_event)),
					
					_ => panic!("Unknown GPU memory type")
				}
			}
			
			ucm_event_type::MEM_TYPE_FREE =>
			{
				let gpu_memory_event = event.mem_type.as_ref();
				
				match gpu_memory_event.mem_type
				{
					ucm_mem_type::CUDA => memory_event_user.use_cuda_gpu_free_event(transmute(gpu_memory_event)),
					
					_ => panic!("Unknown GPU memory type")
				}
			}
			
			_ => panic!("Invalid event type"),
		}
	}
}
