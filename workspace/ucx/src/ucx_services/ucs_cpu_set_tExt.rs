// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An extension trait for `ucs_cpu_set_t`.
#[allow(non_camel_case_types)]
pub trait ucs_cpu_set_tExt
{
	/// Creates a hyper thread (logical CPU) set which just contains the hyper thread.
	#[inline(always)]
	fn create_for_hyper_thread(hyper_thread_index: ZeroBasedHyperThreadIndex) -> Self;
	
	/// Sets a hyper thread (logical CPU).
	#[inline(always)]
	fn set_hyper_thread(&mut self, hyper_thread_index: ZeroBasedHyperThreadIndex);
	
	/// Un-sets a hyper thread (logical CPU).
	#[inline(always)]
	fn unset_hyper_thread(&mut self, hyper_thread_index: ZeroBasedHyperThreadIndex);
	
	/// Checks if a hyper thread (logical CPU) is set.
	/// If negative, it is unset.
	#[inline(always)]
	fn is_hyper_thread_set(&self, hyper_thread_index: ZeroBasedHyperThreadIndex) -> bool;
	
	/// Checks if a hyper thread (logical CPU) is unset.
	/// If negative, it is set.
	#[inline(always)]
	fn is_hyper_thread_unset(&self, hyper_thread_index: ZeroBasedHyperThreadIndex) -> bool
	{
		!self.is_hyper_thread_set(hyper_thread_index)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn index_and_offset(&self, hyper_thread_index: ZeroBasedHyperThreadIndex) -> (usize, usize);
}

impl ucs_cpu_set_tExt for ucs_cpu_set_t
{
	#[inline(always)]
	fn create_for_hyper_thread(hyper_thread_index: ZeroBasedHyperThreadIndex) -> Self
	{
		let mut set = Self
		{
			ucs_bits: unsafe { zeroed() },
		};
		
		set.set_hyper_thread(hyper_thread_index);
		
		set
	}
	
	#[inline(always)]
	fn set_hyper_thread(&mut self, hyper_thread_index: ZeroBasedHyperThreadIndex)
	{
		let (index, offset) = self.index_and_offset(hyper_thread_index);
		self.ucs_bits[index] |= 1 << offset;
	}
	
	#[inline(always)]
	fn unset_hyper_thread(&mut self, hyper_thread_index: ZeroBasedHyperThreadIndex)
	{
		let (index, offset) = self.index_and_offset(hyper_thread_index);
		self.ucs_bits[index] &= !(1 << offset);
	}
	
	#[inline(always)]
	fn is_hyper_thread_set(&self, hyper_thread_index: ZeroBasedHyperThreadIndex) -> bool
	{
		let (index, offset) = self.index_and_offset(hyper_thread_index);
		0 != (self.ucs_bits[index] & (1 << offset))
	}
	
	#[inline(always)]
	fn index_and_offset(&self, hyper_thread_index: ZeroBasedHyperThreadIndex) -> (usize, usize)
	{
		let size_in_bits = 8 * size_of_val(&self.ucs_bits[0]);
		let hyper_thread = hyper_thread_index as usize;
		(hyper_thread / size_in_bits, hyper_thread % size_in_bits)
	}
}
