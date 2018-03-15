// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Receive descriptor
#[derive(Debug)]
pub(crate) struct ReceiveDescriptor(uct_recv_desc);

impl ReceiveDescriptor
{
	/// Release callback.
	#[inline(always)]
	pub(crate) fn release_callback(&self) -> unsafe extern "C" fn(this: *mut uct_recv_desc_t, desc: *mut c_void)
	{
		self.0.cb.unwrap()
	}
}

/// Tagged Messages (TAG)
impl ReceiveDescriptor
{
	/// Release active message descriptor.
	///
	/// Equivalent to `uct_iface_release_desc`.
	///
	/// Releases active message descriptor `active_message_descriptor`, which was passed to `uct_am_callback_t` "the active message callback", and owned by the callee.
	#[inline(always)]
	fn release_active_message_descriptor(active_message_descriptor: *mut c_void)
	{
		#[inline(always)]
		unsafe fn uct_recv_desc(_desc: *mut c_void) -> *mut uct_recv_desc
		{
			*((_desc as *mut *mut uct_recv_desc).offset(-1))
		}
		
		unsafe
		{
			let release_desc = uct_recv_desc(active_message_descriptor);
			(&*release_desc).cb.unwrap()(release_desc, active_message_descriptor)
		}
	}
}
