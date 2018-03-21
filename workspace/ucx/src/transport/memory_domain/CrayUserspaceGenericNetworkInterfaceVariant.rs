// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


///Wraps up part of UCT's approach to configuration.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CrayUserspaceGenericNetworkInterfaceVariant
{
	/// RDMA.
	RDMA,
	
	/// ?Short Message?
	ShortMessage,

	/// ?Unreliable data ??
	UDT,
}

impl CrayUserspaceGenericNetworkInterfaceVariant
{
	/// UCT name for FFI.
	///
	/// No more than 10 characters long (excluding final `\0`).
	#[inline(always)]
	pub fn transport_layer_name(&self) -> &'static CStr
	{
		use self::CrayUserspaceGenericNetworkInterfaceVariant::*;
		
		match *self
		{
			RDMA => unsafe { CStr::from_ptr(b"ugni_rdma\0" as *const u8 as *const c_char) },
			
			ShortMessage => unsafe { CStr::from_ptr(b"ugni_smsg\0" as *const u8 as *const c_char) },
			
			UDT => unsafe { CStr::from_ptr(b"ugni_udt\0" as *const u8 as *const c_char) },
		}
	}
	
	/// UCT configuration prefix.
	#[inline(always)]
	pub fn transport_layer_configuration_prefix(&self) -> &'static CStr
	{
		use self::CrayUserspaceGenericNetworkInterfaceVariant::*;
		
		match *self
		{
			// sic: No final underscore after `UGNI_RDMA`.
			RDMA => unsafe { CStr::from_ptr(b"UGNI_RDMA\0" as *const u8 as *const c_char) },
			
			// sic: No final underscore after `UGNI_SMSG`.
			ShortMessage => unsafe { CStr::from_ptr(b"UGNI_SMSG\0" as *const u8 as *const c_char) },
			
			// sic: No final underscore after `UGNI_UDT`.
			UDT => unsafe { CStr::from_ptr(b"UGNI_UDT\0" as *const u8 as *const c_char) },
		}
	}
}
