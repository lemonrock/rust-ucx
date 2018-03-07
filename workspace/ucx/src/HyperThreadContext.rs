// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An application context is suitable for use on a particular hyper thread.
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct HyperThreadContext
{
	handle: ucp_context_h,
	hyper_thread_context_handle_drop_safety: Rc<HyperThreadContextHandleDropSafety>,
}

impl Debug for HyperThreadContext
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		self.debug_fmt(f)
	}
}

impl PrintInformation for HyperThreadContext
{
	const DebugName: &'static str = "ApplicationContext";
	
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE)
	{
		unsafe { ucp_context_print_info(self.handle, stream) };
	}
}

impl HasAttributes for HyperThreadContext
{
	type Attributes = HyperThreadContextAttributes;
	
	#[inline(always)]
	fn attributes(&self) -> Self::Attributes
	{
		Self::Attributes::query(self.handle)
	}
}

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

impl HyperThreadContext
{
	/// Makes a region of memory remotely accessible (also known as 'mapping', 'registering' or 'pinning').
	/// Panics on any error, including out-of-memory.
	///
	/// If passing a `memory_address` of `AllocateAndRegister` or `AllocateAndRegisterWithAddressAddress`, the resultant allocated address can be found by calling `OurRemotelyAccessibleMemory.attributes().address()` .
	///
	/// `length` must not be zero.
	#[inline(always)]
	pub fn register_memory_as_remotely_accessible(&self, memory_address: MemoryAddress, length: usize, non_blocking: bool) -> OurRemotelyAccessibleMemory
	{
		assert_ne!(length, 0, "length is zero");
		
		let (address, mut flags) = memory_address.address_and_flags();
		
		if non_blocking
		{
			flags |= _bindgen_ty_2::NONBLOCK;
		}
		
		let parameters = ucp_mem_map_params_t
		{
			field_mask: (ucp_mem_map_params_field::ADDRESS | ucp_mem_map_params_field::LENGTH | ucp_mem_map_params_field::FLAGS).0 as u64,
			address: address as *mut c_void,
			length,
			flags: flags.0 as _,
		};
		
		let mut handle = unsafe { uninitialized() };
		panic_on_error!(ucp_mem_map, self.handle, &parameters, &mut handle);
		OurRemotelyAccessibleMemory
		{
			handle,
			our_remotely_accessible_memory_handle_drop_safety: Rc::new(OurRemotelyAccessibleMemoryHandleDropSafety(handle, self.hyper_thread_context_handle_drop_safety.clone())),
			hyper_thread_context_handle: self.handle,
		}
	}
}
