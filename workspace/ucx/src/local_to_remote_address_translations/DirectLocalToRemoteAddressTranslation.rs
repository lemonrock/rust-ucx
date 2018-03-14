// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// 1:1 with no offsets.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DirectLocalToRemoteAddressTranslation;

impl LocalToRemoteAddressTranslation for DirectLocalToRemoteAddressTranslation
{
	#[inline(always)]
	fn from_local_address_to_remote_address(&self, local_address: NonNull<u8>) -> u64
	{
		(local_address.as_ptr() as usize as u64)
	}
}
