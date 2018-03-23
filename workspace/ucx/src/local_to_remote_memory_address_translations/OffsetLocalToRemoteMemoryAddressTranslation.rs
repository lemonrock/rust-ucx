// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Applies an offset.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OffsetLocalToRemoteMemoryAddressTranslation
{
	offset: i64,
}

impl LocalToRemoteMemoryAddressTranslation for OffsetLocalToRemoteMemoryAddressTranslation
{
	#[inline(always)]
	fn from_local_address_to_remote_memory_address(&self, local_memory_address: NonNull<u8>) -> RemoteMemoryAddress
	{
		let pointer_as_usize = local_memory_address.as_ptr() as usize;
		debug_assert!(pointer_as_usize < ::std::i64::MAX as usize, "pointer is too high");
		RemoteMemoryAddress(((pointer_as_usize as i64) + self.offset) as u64)
	}
}
