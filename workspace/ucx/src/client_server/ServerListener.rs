// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A listener.
#[derive(Debug)]
pub struct ServerListener<L: ServerListenerAcceptHandler>
{
	handle: ucp_listener_h,
	worker_handle_drop_safety: Rc<WorkerHandleDropSafety>,
	our_remotely_accessible_server_end_point_address: OurRemotelyAccessibleServerEndPointAddress, // We *MUST* hold a reference to this, otherwise the `params` data used to construct a Server contains raw pointers to socket address structures that may have been dropped.
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
	/// A server listener listens for incoming client connections on a particular address.
	/// It then creates ?end points? to handle them.
	#[inline(always)]
	pub(crate) fn create_server_listener(our_remotely_accessible_server_end_point_address: OurRemotelyAccessibleServerEndPointAddress, server_listener_accept_handler: L, worker_handle_drop_safety: &Rc<WorkerHandleDropSafety>, worker_handle: ucp_worker_h) -> Result<Box<Self>, ErrorCode>
	{
		let mut server_listener = Box::new
		(
			Self
			{
				handle: null_mut(),
				worker_handle_drop_safety: worker_handle_drop_safety.clone(),
				our_remotely_accessible_server_end_point_address,
				server_listener_accept_handler,
			}
		);
		
		let (socket_address, length) = server_listener.our_remotely_accessible_server_end_point_address.suitable_for_ffi();
		
		let parameters = ucp_listener_params_t
		{
			field_mask: (ucp_listener_params_field::SOCK_ADDR | ucp_listener_params_field::ACCEPT_HANDLER).0 as u64,
			sockaddr: ucs_sock_addr_t
			{
				addr: socket_address,
				addrlen: length,
			},
			accept_handler: ucp_listener_accept_handler_t
			{
				cb: Some(Self::accept_callback),
				arg: (&server_listener.server_listener_accept_handler) as *const _ as *mut _,
			},
		};
		
		let status = unsafe { ucp_listener_create(worker_handle, &parameters, &mut server_listener.handle) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(server_listener),
			
			Error(error_code) => Err(error_code),
			
			unexpected @ _ => panic!("Unexpected status '{:?}'", unexpected)
		}
	}
	
	#[inline(always)]
	unsafe extern "C" fn accept_callback(end_point_handle: ucp_ep_h, accept_handler_arg: *mut c_void)
	{
		debug_assert!(!accept_handler_arg.is_null(), "accept_handler_arg is null");
		
		let this_non_null = NonNull::new_unchecked(accept_handler_arg as *mut L);
		this_non_null.as_ref().accept(end_point_handle)
	}
}
