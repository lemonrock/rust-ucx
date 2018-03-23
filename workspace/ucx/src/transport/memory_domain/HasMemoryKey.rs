// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A MemoryRegion or MemoryRegistration
pub trait HasMemoryKey: ByteBuffer
{
	/// Obtains a packed memory key suitable for sharing with other peers.
	#[inline(always)]
	fn packed_memory_key(&self) -> &[u8];
	
	/// Creates a valid IoVector.
	///
	/// Sadly, the resultant IoVector references this memory region or registration unsafely, and so `self` must be kept alive by the caller for as long as ZeroCopyIoVector is alive.
	#[inline(always)]
	fn zero_copy_io_vector(&self, offset: usize, length: usize) -> ZeroCopyIoVector;
}
