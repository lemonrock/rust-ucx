// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A set of 64-bits.
/// Implements the trait `BitIndex` to make it easier to work with.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TagValue(pub u64);

impl BitIndex for TagValue
{
	#[inline(always)]
	fn bit_length() -> usize
	{
		u64::bit_length()
	}
	
	#[inline(always)]
	fn bit(&self, pos: usize) -> bool
	{
		self.0.bit(pos)
	}
	
	#[inline(always)]
	fn bit_range(&self, pos: Range<usize>) -> Self
	{
		self.0.bit_range(pos);
		*self
	}
	
	#[inline(always)]
	fn set_bit(&mut self, pos: usize, val: bool) -> &mut Self
	{
		self.0.set_bit(pos, val);
		self
	}
	
	#[inline(always)]
	fn set_bit_range(&mut self, pos: Range<usize>, val: Self) -> &mut Self
	{
		self.0.set_bit_range(pos, val.0);
		self
	}
}
