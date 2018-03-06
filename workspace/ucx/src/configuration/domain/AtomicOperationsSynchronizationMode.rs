// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Whether to use the CPU, device, or guess.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub enum AtomicOperationsSynchronizationMode
{
	/// CPU mode.
	cpu,
	
	/// Device mode.
	device,
	
	/// Prefer device mode if at least one active transport supports it, otherwise fallback to cpu mode.
	/// This is the default.
	guess,
}

impl Default for AtomicOperationsSynchronizationMode
{
	#[inline(always)]
	fn default() -> Self
	{
		AtomicOperationsSynchronizationMode::guess
	}
}

impl ConfigurationValueConverter for AtomicOperationsSynchronizationMode
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		use self::AtomicOperationsSynchronizationMode::*;
		
		CString::new
		(
			match *self
			{
				cpu => "cpu",
				device => "device",
				guess => "guess",
			}
		).unwrap()
	}
}
