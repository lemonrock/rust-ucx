// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Deserializes (unpacks) active messages, tagged messages and remote memory load/stores which are sent using a buffered copy.
///
/// Will be drop'd after deserializing.
pub trait BufferedCopyDeserializer
{
	/// Deserialize (unpack) the data in the provided buffer `data_to_unpack_buffer`.
	///
	/// Must use the unpacked data.
	#[inline(always)]
	fn deserialize_and_use(&self, data_to_unpack_buffer: &[u8]);
}
