// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Aggregated memory functions which trigger event handlers inside UCM / UCX.
pub trait AggregatedMemoryFunctionsWhichTriggerEventHandlers
{
	/// Call the handlers registered for aggregated VM_MMAP event.
	#[inline(always)]
	unsafe fn virtual_machine_mmap(self, length: size_t);
	
	//noinspection SpellCheckingInspection
	/// Call the handlers registered for aggregated VM_MUNMAP event.
	#[inline(always)]
	unsafe fn virtual_machine_munmap(self, length: size_t);
}

impl AggregatedMemoryFunctionsWhichTriggerEventHandlers for *mut c_void
{
	#[inline(always)]
	unsafe fn virtual_machine_mmap(self, length: size_t)
	{
		ucm_vm_mmap(self, length)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	unsafe fn virtual_machine_munmap(self, length: size_t)
	{
		ucm_vm_munmap(self, length)
	}
}
