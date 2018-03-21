// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An available memory domain component.
#[derive(Debug)]
pub struct AvailableMemoryDomainComponent(uct_md_resource_desc);

impl AvailableMemoryDomainComponent
{
	/// Name.
	///
	/// No more than 16 characters long (excluding final `\0`).
	#[inline(always)]
	pub fn name(&self) -> Cow<CStr>
	{
		let maximum_length = self.0.md_name.len();
		
		let ffi_name = &self.0.md_name as *const c_char;
		
		let length = unsafe { strnlen(ffi_name, maximum_length) };
		
		if length == maximum_length
		{
			// Need to adjust for missing final `\0`.
			let mut bytes = Vec::with_capacity(length + 1);
			let c_string = unsafe
			{
				copy_nonoverlapping(ffi_name as *const u8, bytes.as_mut_ptr(), length);
				bytes.set_len(length);
				CString::from_vec_unchecked(bytes)
			};
			Cow::Owned(c_string)
		}
		else
		{
			Cow::Borrowed(unsafe { CStr::from_ptr(ffi_name) })
		}
	}
}
