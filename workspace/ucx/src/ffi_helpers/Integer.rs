// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


pub(crate) trait Integer: Copy + Eq + Debug
{
	const Zero: Self;
	
	const One: Self;
	
	#[inline(always)]
	fn is_zero(self) -> bool
	{
		self == Self::Zero
	}
	
	#[inline(always)]
	fn is_one(self) -> bool
	{
		self == Self::One
	}
	
	#[inline(always)]
	fn is_non_zero(self) -> bool
	{
		self != Self::Zero
	}
}

macro_rules! integer
{
    ($type: ty) =>
    {
        impl Integer for $type
        {
			const Zero: Self = 0;
			
			const One: Self = 1;
        }
    }
}

integer!(u8);
integer!(u16);
integer!(u32);
integer!(u64);
integer!(usize);

integer!(i8);
integer!(i16);
integer!(i32);
integer!(i64);
integer!(isize);
