// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Their remotely accessible memory.
///
/// Use this to perform remote memory load() and store(), and atomic operations such as fetch_and_add() and compare_and_swap().
#[derive(Debug)]
pub struct TheirRemotelyAccessibleMemory<E: EndPointPeerFailureErrorHandler, A: TheirRemotelyAccessibleEndPointAddress>
{
	handle: ucp_rkey_h,
	parent_end_point: Rc<RefCell<TheirRemotelyAccessibleEndPoint<E, A>>>,
}

impl<E: EndPointPeerFailureErrorHandler, A: TheirRemotelyAccessibleEndPointAddress> Drop for TheirRemotelyAccessibleMemory<E, A>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_rkey_destroy(self.handle) }
	}
}

impl<E: EndPointPeerFailureErrorHandler, A: TheirRemotelyAccessibleEndPointAddress> TheirRemotelyAccessibleMemory<E, A>
{
	/// Returns a local pointer which can be used for all atomic memory operations.
	///
	/// Will only work for `mmap`, `shmem`, `xpmem`, and `knmem` memory domains, ie memory on the same machine.
	#[inline(always)]
	pub fn local_pointer_if_remote_memory_is_shared_memory(&self, remote_address: *mut u8) -> *mut u8
	{
		let mut local_address = unsafe { uninitialized() };
		panic_on_error!(ucp_rkey_ptr, self.handle, remote_address as u64, &mut local_address);
		local_address as *mut u8
	}
}
