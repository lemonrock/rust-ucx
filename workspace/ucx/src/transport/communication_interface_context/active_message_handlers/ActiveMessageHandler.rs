// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// An active message receive handler.
pub trait ActiveMessageHandler
{
	/// If flags contains `DESC`, then the `descriptor_or_part_thereof` is part of a descriptor which must be released later by `ReceiveDescriptor::release_message_descriptor` (`uct_iface_release_desc`) by the user if the callback returns `false`.
	///
	/// * Returns `true` if completed and the descriptor was consumed, and can be released by the caller.
	/// * Returns `false` if in progress and if descriptor is owned by the callee, and would be released later. Supported only if `DESC` is in `flags`. Otherwise an error.
	#[inline(always)]
	fn invoke(&self, descriptor_or_part_thereof: UcxAllocatedByteBuffer, flags: uct_cb_param_flags) -> bool;
}
