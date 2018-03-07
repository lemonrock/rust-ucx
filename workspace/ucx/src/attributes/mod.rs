// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use ::std::cmp::Eq;
use ::std::cmp::Ord;
use ::std::cmp::Ordering;
use ::std::cmp::PartialEq;
use ::std::cmp::PartialOrd;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::mem::uninitialized;
use ::ucx_sys::*;


include!("HasAttributes.rs");
include!("HyperThreadContextAttributes.rs");
include!("MappedMemoryAttributes.rs");
include!("WorkerAttributes.rs");
include!("WorkerThreadMode.rs");
