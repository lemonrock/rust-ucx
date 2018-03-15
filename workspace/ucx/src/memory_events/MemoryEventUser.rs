// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A trait to use memory events.
///
/// Events are dispatched in order of callback priority (low to high).
///
/// The `result` field of a memory event is initialized to an invalid erroneous return value if the event can modify the output.
///
/// The callback is allowed to modify the fields where a mutable event is passed, and those modifications will be passed to the next callback.
/// Also, the callback is allowed to modify the `result`, but **only if it's currently invalid**.
/// A valid result indicates that a previous callback already performed the requested memory operation, so a callback should **refrain from actions with side-effects** in this case.
///
/// If the result is still invalid after all callbacks are called, the parameters, possibly modified by the callbacks, will be passed to the original handler.
pub trait MemoryEventUser
{
	// /// Registers this event handler.
	
	/// Handle a mmap event.
	///
	/// Will be called both before and after the `mmap` call.
	///
	/// It is possible to modify (mutate) the event object.
	///
	/// Default implementation does nothing.
	#[allow(unused_variables)]
	#[inline(always)]
	fn use_mmap_event(&self, event: &mut MMapEvent)
	{
	}
	
	//noinspection SpellCheckingInspection
	/// Handle a munmap event.
	///
	/// It is possible to modify (mutate) the event object.
	///
	/// Default implementation does nothing.
	#[allow(unused_variables)]
	#[inline(always)]
	fn use_munmap_event(&self, event: &mut MUnmapEvent)
	{
	}
	
	//noinspection SpellCheckingInspection
	/// Handle a mremap event.
	///
	/// It is possible to modify (mutate) the event object.
	///
	/// Default implementation does nothing.
	#[allow(unused_variables)]
	#[inline(always)]
	fn use_mremap_event(&self, event: &mut MRemapEvent)
	{
	}
	
	//noinspection SpellCheckingInspection
	/// Handle a shmat event.
	///
	/// It is possible to modify (mutate) the event object.
	///
	/// Default implementation does nothing.
	#[allow(unused_variables)]
	#[inline(always)]
	fn use_shmat_event(&self, event: &mut ShmAtEvent)
	{
	}
	
	//noinspection SpellCheckingInspection
	/// Handle a shmdt event.
	///
	/// It is possible to modify (mutate) the event object.
	///
	/// Default implementation does nothing.
	#[allow(unused_variables)]
	#[inline(always)]
	fn use_shmdt_event(&self, event: &mut ShmDtEvent)
	{
	}
	
	//noinspection SpellCheckingInspection
	/// Handle a sbrk event.
	///
	/// It is possible to modify (mutate) the event object.
	///
	/// Default implementation does nothing.
	#[allow(unused_variables)]
	#[inline(always)]
	fn use_sbrk_event(&self, event: &mut SBrkEvent)
	{
	}
	
	//noinspection SpellCheckingInspection
	/// Handle a virtual machine mmap event.
	///
	/// Will be called after the virtual machine mmap event.
	///
	/// It is not possible to modify (mutate) the event object and it does not have a `result`.
	///
	/// Default implementation does nothing.
	#[allow(unused_variables)]
	#[inline(always)]
	fn use_virtual_machine_mmap_event(&self, event: &VirtualMachineMMapEvent)
	{
	}
	
	//noinspection SpellCheckingInspection
	/// Handle a virtual machine munmap event.
	///
	/// Will be called before the virtual machine mmap event.
	///
	/// It is not possible to modify (mutate) the event object and it does not have a `result`.
	///
	/// Default implementation does nothing.
	#[allow(unused_variables)]
	#[inline(always)]
	fn use_virtual_machine_munmap_event(&self, event: &VirtualMachineMUnmapEvent)
	{
	}
	
	//noinspection SpellCheckingInspection
	/// Handle a GPU alloc event.
	///
	/// It is not possible to modify (mutate) the event object and it does not have a `result`.
	///
	/// Default implementation does nothing.
	#[allow(unused_variables)]
	#[inline(always)]
	fn use_cuda_gpu_alloc_event(&self, event: &MemoryGpuAllocEvent)
	{
	}
	
	//noinspection SpellCheckingInspection
	/// Handle a GPU alloc event.
	///
	/// It is not possible to modify (mutate) the event object and it does not have a `result`.
	///
	/// Default implementation does nothing.
	#[allow(unused_variables)]
	#[inline(always)]
	fn use_cuda_gpu_free_event(&self, event: &MemoryGpuFreeEvent)
	{
	}
}
