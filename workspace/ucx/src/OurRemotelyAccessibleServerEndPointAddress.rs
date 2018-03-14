// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A socket addres which we listen on as a server for inbound (client) connection requests.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct OurRemotelyAccessibleServerEndPointAddress(SocketAddress);

impl OurRemotelyAccessibleServerEndPointAddress
{
	/// A reference suitable for FFI.
	/// Be very careful - `&self` needs to live as long as `*const sockaddr` is in use for.
	#[inline(always)]
	pub(crate) fn suitable_for_ffi(&self) -> (*const sockaddr, socklen_t)
	{
		self.0.suitable_for_ffi()
	}
}
