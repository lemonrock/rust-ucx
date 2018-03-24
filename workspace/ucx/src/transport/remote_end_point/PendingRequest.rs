// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Pending requests are passed to the remote end point and executed when possible.
#[derive(Debug)]
pub struct PendingRequest<P: PendingRequestHandler>
{
	pending_request_handler: P,
	call_when_purged_which_must_be_penultimate_field_in_struct: PendingRequestCallWhenPurged,
	uct_required_field_which_must_be_last_field_in_struct: uct_pending_req,
}

impl<P: PendingRequestHandler> PendingRequest<P>
{
	/// A new pending request.
	#[inline(always)]
	pub fn new(pending_request_handler: P) -> Box<Self>
	{
		Box::new
		(
			Self
			{
				pending_request_handler,
				call_when_purged_which_must_be_penultimate_field_in_struct: Self::call_when_purged,
				uct_required_field_which_must_be_last_field_in_struct: uct_pending_req
				{
					func: Self::pending_callback,
					priv_: unsafe { zeroed() },
				}
			}
		)
	}
	
	unsafe extern "C" fn pending_callback(raw_pointer: *mut uct_pending_req) -> ucs_status_t
	{
		debug_assert!(!raw_pointer.is_null(), "raw_pointer is null");
		
		let this = Self::from_raw_pointer(raw_pointer);
		match this.pending_request_handler.invoke()
		{
			Ok(()) => ucs_status_t::UCS_OK,
			
			Err(error_code) => error_code.to_ucs_status_t()
		}
	}
	
	unsafe extern "C" fn call_when_purged(raw_pointer: *mut uct_pending_req)
	{
		debug_assert!(!raw_pointer.is_null(), "raw_pointer is null");
		
		let this = Self::from_raw_pointer(raw_pointer);
		this.pending_request_handler.purged();
	}
	
	#[inline(always)]
	pub(crate) fn to_raw_pointer(pending_request: Box<Self>) -> *mut uct_pending_req
	{
		let raw = Box::into_raw(pending_request) as *mut u8;
		unsafe { raw.offset(Self::offset_of_uct_required_field_which_must_be_last_field_in_struct()) as *mut uct_pending_req }
	}
	
	#[inline(always)]
	pub(crate) fn from_raw_pointer(raw_pointer: *mut uct_pending_req) -> Box<Self>
	{
		debug_assert!(!raw_pointer.is_null(), "raw_pointer is null");
		unsafe
		{
			let raw = (raw_pointer as *mut u8).offset(-Self::offset_of_uct_required_field_which_must_be_last_field_in_struct());
			Box::from_raw(raw as *mut Self)
		}
	}
	
	#[inline(always)]
	pub(crate) unsafe fn pending_request_purged(raw_pointer: *mut uct_pending_req)
	{
		let raw = raw_pointer as *mut u8;
		let raw_call_when_purged_function_pointer = raw.offset(-Self::offset_of_call_when_purged_which_must_be_penultimate_field_in_struct());
		let function_pointer: PendingRequestCallWhenPurged = transmute(raw_call_when_purged_function_pointer);
		function_pointer(raw_pointer)
	}
	
	#[inline(always)]
	fn offset_of_call_when_purged_which_must_be_penultimate_field_in_struct() -> isize
	{
		offset_of!(Self, call_when_purged_which_must_be_penultimate_field_in_struct) as isize
	}
	
	#[inline(always)]
	fn offset_of_uct_required_field_which_must_be_last_field_in_struct() -> isize
	{
		offset_of!(Self, uct_required_field_which_must_be_last_field_in_struct) as isize
	}
}
