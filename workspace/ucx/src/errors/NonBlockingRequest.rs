// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A wrapper around requests to make them easier to work with.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NonBlockingRequest(NonNull<u8>);

impl Drop for NonBlockingRequest
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_request_free(self.pointer()) };
	}
}

impl NonBlockingRequest
{
	#[inline(always)]
	fn block_until_non_blocking_operation_is_complete(parent_worker: &Worker, status_pointer: ucs_status_ptr_t) -> Result<(), ErrorCode>
	{
		use self::StatusOrNonBlockingRequest::*;
		
		match status_pointer.parse().unwrap("Invalid status_pointer")
		{
			Status(IsOk) => Ok(()),
			
			NonBlockingRequest(non_blocking_request) =>
			{
				while
				{
					parent_worker.progress();
					
					non_blocking_request.is_still_in_progress()?
				}
				{
				}
				
				drop(non_blocking_request);
				Ok(())
			},
			
			Status(ErrorCode(error_code)) => Err(error_code),
			
			unexpected @ _ => panic!("Unexpected status_pointer: {:?}", unexpected),
		}
	}
	
	/// Check if the request is still in progress.
	///
	/// An Ok(true) means is completed successfully,
	/// An Ok(false) means it is still in progress.
	/// An Err() means it completed with an error.
	#[inline(always)]
	pub fn is_still_in_progress(&self) -> Result<bool, ErrorCode>
	{
		let status = unsafe { ucp_request_check(self.pointer()) };
		
		use self::Status::*;
		
		match status.parse().expect("Invalid status")
		{
			IsOk => Ok(false),
			
			OperationInProgress => Ok(true),
			
			Error(error_code) => Err(error_code),
			
			unexpected @ _ => panic!("status '{:?}' was unexpected", unexpected),
		}
	}
	
	/// Check if the request is still in progress when receiving tag messages.
	/// Use this after calling `ucp_tag_recv_nb` or `ucp_tag_recv_nbr`.
	///
	/// An Ok(true, tag_receive_information) means is completed successfully,
	/// An Ok(false, tag_receive_information) means it is still in progress.
	/// An Err() means it completed with an error.
	#[inline(always)]
	pub fn is_still_in_progress_for_tag_receive(&self) -> Result<(bool, ucp_tag_recv_info_t), ErrorCode>
	{
		let mut tag_receive_information = unsafe { unintialized() };
		
		let status = unsafe { ucp_tag_recv_request_test(self.pointer(), &mut tag_receive_information) };
		
		match status.parse().expect("Invalid status")
		{
			IsOk => Ok((false, tag_receive_information)),
			
			OperationInProgress => Ok((true, tag_receive_information)),
			
			Error(error_code) => Err(error_code),
			
			unexpected @ _ => panic!("status '{:?}' was unexpected", unexpected),
		}
	}
	
	/// Check if the request is still in progress when receiving tag messages.
	/// Use this after calling `ucp_stream_recv_nb`.
	///
	/// An Ok(Some(received_data_in_bytes)) means is completed successfully,
	/// An Ok(None) means it is still in progress.
	/// An Err() means it completed with an error.
	#[inline(always)]
	pub fn is_still_in_progress_for_stream(&self) -> Result<Option<usize>, ErrorCode>
	{
		let mut length = unsafe { unintialized() };
		
		let status = unsafe { ucp_tag_recv_request_test(self.pointer(), &mut length) };
		
		match status.parse().expect("Invalid status")
		{
			IsOk => Ok(length),
			
			OperationInProgress => Ok(None),
			
			Error(error_code) => Err(error_code),
			
			unexpected @ _ => panic!("status '{:?}' was unexpected", unexpected),
		}
	}
	
	/// Cancel an outstanding communications request.
	///
	/// This routine tries to cancels an outstanding communication request.
	/// After calling this routine, the request will be in completed or canceled (but not both) state regardless of the status of the target endpoint associated with the communication request.
	/// If the request is completed successfully, the `ucp_send_callback_t` "send" or `ucp_tag_recv_callback_t` "receive" completion callbacks (based on the type of the request) will be called with the `status` argument of the callback set to `UCS_OK`, and in a case it is cancelled the status argument is set to `UCS_ERR_CANCELED`.
	#[inline(always)]
	pub fn cancel(self, parent_worker: &Worker)
	{
		unsafe { ucp_request_cancel(parent_worker.handle) };
		drop(self)
	}
	
	#[inline(always)]
	fn pointer(&self) -> *mut c_void
	{
		self.0.as_ptr() as *mut c_void
	}
}
