// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Does nothing.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DoNothingUnexpectedTaggedMessageHandler;

impl UnexpectedTaggedMessageHandler for DoNothingUnexpectedTaggedMessageHandler
{
	#[inline(always)]
	fn receive_headroom(&self) -> usize
	{
		0
	}
	
	#[inline(always)]
	fn unexpected_eager_tagged_message(&self, _sender_tag: TagValue, _tagged_message_data: UcxAllocatedByteBuffer, _immediate_data: u64)
	{
	}
	
	#[inline(always)]
	fn unexpected_eager_tagged_message_with_descriptor_with_user_defined_receive_headroom(&self, _sender_tag: TagValue, _descriptor_with_receive_headroom_and_tagged_message_data: UcxAllocatedByteBuffer, _immediate_data: u64) -> bool
	{
		true
	}
	
	#[inline(always)]
	fn unexpected_rendezvous_tagged_message(&self, _sender_tag: TagValue, _header: UcxAllocatedByteBuffer, _remote_memory_address: RemoteMemoryAddress, _remote_length: usize, _sender_buffer_packed_remote_key: NonNull<u8>)
	{
	}
	
	#[inline(always)]
	fn unexpected_rendezvous_tagged_message_with_descriptor_with_user_defined_receive_headroom(&self, _sender_tag: TagValue, _header: UcxAllocatedByteBuffer, _remote_memory_address: RemoteMemoryAddress, _remote_length: usize, _sender_buffer_packed_remote_key: NonNull<u8>) -> bool
	{
		true
	}
}
