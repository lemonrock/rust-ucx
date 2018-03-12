// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A simple trait to make it possible to target just integers.
pub trait Integer: Copy + Eq + Debug + BitOr<Self, Output=Self>
{
	/// Value representing zero.
	const Zero: Self;
	
	/// Value representing one.
	const One: Self;
	
	/// Value representing minimum value.
	const Minimum: Self;
	
	/// Value representing maximum value.
	const Maximum: Self;
	
	/// Is this zero?
	#[inline(always)]
	fn is_zero(self) -> bool
	{
		self == Self::Zero
	}
	
	/// Is this one?
	#[inline(always)]
	fn is_one(self) -> bool
	{
		self == Self::One
	}
	
	/// Is this non-zero (ie all values except for zero)?
	#[inline(always)]
	fn is_non_zero(self) -> bool
	{
		self != Self::Zero
	}
	
	/// To Big Endian (no-op on a Big Endian platform).
	fn to_big_endian(self) -> Self;
	
	/// From Big Endian (no-op on a Big Endian platform).
	fn from_big_endian(value: Self) -> Self;
}

macro_rules! integer
{
    ($type: ty, $minimum: expr, $maximum: expr) =>
    {
        impl Integer for $type
        {
			const Zero: Self = 0;
			
			const One: Self = 1;
			
			const Minimum: Self = $minimum;
			
			const Maximum: Self = $maximum;
	
			fn to_big_endian(self) -> Self
			{
				self.to_be()
			}
			
			fn from_big_endian(value: Self) -> Self
			{
				Self::from_be(value)
			}
        }
    }
}

integer!(u8, ::std::u8::MIN, ::std::u8::MAX);
integer!(u16, ::std::u16::MIN, ::std::u16::MAX);
integer!(u32, ::std::u32::MIN, ::std::u32::MAX);
integer!(u64, ::std::u64::MIN, ::std::u64::MAX);
integer!(usize, ::std::usize::MIN, ::std::usize::MAX);

integer!(i8, ::std::i8::MIN, ::std::i8::MAX);
integer!(i16, ::std::i16::MIN, ::std::i16::MAX);
integer!(i32, ::std::i32::MIN, ::std::i32::MAX);
integer!(i64, ::std::i64::MIN, ::std::i64::MAX);
integer!(isize, ::std::isize::MIN, ::std::isize::MAX);
