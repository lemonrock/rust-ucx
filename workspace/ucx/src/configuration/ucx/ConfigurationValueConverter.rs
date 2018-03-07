// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A way to convert values into CStrings suitable for use with the OpenUCX configuration API.
pub trait ConfigurationValueConverter
{
	/// Converts a value to a CString suitable for OpenUCX.
	#[inline(always)]
	fn convert(&self) -> CString;
}

impl ConfigurationValueConverter for bool
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		let string = if *self
		{
			"y"
		}
		else
		{
			"n"
		};
		CString::new(string.to_string()).unwrap()
	}
}

impl ConfigurationValueConverter for u32
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		CString::new(format!("{}", *self)).unwrap()
	}
}

impl ConfigurationValueConverter for f64
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		CString::new(format!("{}", *self)).unwrap()
	}
}

impl<T: Eq + Hash + ConfigurationValueJoin> ConfigurationValueConverter for HashSet<T>
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		let mut joined = String::with_capacity(self.len() * 10);
		let mut after_first = false;
		
		for value in self.iter()
		{
			if after_first
			{
				joined.push(',');
			}
			else
			{
				after_first = true;
			}
			value.push(&mut joined);
		}
		
		CString::new(joined).unwrap()
	}
}

impl<T: Eq + Hash + ConfigurationValueJoin> ConfigurationValueConverter for IndexSet<T>
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		let mut joined = String::with_capacity(self.len() * 10);
		let mut after_first = false;
		
		for value in self.iter()
		{
			if after_first
			{
				joined.push(',');
			}
			else
			{
				after_first = true;
			}
			value.push(&mut joined);
		}
		
		CString::new(joined).unwrap()
	}
}
