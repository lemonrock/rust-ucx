// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A listener.
#[derive(Debug)]
pub struct ServerListener<L: ServerListenerAcceptHandler>
{
	handle: ucp_listener_h,
	worker_handle_drop_safety: Rc<WorkerHandleDropSafety>,
	server_listener_accept_handler: L,
}

impl<L: ServerListenerAcceptHandler> Drop for ServerListener<L>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if !self.handle.is_null()
		{
			unsafe { ucp_listener_destroy(self.handle) }
		}
	}
}

impl<L: ServerListenerAcceptHandler> ServerListener<L>
{
	#[inline(always)]
	unsafe extern "C" fn accept_callback(ep: ucp_ep_h, arg: *mut c_void)
	{
		debug_assert!(!arg.is_null(), "arg is null");
		
		let this_non_null = unsafe { NonNull::new_unchecked(arg) };
		this_non_null.as_ref().listener_accept_handler.accept(ep)
	}
}
