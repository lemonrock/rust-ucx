// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


unsafe extern "C" fn buffered_copy_deserialize_callback<BCD: BufferedCopyDeserializer>(arg: *mut c_void, data: *const c_void, length: usize)
{
	debug_assert!(!arg.is_null(), "arg is null");
	debug_assert!(!data.is_null(), "data is null");
	debug_assert_ne!(length, 0, "length is 0");
	
	let buffered_copy_deserializer = Box::from_raw(arg as *mut BCD);
	
	let data_to_unpack_buffer = from_raw_parts(data as *const u8, length);
	
	buffered_copy_deserializer.deserialize(data_to_unpack_buffer)
}
