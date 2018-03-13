// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A non-blocking request.
pub trait NonBlockingRequest: Sized
{
	/// Block until a non-blocking operation is complete.
	///
	/// Useful when the UCX API exposes non-blocking operations, but an application needs to wait for them to finish.
	#[inline(always)]
	fn subsequently_block_until_non_blocking_request_is_complete(self, parent_worker: &Worker) -> Result<(), ErrorCode>
	{
		while
		{
			parent_worker.progress();
			
			self.is_still_in_progress()?
		}
		{
		}
		
		Ok(())
	}
	
	/// Check if the request is still in progress.
	///
	/// An Ok(true) means is completed successfully,
	/// An Ok(false) means it is still in progress.
	/// An Err() means it completed with an error.
	#[inline(always)]
	fn is_still_in_progress(&self) -> Result<bool, ErrorCode>
	{
		let status = unsafe { ucp_request_check_status(self.non_null_pointer().as_ptr() as *mut c_void) };
		
		match status.parse()
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
	fn is_still_in_progress_for_tag_receive(&self) -> Result<(bool, ucp_tag_recv_info_t), ErrorCode>
	{
		let mut tag_receive_information = unsafe { uninitialized() };
		
		let status = unsafe { ucp_tag_recv_request_test(self.non_null_pointer().as_ptr() as *mut c_void, &mut tag_receive_information) };
		
		match status.parse()
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
	fn is_still_in_progress_for_stream(&self) -> Result<Option<usize>, ErrorCode>
	{
		let mut length = unsafe { uninitialized() };
		
		let status = unsafe { ucp_stream_recv_request_test(self.non_null_pointer().as_ptr() as *mut c_void, &mut length) };
		
		match status.parse()
		{
			IsOk => Ok(Some(length)),
			
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
	fn cancel(self, parent_worker: &Worker)
	{
		unsafe { ucp_request_cancel(parent_worker.handle, self.non_null_pointer().as_ptr() as *mut c_void) };
		drop(self)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn non_null_pointer(&self) -> NonNull<u8>;
}
