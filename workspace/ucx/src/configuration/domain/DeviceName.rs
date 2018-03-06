// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Device name
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub enum DeviceName
{
	/// All devices.
	all,
	
	/// A specific device name.
	Specific(String),
}

impl ConfigurationValueJoin for DeviceName
{
	#[inline(always)]
	fn push(&self, string: &mut String)
	{
		string.push_str(self.to_str())
	}
}

impl ConfigurationValueConverter for DeviceName
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		CString::new(self.to_str()).unwrap()
	}
}

impl DeviceName
{
	#[inline(always)]
	fn to_str(&self) -> &str
	{
		use self::DeviceName::*;
		
		match *self
		{
			all => "all",
			Specific(ref value) => value,
		}
	}
}
