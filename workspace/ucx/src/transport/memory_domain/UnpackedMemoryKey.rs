// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An unpacked memory key.
#[derive(Debug)]
pub struct UnpackedMemoryKey(uct_rkey_bundle);

impl Drop for UnpackedMemoryKey
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let status = unsafe { uct_rkey_release(&self.0) };
		
		if !status.is_ok()
		{
			panic!("Unexpected status '{:?}'", status.parse())
		}
	}
}

impl UnpackedMemoryKey
{
	/// Unpack a `sender_buffer_packed_remote_key` to a remote memory key.
	#[inline(always)]
	pub fn from_tagged_message_rendezvous_sender_buffer_packed_remote_key(sender_buffer_packed_remote_key: NonNull<u8>) -> Result<Self, ErrorCode>
	{
		let mut this: Self = unsafe { uninitialized() };
		
		let status = unsafe { uct_rkey_unpack(sender_buffer_packed_remote_key.as_ptr() as *const c_void, &mut this.0) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(this),
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
	
	/// Unpack a `packed_key_buffer` to a remote memory key.
	#[inline(always)]
	pub fn unpack(packed_key_buffer: &[u8]) -> Result<Self, ErrorCode>
	{
		Self::from_tagged_message_rendezvous_sender_buffer_packed_remote_key(unsafe { NonNull::new_unchecked(packed_key_buffer.as_ptr() as *mut _) })
	}
	
	/// A local pointer to remote memory.
	///
	/// Only for intra-node (shared memory) and loopback-to-self memory domains (strictly, the memory domain's `attributes().supports_feature(_bindgen_ty_1::RKEY_PTR)`.
	/// This is ***not tested*** for in debug builds; instead, an error is returned.
	///
	/// Does not work for GPU accelerators (eg CUDA, RoCM).
	#[inline(always)]
	pub fn to_local_pointer_if_memory_domain_is_intra_node_or_loopback_to_self(&mut self, remote_memory_address: RemoteMemoryAddress) -> Result<NonNull<u8>, ErrorCode>
	{
		let mut local_address = unsafe { uninitialized() };
		
		let status = unsafe { uct_rkey_ptr(&mut self.0, remote_memory_address.0, &mut local_address) };
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => Ok(unsafe { NonNull::new_unchecked(local_address as *mut u8) }),
			
			Error(error_code) => Err(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status),
		}
	}
	
	/// Remote key.
	#[inline(always)]
	pub fn remote_key_descriptor(&self) -> uct_rkey_t
	{
		self.0.rkey
	}
	
//	/// Handle.
//	#[inline(always)]
//	pub(crate) fn handle(&self) -> *mut c_void
//	{
//		self.0.handle
//	}
//
//	/// Memory domain component pointer.
//	///
//	/// (ie `*mut uct_md_component_t`).
//	#[inline(always)]
//	pub(crate) fn uct_md_component_t(&self) -> *mut c_void
//	{
//		self.0.type_
//	}
}
