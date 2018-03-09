// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Conversions to a C representation of a boolean value.
pub(crate) trait ToCBool<I: Integer>
{
	/// Zero for false.
	/// One for true.
	#[inline(always)]
	fn to_c_bool(self) -> I;
}

impl<I: Integer> ToCBool<I> for bool
{
	#[inline(always)]
	fn to_c_bool(self) -> I
	{
		if self
		{
			I::One
		}
		else
		{
			I::Zero
		}
	}
}
