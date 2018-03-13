// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// This works because on 64-bit platforms nearly all pointers are malloc'd on 8-byte (64-bit) boundaries.
/// We can also use the top 16-bits, but we have to sign extend bit 48 to do so.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct TagForLowestThreeBits(usize);

impl TagForLowestThreeBits
{
	#[cfg(not(target_pointer_width = "64"))]
	const Error: () = "Pointer size should be 64 bits";
	
	const TagMask: usize = 0b111;
	
	const PointerMask: usize = !Self::TagMask;
	
	const SerializerTag: Self = TagForLowestThreeBits(0b000);
	
	const DeserializerTag: Self = TagForLowestThreeBits(0b001);
	
	#[inline(always)]
	fn is(self, other: Self) -> bool
	{
		self.0 == other.0
	}
	
	#[inline(always)]
	fn tag<T>(self, untagged_pointer: *mut T) -> *mut T
	{
		debug_assert_eq!((untagged_pointer as usize) & Self::TagMask, 0, "untagged_pointer '{:?}' has non-zero bottom 3 bits, ie it is not aligned", untagged_pointer);
		
		(self.0 | (untagged_pointer as usize)) as *mut T
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn untag_just_tag<T>(tagged_pointer: *mut T) -> Self
	{
		let bits = tagged_pointer as usize;
		
		TagForLowestThreeBits(bits & Self::TagMask)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn untag_just_pointer<T>(tagged_pointer: *mut T) -> *mut T
	{
		let bits = tagged_pointer as usize;
		
		(bits & Self::PointerMask) as *mut T
	}
}
