// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A server connection request.
pub trait ServerConnectionRequest
{
	/// Process this connection request to the server.
	/// Return true if successful.
	/// This callback routine will be invoked on the server side upon receiving an incoming connection request.
	///
	/// This callback has to be thread safe if callback flags `ASYNC` were used to establish it.
	///
	/// Other than communication progress routines, it is allowed to call other UCT communication routines from this callback.
	#[inline(always)]
	fn connection_request(&self, connection_private_data_from_uct_ep_create_sockaddr: UcxAllocatedByteBuffer) -> bool;
	
	/// SYNC, ASYNC or both?
	#[inline(always)]
	fn connection_callback_flags(&self) -> uct_cb_flags;
}
