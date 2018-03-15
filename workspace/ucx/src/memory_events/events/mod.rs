// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use ::libc::c_void;
use ::ucx_sys::*;


include!("MMapEvent.rs");
include!("MUnmapEvent.rs");
include!("MRemapEvent.rs");
include!("ShmAtEvent.rs");
include!("ShmDtEvent.rs");
include!("SBrkEvent.rs");
include!("VirtualMachineMMapEvent.rs");
include!("VirtualMachineMUnmapEvent.rs");
include!("MemoryAddressAndSize.rs");
include!("MemoryGpuAllocEvent.rs");
include!("MemoryGpuFreeEvent.rs");
