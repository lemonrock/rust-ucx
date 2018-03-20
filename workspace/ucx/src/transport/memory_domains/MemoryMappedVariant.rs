// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


///Wraps up part of UCT's approach to configuration.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MemoryMappedVariant
{
	/// POSIX.
	POSIX,
	
	/// SysV.
	SysV,
	
	/// XPMEM (An out-of-tree Linux kernel module).
	/// Performs better than all other intra-node memory domain components.
	XPMEM,
}

impl MemoryMappedVariant
{
	/// UCT name for FFI.
	///
	/// No more than 16 characters long (excluding final `\0`).
	#[inline(always)]
	pub fn memory_domain_component_name(&self) -> &'static CStr
	{
		use self::MemoryMappedVariant::*;
		
		match *self
		{
			POSIX => unsafe { CStr::from_ptr(b"posix\0" as *const u8 as *const c_char) },
			
			SysV => unsafe { CStr::from_ptr(b"sysv\0" as *const u8 as *const c_char) },
			
			XPMEM => unsafe { CStr::from_ptr(b"xpmem\0" as *const u8 as *const c_char) },
		}
	}
	
	/// UCT configuration prefix.
	#[inline(always)]
	pub fn memory_domain_configuration_prefix(&self) -> &'static CStr
	{
		use self::MemoryMappedVariant::*;
		
		match *self
		{
			POSIX => unsafe { CStr::from_ptr(b"POSIX_\0" as *const u8 as *const c_char) },
			
			SysV => unsafe { CStr::from_ptr(b"SYSV_\0" as *const u8 as *const c_char) },
			
			XPMEM => unsafe { CStr::from_ptr(b"XPMEM_\0" as *const u8 as *const c_char) },
		}
	}
}
