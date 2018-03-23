// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A wrapper 'new type' to make it easier to work with `uct_iovec_t`.
#[derive(Debug)]
pub struct ZeroCopyIoVector(uct_iov);

impl ByteBuffer for ZeroCopyIoVector
{
	/// Equivalent to `uct_iovec_t.buffer`.
	#[inline(always)]
	fn address(&self) -> NonNull<u8>
	{
		unsafe { NonNull::new_unchecked(self.0.buffer as *mut u8) }
	}
	
	/// Calculates total length of this `IoVector`.
	///
	/// Equivalent to `uct_iov_get_length`.
	///
	/// Currently has no support for stride.
	///
	/// If stride supported it should be like: `self.size_of_one_item() + ((self.number_of_items() - 1) * self.stride())`
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.number_of_items() * self.size_of_one_item()
	}
}

impl ZeroCopyIoVector
{
	#[inline(always)]
	fn new(address: NonNull<u8>, length: usize, memory_handle: NonNull<c_void>) -> Self
	{
		ZeroCopyIoVector
		(
			uct_iov
			{
				buffer: address.as_ptr() as *mut c_void,
				length,
				memh: memory_handle.as_ptr(),
				stride: 0,
				count: 1,
			}
		)
	}
	
	/// Size of one item.
	///
	/// Equivalent to `uct_iovec_t.length`.
	#[inline(always)]
	pub fn size_of_one_item(&self) -> usize
	{
		self.0.length
	}
	
	/// Memory handle.
	///
	/// Equivalent to `uct_iovec_t.memh`.
	#[inline(always)]
	pub fn memory_handle(&self) -> uct_mem_h
	{
		self.0.memh
	}
	
	/// Stride: not supported yet by UCX.
	///
	/// Equivalent to `uct_iovec_t.stride`.
	///
	/// Currently always zero (0).
	#[inline(always)]
	pub fn stride(&self) -> usize
	{
		self.0.stride
	}

	/// Number of items.
	///
	/// Equivalent to `uct_iovec_t.count`.
	///
	/// Currently always one (1).
	#[inline(always)]
	pub fn number_of_items(&self) -> usize
	{
		self.0.count as usize
	}
	
	/// Calculates total length of all `io_vectors`.
	///
	/// Equivalent to `uct_iov_total_length`.
	///
	/// Currently has no support for stride.
	///
	/// If stride supported it should be like: `self.length() + ((self.number_of_items() - 1) * self.stride())`
	#[inline(always)]
	pub fn total_length(io_vectors: &[Self]) -> usize
	{
		let mut total_length = 0;
		for io_vector in io_vectors.iter()
		{
			total_length += io_vector.length();
		}
		total_length
	}
}
