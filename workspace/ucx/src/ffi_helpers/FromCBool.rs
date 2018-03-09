// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Conversions from a C representation of a boolean value.
pub(crate) trait FromCBool: Integer
{
	/// Converts zero to false.
	/// Converts non-zero (not just one) to true.
	#[inline(always)]
	fn from_c_bool(self) -> bool;
	
	/// Converts zero to false.
	/// Converts one to true.
	/// Panics for all other values.
	#[inline(always)]
	fn from_c_bool_strict(self) -> bool;
}

impl<I: Integer> FromCBool for I
{
	#[inline(always)]
	fn from_c_bool(self) -> bool
	{
		// Not self.is_one(), because C considers any non-zero value to be true.
		self.is_non_zero()
	}
	
	#[inline(always)]
	fn from_c_bool_strict(self) -> bool
	{
		match self
		{
			Self::Zero => false,
			Self::One => true,
			_ => panic!("Value is not a C boolean of 0 or 1, but '{:?}'", self),
		}
	}
}
