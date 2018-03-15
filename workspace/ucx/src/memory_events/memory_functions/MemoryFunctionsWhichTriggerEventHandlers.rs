// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Memory functions which trigger event handlers inside UCM / UCX.
pub trait MemoryFunctionsWhichTriggerEventHandlers
{
	//noinspection SpellCheckingInspection
	/// mmap which triggers event handlers.
	#[inline(always)]
	unsafe fn mmap_which_triggers_event_handlers(self, length: size_t, prot: c_int, flags: c_int, fd: RawFd, offset: off_t) -> Self;
	
	//noinspection SpellCheckingInspection
	/// munmap which triggers event handlers.
	unsafe fn munmap_which_triggers_event_handlers(self, length: size_t) -> c_int;
	
	//noinspection SpellCheckingInspection
	/// mremap which triggers event handlers.
	unsafe fn mremap_which_triggers_event_handlers(self, old_size: size_t, new_size: size_t, flags: c_int) -> Self;
	
	//noinspection SpellCheckingInspection
	/// shmat which triggers event handlers (argument order differs; `shmaddr` is `self`).
	#[inline(always)]
	unsafe fn shmat_which_triggers_event_handlers(self, shmid: c_int, shmflg: c_int) -> Self;
	
	//noinspection SpellCheckingInspection
	/// shmdt which triggers event handlers.
	#[inline(always)]
	unsafe fn shmdt_which_triggers_event_handlers(self) -> c_int;
	
	//noinspection SpellCheckingInspection
	/// sbrk which triggers event handlers.
	#[inline(always)]
	unsafe fn sbrk_which_triggers_event_handlers(increment: intptr_t) -> Self;
}

impl MemoryFunctionsWhichTriggerEventHandlers for *mut c_void
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	unsafe fn mmap_which_triggers_event_handlers(self, length: size_t, prot: c_int, flags: c_int, fd: RawFd, offset: off_t) -> Self
	{
		ucm_mmap(self, length, prot, flags, fd, offset)
	}
	
	//noinspection SpellCheckingInspection
	unsafe fn munmap_which_triggers_event_handlers(self, length: size_t) -> c_int
	{
		ucm_munmap(self, length)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	unsafe fn mremap_which_triggers_event_handlers(self, old_size: size_t, new_size: size_t, flags: c_int) -> Self
	{
		ucm_mremap(self, old_size, new_size, flags)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	unsafe fn shmat_which_triggers_event_handlers(self, shmid: c_int, shmflg: c_int) -> Self
	{
		ucm_shmat(shmid, self, shmflg)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	unsafe fn shmdt_which_triggers_event_handlers(self) -> c_int
	{
		ucm_shmdt(self)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	unsafe fn sbrk_which_triggers_event_handlers(increment: intptr_t) -> Self
	{
		ucm_sbrk(increment)
	}
}
