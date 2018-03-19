// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


pub(crate) trait ActiveMessageTracer
{
	#[inline(always)]
	fn trace(&self, active_message_trace_type: uct_am_trace_type_t, active_message_identifier: ActiveMessageIdentifier, read_only: UcxAllocatedByteBuffer, write_debug_c_string_to: &mut [c_char]);
}
