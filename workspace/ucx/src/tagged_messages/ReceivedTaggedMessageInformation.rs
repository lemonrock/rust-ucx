// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Wrapper to make it easier to work with the underlying data.
#[derive(Default, Debug)]
pub struct ReceivedTaggedMessageInformation(pub(crate) ucp_tag_recv_info_t);

impl Clone for ReceivedTaggedMessageInformation
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		ReceivedTaggedMessageInformation
		(
			ucp_tag_recv_info_t
			{
				sender_tag: self.0.sender_tag,
				length: self.0.length,
			}
		)
	}
}

impl ReceivedTaggedMessageInformation
{
	/// Message's tag.
	#[inline(always)]
	pub fn message_tag(&self) -> TagValue
	{
		TagValue(self.0.sender_tag)
	}
	
	/// Message's length.
	#[inline(always)]
	pub fn message_length(&self) -> usize
	{
		self.0.length
	}
}
