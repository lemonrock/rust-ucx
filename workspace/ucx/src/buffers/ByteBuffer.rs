// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// This implements a buffer of bytes over raw memory.
pub trait ByteBuffer
{
	/// Start address of this buffer.
	#[inline(always)]
	fn address(&self) -> NonNull<u8>;
	
	/// Length of this buffer.
	#[inline(always)]
	fn length(&self) -> usize;
	
	/// As a slice.
	#[inline(always)]
	fn as_slice<'a>(&'a self) -> &'a [u8]
	{
		unsafe { from_raw_parts(self.address().as_ptr() as *const _, self.length()) }
	}
	
	/// Seals using AEAD encryption.
	/// Returns an encrypted data 'box' with a trailing suffix (also known as a `tag`).
	#[inline(always)]
	fn seal(&self, sealing_key: &SealingKey, random_source: &SystemRandom) -> Sealed
	{
		Sealed::seal(sealing_key, random_source, self.address(), self.length())
	}
}
