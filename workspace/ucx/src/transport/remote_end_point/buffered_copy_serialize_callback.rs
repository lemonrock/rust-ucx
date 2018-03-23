// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


unsafe extern "C" fn buffered_copy_serialize_callback<BCS: BufferedCopySerializer>(dest: *mut c_void, arg: *mut c_void) -> usize
{
	debug_assert!(!dest.is_null(), "dest is null");
	debug_assert!(!arg.is_null(), "arg is null");
	
	let buffered_copy_serializer = Box::from_raw(arg as *mut BCS);
	buffered_copy_serializer.serialize(NonNull::new_unchecked(dest as *mut u8))
}
