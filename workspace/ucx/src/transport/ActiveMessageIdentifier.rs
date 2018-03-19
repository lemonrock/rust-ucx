// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct ActiveMessageIdentifier(u8);

impl ActiveMessageIdentifier
{
	pub const NumberOfBits: u8 = 5;
	
	pub const ExclusiveMaximum: u8 = 1 << Self::NumberOfBits;
	
	pub const InclusiveMinimum: Self = ActiveMessageIdentifier(0);
	
	pub const InclusiveMaximum: Self = ActiveMessageIdentifier(Self::ExclusiveMaximum as u8 - 1);
	
	/// Will panic in debug if value out-of-range
	#[inline(always)]
	pub fn new(value: u8) -> Self
	{
		debug_assert!(value < Self::ExclusiveMaximum, "value '{}' equals or exceeds ExclusiveMaximum '{}'", value, Self::ExclusiveMaximum);
		
		ActiveMessageIdentifier(value)
	}
}
