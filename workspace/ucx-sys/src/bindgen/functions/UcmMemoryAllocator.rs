// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ucm_mmap"] pub fn ucm_mmap(addr: *mut c_void, length: usize, prot: c_int, flags: c_int, fd: c_int, offset: off_t) -> *mut c_void;
	#[link_name = "\u{1}_ucm_mremap"] pub fn ucm_mremap(old_address: *mut c_void, old_size: usize, new_size: usize, flags: c_int) -> *mut c_void;
	#[link_name = "\u{1}_ucm_munmap"] pub fn ucm_munmap(addr: *mut c_void, length: usize) -> c_int;
	#[link_name = "\u{1}_ucm_sbrk"] pub fn ucm_sbrk(increment: isize) -> *mut c_void;
	#[link_name = "\u{1}_ucm_shmat"] pub fn ucm_shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
	#[link_name = "\u{1}_ucm_shmdt"] pub fn ucm_shmdt(shmaddr: *const c_void) -> c_int;
	#[link_name = "\u{1}_ucm_vm_mmap"] pub fn ucm_vm_mmap(addr: *mut c_void, length: usize);
	#[link_name = "\u{1}_ucm_vm_munmap"] pub fn ucm_vm_munmap(addr: *mut c_void, length: usize);
}
