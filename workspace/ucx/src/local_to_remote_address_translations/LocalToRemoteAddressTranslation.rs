// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// How to translate from local addresses to remote addresses for remotely accessible memory.
pub trait LocalToRemoteAddressTranslation
{
	/// Convert from a local address to a remote address.
	#[inline(always)]
	fn from_local_address_to_remote_address(&self, local_address: NonNull<u8>) -> u64;
}
