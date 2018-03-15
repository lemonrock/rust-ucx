// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// This trait provides access to the original implementations of `mmap` and friends which do not trigger memory events inside UCM / UCX.
pub trait MemoryFunctionsWhichDoNotTriggerEvents
{
	//noinspection SpellCheckingInspection
	/// mmap
	#[inline(always)]
	unsafe fn mmap(self, length: size_t, prot: c_int, flags: c_int, fd: RawFd, offset: off_t) -> Self;
	
	//noinspection SpellCheckingInspection
	/// munmap
	#[inline(always)]
	unsafe fn munmap(self, length: size_t) -> c_int;
	
	//noinspection SpellCheckingInspection
	/// mremap
	#[inline(always)]
	unsafe fn mremap(self, old_size: size_t, new_size: size_t, flags: c_int) -> Self;
	
	//noinspection SpellCheckingInspection
	/// shmat (argument order differs; `shmaddr` is `self`).
	#[inline(always)]
	unsafe fn shmat(self, shmid: c_int, shmflg: c_int) -> Self;
	
	//noinspection SpellCheckingInspection
	/// shmdt
	#[inline(always)]
	unsafe fn shmdt(self) -> c_int;
	
	//noinspection SpellCheckingInspection
	/// sbrk
	#[inline(always)]
	unsafe fn sbrk(increment: intptr_t) -> Self;
}

impl MemoryFunctionsWhichDoNotTriggerEvents for *mut c_void
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	unsafe fn mmap(self, length: size_t, prot: c_int, flags: c_int, fd: RawFd, offset: off_t) -> Self
	{
		ucm_orig_mmap(self, length, prot, flags, fd, offset)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	unsafe fn munmap(self, length: size_t) -> c_int
	{
		ucm_orig_munmap(self, length)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	unsafe fn mremap(self, old_size: size_t, new_size: size_t, flags: c_int) -> Self
	{
		ucm_orig_mremap(self, old_size, new_size, flags)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	unsafe fn shmat(self, shmid: c_int, shmflg: c_int) -> Self
	{
		ucm_orig_shmat(shmid, self, shmflg)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	unsafe fn shmdt(self) -> c_int
	{
		ucm_orig_shmdt(self)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	unsafe fn sbrk(increment: intptr_t) -> Self
	{
		ucm_orig_sbrk(increment)
	}
}
