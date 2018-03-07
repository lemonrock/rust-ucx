// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A `OurRemotelyAccessibleMemoryKey` is an opaque piece of data that needs to be sent to other machines so that they can uniquely connect to our remotely accessible memory.
///
/// In Rust terms, it's a bit like a `Vec<u8>`.
#[derive(Debug)]
pub struct OurRemotelyAccessibleMemoryKey
{
	address: NonNull<u8>,
	length: usize,
	our_remotely_accessible_memory_handle_drop_safety: Rc<OurRemotelyAccessibleMemoryHandleDropSafety>
}

impl Drop for OurRemotelyAccessibleMemoryKey
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_rkey_buffer_release(self.address.as_ptr() as *mut c_void) }
	}
}

impl ByteBuffer for OurRemotelyAccessibleMemoryKey
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
