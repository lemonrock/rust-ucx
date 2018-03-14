// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A convenience to make it easier to focus on tagged message receiving.
///
/// Dereferences to a `Worker`.
#[derive(Clone)]
pub struct TaggedMessageReceivingWorker
{
	parent_worker: Worker,
}

impl Deref for TaggedMessageReceivingWorker
{
	type Target = Worker;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.parent_worker
	}
}

impl TaggedMessageReceivingWorker
{
	#[inline(always)]
	pub(crate) fn new(parent_worker: &Worker) -> Self
	{
		Self
		{
			parent_worker: parent_worker.clone(),
		}
	}
	
	///
	/// Never completes immediately.
	///
	/// The provided message is not safe to re-use, reading or writing until this request has completed; it should be thought of as `::std::mem::uninitialized()` memory.
	///
	/// For a `callback_when_finished_or_cancelled` that does nothing, use `::ucx::receive_callback_is_ignored`.
	/// `request` should not be freed inside the `callback_when_finished_or_cancelled`.
	///
	/// If a returned `SendingTaggedMessageNonBlockingRequest` is neither cancelled or completed (ie it falls out of scope) then the request will be cancelled and the `message` dropped.
	#[inline(always)]
	pub fn non_blocking_receive_tagged_message_ucx_allocated<'worker, M: Message>(&'worker self, message: M, tag_matcher: TagMatcher, callback_when_finished_or_cancelled: unsafe extern "C" fn(request: *mut c_void, status: ucs_status_t, info: *mut ucp_tag_recv_info_t)) -> Result<NonBlockingRequestCompletedOrInProgress<M, ReceivingTaggedMessageNonBlockingRequest<'worker, M>>, ErrorCodeWithMessage<M>>
	{
		let status_pointer = unsafe { ucp_tag_recv_nb(self.parent_worker.debug_assert_handle_is_valid(), message.address().as_ptr() as *mut u8 as *mut c_void, message.count(), message.data_type_descriptor(), tag_matcher.value.0, tag_matcher.bit_mask.0, Some(callback_when_finished_or_cancelled)) };
		
		match self.parent_worker.parse_status_pointer(status_pointer)
		{
			Ok(non_blocking_request_completed_or_in_progress) => match non_blocking_request_completed_or_in_progress
			{
				Completed(()) => Ok(Completed(message)),
				
				InProgress(non_blocking_request_in_progress) => Ok(InProgress(ReceivingTaggedMessageNonBlockingRequest::new(non_blocking_request_in_progress, message))),
			},
			
			Err(error_code) => Err(ErrorCodeWithMessage::new(error_code, message))
		}
	}
	
	/*
	
	#[link_name = "\u{1}_ucp_tag_msg_recv_nb"] pub fn ucp_tag_msg_recv_nb(worker: ucp_worker_h, buffer: *mut c_void, count: usize, datatype: ucp_datatype_t, message: ucp_tag_message_h, cb: ucp_tag_recv_callback_t) -> ucs_status_ptr_t;
	
	
	#[link_name = "\u{1}_ucp_tag_recv_nbr"] pub fn ucp_tag_recv_nbr(worker: ucp_worker_h, buffer: *mut c_void, count: usize, datatype: ucp_datatype_t, tag: ucp_tag_t, tag_mask: ucp_tag_t, req: *mut c_void) -> ucs_status_t;
	
	*/
	
	
	
	
	/// This routine checks for a message which has been partially or fully received and matches according to `tag_matcher`.
	///
	/// It does not wait for a message to be present; returns immediately.
	///
	/// It does not remove a matching message from the queue of received messages.
	///
	/// Call `self.progress()` occasionally; this function does not do any polling of the network.
	#[inline(always)]
	pub fn peek(&self, tag_matcher: TagMatcher) -> Option<ReceivedTaggedMessageInformation>
	{
		self.probe(tag_matcher, false.to_c_bool()).map(|(received_tagged_message_information, _message_handle)| received_tagged_message_information)
	}
	
	/// This routine pops a message which has been partially or fully received and matches according to `tag_matcher`.
	///
	/// It does not wait for a message to be present; returns immediately.
	///
	/// It removes a matching message from the queue of received messages.
	///
	/// For a `callback_when_finished_or_cancelled` that does nothing, use `::ucx::receive_callback_is_ignored`.
	/// `request` should not be freed inside the `callback_when_finished_or_cancelled`.
	///
	/// Call `self.progress()` occasionally; this function does not do any polling of the network.
	#[inline(always)]
	pub fn pop<'worker, MP: MessageProvider>(&'worker self, tag_matcher: TagMatcher, message_provider: &mut MP, callback_when_finished_or_cancelled: unsafe extern "C" fn(request: *mut c_void, status: ucs_status_t, info: *mut ucp_tag_recv_info_t)) -> Option<Result<NonBlockingRequestCompletedOrInProgress<MP::M, ReceivingTaggedMessageNonBlockingRequest<'worker, MP::M>>, ErrorCodeWithMessage<MP::M>>>
	{
		match self.probe(tag_matcher, true.to_c_bool())
		{
			None => None,
			
			Some((received_tagged_message_information, message_handle)) =>
			{
				let message = message_provider.provide_uninitialized_message(received_tagged_message_information);
				
				let status_pointer = unsafe { ucp_tag_msg_recv_nb(self.parent_worker.debug_assert_handle_is_valid(), message.address().as_ptr() as *mut u8 as *mut c_void, message.count(), message.data_type_descriptor(), message_handle.as_ptr(), Some(callback_when_finished_or_cancelled)) };
				
				let popped = match self.parent_worker.parse_status_pointer(status_pointer)
				{
					Ok(non_blocking_request_completed_or_in_progress) => match non_blocking_request_completed_or_in_progress
					{
						Completed(()) => Ok(Completed(message)),
						
						InProgress(non_blocking_request_in_progress) => Ok(InProgress(ReceivingTaggedMessageNonBlockingRequest::new(non_blocking_request_in_progress, message))),
					},
					
					Err(error_code) => Err(ErrorCodeWithMessage::new(error_code, message))
				};
				
				Some(popped)
			}
		}
	}
	
	#[inline(always)]
	fn probe(&self, tag_matcher: TagMatcher, remove_c_bool: i32) -> Option<(ReceivedTaggedMessageInformation, NonNull<ucp_recv_desc>)>
	{
		let mut received_tagged_message_information: ReceivedTaggedMessageInformation = unsafe { uninitialized() };
		let message_handle = unsafe { ucp_tag_probe_nb(self.parent_worker.debug_assert_handle_is_valid(), tag_matcher.value.0, tag_matcher.bit_mask.0, remove_c_bool, &mut received_tagged_message_information.0) };
		
		if message_handle.is_null()
		{
			None
		}
		else
		{
			Some((received_tagged_message_information, unsafe { NonNull::new_unchecked(message_handle) }))
		}
	}
}
