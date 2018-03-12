// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// `ib_addr`.
/// Debug format is two 64-bit integers: A subnet prefix, and an interface id.
#[repr(C)]
pub union ib_addr
{
	/// Raw byte form.
	pub uib_addr8: [u8; 16],
	
	/// Always big-endian.
	pub uib_addr16: [BigEndian<u16>; 8],
	
	/// Always big-endian.
	pub uib_addr32: [BigEndian<u32>; 4],
	
	/// Always big-endian.
	pub uib_addr64: [BigEndian<u64>; 2],
}

impl Debug for ib_addr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{:#018X}:{:#018X}", self.sib_subnet_prefix().as_raw_native_endian_value(), self.sib_interface_id().as_raw_native_endian_value())
	}
}

impl Clone for ib_addr
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			uib_addr64: unsafe { self.uib_addr64 },
		}
	}
}

impl Copy for ib_addr
{
}

impl PartialEq for ib_addr
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe { self.uib_addr64 == other.uib_addr64 }
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool
	{
		unsafe { self.uib_addr64 != other.uib_addr64 }
	}
}

impl Eq for ib_addr
{
}

impl PartialOrd for ib_addr
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		unsafe { self.uib_addr64.partial_cmp(&other.uib_addr64) }
	}
}

impl Ord for ib_addr
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		unsafe { self.uib_addr64.cmp(&other.uib_addr64) }
	}
}

impl Hash for ib_addr
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		unsafe { self.uib_addr64.hash(state) }
	}
}

impl ib_addr
{
	/// A wildcard bind address.
	pub const Any: Self = Self
	{
		uib_addr64: [BigEndian::Zero; 2]
	};
	
	/// Useless internal method for C API compatibility.
	/// All parameters are big-endian.
	#[inline(always)]
	pub fn new(subnet_prefix: BigEndian<u64>, interface_id: BigEndian<u64>) -> Self
	{
		Self
		{
			uib_addr64:
			[
				subnet_prefix,
				interface_id,
			],
		}
	}
	
	/// Useless internal method for C API compatibility.
	/// All parameters are big-endian.
	#[inline(always)]
	pub fn set(&mut self, w1: BigEndian<u32>, w2: BigEndian<u32>, w3: BigEndian<u32>, w4: BigEndian<u32>)
	{
		unsafe
		{
			* self.uib_addr32.get_unchecked_mut(0) = w1;
			* self.uib_addr32.get_unchecked_mut(1) = w2;
			* self.uib_addr32.get_unchecked_mut(2) = w3;
			* self.uib_addr32.get_unchecked_mut(3) = w4;
		}
	}
	
	/// Does this instance represent any address?
	/// (Internally, are all bytes zero).
	#[inline(always)]
	pub fn ib_addr_any(&self) -> bool
	{
		unsafe
		{
			self.uib_addr64.get_unchecked(0).is_zero() && self.uib_addr64.get_unchecked(1).is_zero()
		}
	}
	
	/// Is this the loopback address?
	#[inline(always)]
	pub fn ib_addr_loopback(&self) -> bool
	{
		(
			self.u32_0()
			| self.u32_1()
			| self.u32_2()
			| (self.u32_3() ^ BigEndian::from_native_endian_value(1).as_raw_native_endian_value())
		) == 0
	}
	
	/// Raw form.
	#[inline(always)]
	pub fn sib_raw(&self) -> &[u8; 16]
	{
		unsafe { self.sib_addr8() }
	}
	
	/// Subnet Prefix, big endian.
	#[inline(always)]
	pub fn sib_subnet_prefix(&self) -> &BigEndian<u64>
	{
		unsafe { self.ib_u().uib_addr64.get_unchecked(0) }
	}
	
	/// Subnet Interface Id, big endian.
	#[inline(always)]
	pub fn sib_interface_id(&self) -> &BigEndian<u64>
	{
		unsafe { self.ib_u().uib_addr64.get_unchecked(1) }
	}
	
	/// Useless internal method for C API compatibility.
	#[inline(always)]
	pub fn ib_u(&self) -> &Self
	{
		self
	}
	
	/// Useless internal method for C API compatibility.
	#[inline(always)]
	pub unsafe fn sib_addr8(&self) -> &[u8; 16]
	{
		&self.ib_u().uib_addr8
	}
	
	/// Useless internal method for C API compatibility.
	/// Always big-endian.
	#[inline(always)]
	pub unsafe fn sib_addr16(&self) -> &[BigEndian<u16>; 8]
	{
		&self.ib_u().uib_addr16
	}
	
	/// Useless internal method for C API compatibility.
	/// Always big-endian.
	#[inline(always)]
	pub unsafe fn sib_addr32(&self) -> &[BigEndian<u32>; 4]
	{
		&self.ib_u().uib_addr32
	}
	
	/// Useless internal method for C API compatibility.
	/// Always big-endian.
	#[inline(always)]
	pub unsafe fn sib_addr64(&self) -> &[BigEndian<u64>; 2]
	{
		&self.ib_u().uib_addr64
	}
	
	#[inline(always)]
	fn u32_0(&self) -> u32
	{
		(unsafe { self.uib_addr32.get_unchecked(0) }).as_raw_native_endian_value()
	}
	
	#[inline(always)]
	fn u32_1(&self) -> u32
	{
		(unsafe { self.uib_addr32.get_unchecked(1) }).as_raw_native_endian_value()
	}
	
	#[inline(always)]
	fn u32_2(&self) -> u32
	{
		(unsafe { self.uib_addr32.get_unchecked(2) }).as_raw_native_endian_value()
	}
	
	#[inline(always)]
	fn u32_3(&self) -> u32
	{
		(unsafe { self.uib_addr32.get_unchecked(3) }).as_raw_native_endian_value()
	}
}
