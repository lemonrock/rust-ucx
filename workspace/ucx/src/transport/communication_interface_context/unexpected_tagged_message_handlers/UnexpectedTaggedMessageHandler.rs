// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Handles an unexpected tagged message.
pub trait UnexpectedTaggedMessageHandler
{
	/// How many bytes headroom to reserve for eager tagged messages.
	///
	/// Defaults to `0`.
	#[inline(always)]
	fn receive_headroom(&self) -> usize
	{
		0
	}
	
	/// Received an unexpected eager tagged message.
	///
	/// All message data is supplied to the call.
	#[inline(always)]
	fn unexpected_eager_tagged_message(&self, sender_tag: TagValue, tagged_message_data: UcxAllocatedByteBuffer, immediate_data: u64);
	
	/// Received an unexpected eager tagged message.
	///
	/// All message data is supplied to the call.
	///
	/// Return `true` if the message is processed, or `false` if the descriptor has been used; see definitions of `UCT_CB_PARAM_FLAG_DESC` in `uct_def.h`.
	///
	/// If false is returned then the callee must, at some point, call `ReceiveDescriptor::release_message_descriptor` (`uct_iface_release_desc`).
	#[inline(always)]
	fn unexpected_eager_tagged_message_with_descriptor_with_user_defined_receive_headroom(&self, sender_tag: TagValue, descriptor_with_receive_headroom_and_tagged_message_data: UcxAllocatedByteBuffer, immediate_data: u64) -> bool;
	
	/// Received an unexpected rendezvous tagged message.
	///
	/// The message header data is supplied to the call.
	///
	/// The remote location of the message data, and the remote end point to which it relates is supplied to the call.
	///
	/// The remote data can probably be received by a non-blocking `get` remote memory operation (RMO).
	///
	/// `sender_buffer_packed_remote_key` can be passed to `UnpackedMemoryKey::from_tagged_message_rendezvous_sender_buffer_packed_remote_key(sender_buffer_packed_remote_key).remote_key_descriptor()`.
	#[inline(always)]
	fn unexpected_rendezvous_tagged_message(&self, sender_tag: TagValue, header: UcxAllocatedByteBuffer, remote_memory_address: RemoteMemoryAddress, remote_length: usize, sender_buffer_packed_remote_key: NonNull<u8>);
	
	/// Received an unexpected rendezvous tagged message.
	///
	/// The message header data is supplied to the call.
	///
	/// The remote location of the message data, and the remote end point to which it relates is supplied to the call.
	///
	/// The remote data can probably be received by a non-blocking `get` remote memory operation (RMO).
	///
	/// Return `true` if the message is processed, or `false` if the descriptor has been used; see definitions of `UCT_CB_PARAM_FLAG_DESC` in `uct_def.h`.
	///
	/// If false is returned then the callee must, at some point, call `ReceiveDescriptor::release_message_descriptor` (`uct_iface_release_desc`).
	///
	/// `sender_buffer_packed_remote_key` can be passed to `UnpackedMemoryKey::from_tagged_message_rendezvous_sender_buffer_packed_remote_key(sender_buffer_packed_remote_key).remote_key_descriptor()`.
	#[inline(always)]
	fn unexpected_rendezvous_tagged_message_with_descriptor_with_user_defined_receive_headroom(&self, sender_tag: TagValue, header: UcxAllocatedByteBuffer, remote_memory_address: RemoteMemoryAddress, remote_length: usize, sender_buffer_packed_remote_key: NonNull<u8>) -> bool;
}
