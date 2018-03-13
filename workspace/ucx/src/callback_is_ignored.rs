// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.



/// This function is provided to pass to non-blocking calls where the caller doesn't care about being notified of completion of the non-blocking call.
#[inline(always)]
pub unsafe extern "C" fn callback_is_ignored(_request: *mut c_void, _status: ucs_status_t)
{
}
