// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// What kind of memory address are we allocating and / or registering?
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MemoryAddress
{
	/// Use this to get memory allocated and registered.
	AllocateAndRegister,
	
	/// Use this to register memory allocated previously by any other method (eg `malloc`, `mmap`, etc).
	Register
	{
		/// An address already allocated at.
		already_allocated_at_address: NonNull<u8>,
	},
	
	/// Use this to get memory allocated and registered, with allocation at a preferred (hinted at) address.
	AllocateAndRegisterWithAddressHint
	{
		/// The hinted at address.
		address_hint: NonNull<u8>,
	},
	
	/// Use this to get memory allocated and registered, with allocation at a fixed address.
	/// Allocation will fail if this is not possible.
	AllocateAndRegisterWithFixedAddress
	{
		/// The fixed address.
		fixed_hint: NonNull<u8>,
	}
}

impl Default for MemoryAddress
{
	#[inline(always)]
	fn default() -> Self
	{
		MemoryAddress::AllocateAndRegister
	}
}

impl MemoryAddress
{
	const NoFlags: _bindgen_ty_2 = _bindgen_ty_2(0);
	
	#[inline(always)]
	fn address_and_flags(&self) -> (*mut c_void, _bindgen_ty_2)
	{
		use self::MemoryAddress::*;
		
		match *self
		{
			AllocateAndRegister => (null_mut(), _bindgen_ty_2::ALLOCATE),
			
			Register { already_allocated_at_address } => (already_allocated_at_address.as_ptr() as *mut c_void, Self::NoFlags),
			
			AllocateAndRegisterWithAddressHint { address_hint } => (address_hint.as_ptr() as *mut _, _bindgen_ty_2::ALLOCATE),
			
			AllocateAndRegisterWithFixedAddress { fixed_hint } => (fixed_hint.as_ptr() as *mut _, _bindgen_ty_2::ALLOCATE | _bindgen_ty_2::FIXED),
		}
	}
}
