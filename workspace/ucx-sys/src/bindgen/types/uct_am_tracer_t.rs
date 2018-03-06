// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


pub type uct_am_tracer_t = Option<unsafe extern "C" fn(arg: *mut c_void, type_: uct_am_trace_type_t, id: u8, data: *const c_void, length: usize, buffer: *mut c_char, max: usize)>;
