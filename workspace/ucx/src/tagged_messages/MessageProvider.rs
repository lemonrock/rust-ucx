// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A trait to use with `pop()` to create correctly-sized (or at least, sufficiently-sized) uninitialized messages.
pub trait MessageProvider
{
	/// Type of messages provided.
	type M: Message;
	
	/// Provides an uninitialized message.
	#[inline(always)]
	fn provide_uninitialized_message(&mut self, received_tagged_message_information: ReceivedTaggedMessageInformation) -> Self::M;
}
