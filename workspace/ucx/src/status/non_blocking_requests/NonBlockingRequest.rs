// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A non-blocking request.
pub trait NonBlockingRequest: Sized
{
	#[doc(hidden)]
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
	
	#[doc(hidden)]
	#[inline(always)]
	fn subsequently_block_until_non_blocking_request_is_complete_for_tagged_message_receive(self, parent_worker: &Worker) -> Result<ReceivedTaggedMessageInformation, ErrorCode>
	{
		loop
		{
			parent_worker.progress();
			
			if let Some(received_tagged_message_information) = self.is_still_in_progress_for_tagged_message_receive()?
			{
				return Ok(received_tagged_message_information)
			}
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn subsequently_block_until_non_blocking_request_is_complete_for_stream_receive(self, parent_worker: &Worker) -> Result<StreamLengthOfReceivedDataInBytes, ErrorCode>
	{
		loop
		{
			parent_worker.progress();
			
			if let Some(length) = self.is_still_in_progress_for_stream_receive()?
			{
				return Ok(length)
			}
		}
	}
	
	#[doc(hidden)]
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
	
	#[doc(hidden)]
	#[inline(always)]
	fn is_still_in_progress_for_tagged_message_receive(&self) -> Result<Option<ReceivedTaggedMessageInformation>, ErrorCode>
	{
		let mut tag_receive_information = unsafe { uninitialized() };
		
		// tag_receive_information is only populated if status is not UCS_INPROGRESS.
		let status = unsafe { ucp_tag_recv_request_test(self.non_null_pointer().as_ptr() as *mut c_void, &mut tag_receive_information) };
		
		match status.parse()
		{
			IsOk => Ok(Some(ReceivedTaggedMessageInformation(tag_receive_information))),
			
			OperationInProgress => Ok(None),
			
			Error(error_code) => Err(error_code),
			
			unexpected @ _ => panic!("status '{:?}' was unexpected", unexpected),
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn is_still_in_progress_for_stream_receive(&self) -> Result<Option<StreamLengthOfReceivedDataInBytes>, ErrorCode>
	{
		let mut length = unsafe { uninitialized() };
		
		// length is only populated if status is not UCS_INPROGRESS.
		let status = unsafe { ucp_stream_recv_request_test(self.non_null_pointer().as_ptr() as *mut c_void, &mut length) };
		
		match status.parse()
		{
			IsOk => Ok(Some(length)),
			
			OperationInProgress => Ok(None),
			
			Error(error_code) => Err(error_code),
			
			unexpected @ _ => panic!("status '{:?}' was unexpected", unexpected),
		}
	}
	
	#[doc(hidden)]
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
