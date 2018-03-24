// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Allows cancellation of the sending of a tagged message using the rendezvous protocol and zero-copy.
#[derive(Debug)]
pub struct RendezvousCancellation<'remote_end_point, 'completion, C: 'completion + CompletionHandler>
{
	cancellation_handle: *mut c_void,
	remote_end_point: &'remote_end_point RemoteEndPoint,
	completion: &'completion Completion<C>,
}

impl<'remote_end_point, 'completion, C: 'completion + CompletionHandler> RendezvousCancellation<'remote_end_point, 'completion, C>
{
	/// Cancels a completion.
	///
	/// Returns `true` if successful and `false` if not.
	/// Does not call the completion handle and so MAY RESULT IN A MEMORY LEAK.
	#[inline(always)]
	pub fn cancel(self) -> bool
	{
		if self.completion.callback_already_invoked_by_uct()
		{
			false
		}
		else
		{
			self.remote_end_point.cancel_rendezvous_zero_copy_tagged_message(self.cancellation_handle);
			true
		}
	}
}
