// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An end point.
/// *MUST* be used inside a `Rc`.
pub struct EndPoint<E: EndPointPeerFailureErrorHandler>
{
	handle: ucp_ep_h,
	user_data_and_peer_failure_error_handler: E,
	parent_worker: Worker,
	end_point_parameters: ucp_ep_params_t,
}

impl<E: EndPointPeerFailureErrorHandler> Drop for EndPoint<E>
{
	// Dropping because there are no more Rc strong references.
	#[inline(always)]
	fn drop(&mut self)
	{
		#[inline(always)]
		fn drop_user_data<E: EndPointPeerFailureErrorHandler>(user_data: *mut c_void)
		{
			let weak: Weak<EndPoint<E>> = unsafe { transmute(user_data) };
			drop(weak);
		}
		
		// Never properly initialized.
		if self.handle.is_null()
		{
			let user_data = self.end_point_parameters.user_data;
			if !user_data.is_null()
			{
				drop_user_data(user_data)
			}
		}
		// Initialized and in-use.
		else
		{
			// We need to modify the end-point, and remove the user data (ie set it to null)
			// So any callbacks from UCX now fail.
			
			use self::StatusOrNonBlockingRequest::*;
			use self::Status::*;
			
			// Modify the end point to release the user_data and error_handler so we can free them.
			let user_data_original = self.end_point_parameters.user_data;
			self.end_point_parameters.user_data = null_mut();
			self.end_point_parameters.err_handler = None;
			let change_user_data_status_pointer = unsafe { ucp_ep_modify_nb(self.handle, &self.end_point_parameters) };
			// We discard any errors; there's nothing we can do with them.
			self.parent_worker.block_until_non_blocking_operation_is_complete(change_user_data_status_pointer);
			
			// Drop the weak reference in user data.
			drop_user_data(user_data_original);
			
			let close_status_pointer = unsafe { ucp_ep_close_nb(self.handle, ucp_ep_close_mode::UCP_EP_CLOSE_MODE_FLUSH) };
			// We discard any errors; there's nothing we can do with them.
			self.parent_worker.block_until_non_blocking_operation_is_complete(close_status_pointer);
			
			self.handle = null_mut();
		}
	}
}

impl<E: EndPointPeerFailureErrorHandler> Debug for EndPoint<E>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		self.debug_fmt(f)
	}
}

impl<E: EndPointPeerFailureErrorHandler> PrintInformation for EndPoint<E>
{
	const DebugName: &'static str = "EndPoint";
	
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		unsafe { ucp_ep_print_info(self.handle, stream) };
	}
}

impl<E: EndPointPeerFailureErrorHandler> EndPoint<E>
{
	
	// ucp_tag_recv_nb()
	
	// ucp_tag_*()
	
	// ucp_ep_flush_nb
	
	// ucp_stream_data_release
	
	
	
	
	
	#[inline(always)]
	pub(crate) fn new_end_point(peer_failure_error_handler: E, their_remote_address: TheirRemoteAddress, guarantee_that_send_requests_are_always_completed_successfully_or_error: bool, parent_worker: &Worker) -> Rc<RefCell<Self>>
	{
		#[inline(always)]
		fn populated_by_their_remote_address<T>() -> T
		{
			unsafe { zeroed() }
		}
		
		use self::ucp_err_handling_mode_t::*;
		
		let end_point = Rc::new
		(
			RefCell::new
			(
				Self
				{
					handle: null_mut(),
					user_data_and_peer_failure_error_handler: peer_failure_error_handler,
					parent_worker: parent_worker.clone(),
					end_point_parameters: their_remote_address.populate_end_point_parameters
					(
						ucp_ep_params_t
						{
							field_mask: ucp_ep_params_field::ERR_HANDLING_MODE | ucp_ep_params_field::ERR_HANDLER | ucp_ep_params_field::USER_DATA,
							address: populated_by_their_remote_address(),
							err_mode: if guarantee_that_send_requests_are_always_completed_successfully_or_error
							{
								UCP_ERR_HANDLING_MODE_PEER
							}
							else
							{
								UCP_ERR_HANDLING_MODE_NONE
							},
							err_handler: ucp_err_handler
							{
								cb: Some(EndPoint::peer_failure_error_callback),
								arg: null_mut(), // Is overridden by `ucp_ep_params_t.user_data`.
							},
							user_data: null_mut(),
							
							flags: populated_by_their_remote_address(),
							sockaddr: populated_by_their_remote_address(),
						}
					),
				}
			)
		);
		Self::assign_user_data_to_self(&end_point);
		
		end_point.connectOrReconnect(destinationAddress);
		
		end_point
	}
	
	// Yes, this is horrible, but how else does one pack a Weak<EndPoint<E>> into a C FFI `user_data` field of type void*?
	// (Actually, by possibly using a user_data = Box<Weak<EndPoint<E>>>::into_raw()... but that involves indirection).
	#[inline(always)]
	pub(crate) fn assign_user_data_to_self(this: &Rc<RefCell<Self>>)
	{
		let borrow = this.borrow_mut();
		end_point.end_pointer_parameters.user_data = unsafe { transmute(Rc::downgrade(this)) };
	}
	
	// Yes, this is also horrible.
	// `user_data` is an aliased value - there can be multiple copies for one logical `Weak<Self>`.
	#[inline(always)]
	pub(crate) fn end_point_from_user_data(user_data: *mut c_void, handle: ucp_ep_h) -> Option<Rc<RefCell<Self>>>
	{
		// This can only happen during the drop of the end point (we tell UCX to modify the end point and give it null user_data).
		if user_data.is_null()
		{
			return None;
		}
		
		let weak: Weak<Self> = unsafe { transmute(user_data) };
		let possibly_strong = weak.upgrade();
		forget(weak);
		
		if let Some(strong) = possibly_strong
		{
			// Either not yet initialized (so no errors should have been raised).
			// Or has been dropped but not freed.
			let our_handle = strong.borrow().handle;
			if our_handle.is_null()
			{
				None
			}
			else
			{
				debug_assert!(handle == our_handle);
				Some(strong)
			}
		}
		else
		{
			None
		}
	}
	
	// Yes, this is another horrible piece of code.
	#[inline(always)]
	unsafe extern "C" fn peer_failure_error_callback(user_data: *mut c_void, ep: ucp_ep_h, status: ucs_status_t)
	{
		if let Some(this) = Self::end_point_from_user_data(user_data, ep)
		{
			this.borrow_mut().user_data_and_peer_failure_error_handler.peer_failure(status.error_code_or_panic())
		}
	}
}
