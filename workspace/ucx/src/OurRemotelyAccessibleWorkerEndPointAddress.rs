// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// This address can be passed to remote instances of the UCP library in order to to connect to this worker.
#[derive(Debug)]
pub struct OurRemotelyAccessibleWorkerEndPointAddress
{
	address: NonNull<u8>,
	length: usize,
	worker_handle: ucp_worker_h,
	worker_handle_drop_safety: Rc<WorkerHandleDropSafety>,
}

impl Drop for OurRemotelyAccessibleWorkerEndPointAddress
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_worker_release_address(self.worker_handle, self.address.as_ptr() as *mut _) }
	}
}

impl ByteBuffer for OurRemotelyAccessibleWorkerEndPointAddress
{
	#[inline(always)]
	fn address(&self) -> NonNull<u8>
	{
		self.address
	}
	
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.length
	}
}
