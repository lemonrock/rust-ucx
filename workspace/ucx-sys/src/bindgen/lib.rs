// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.



extern crate libc;


use ::std::clone::Clone;
use ::std::default::Default;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::fmt::Result;
use ::std::marker::Copy;
use ::std::marker::PhantomData;
use ::std::mem::transmute;
use ::std::mem::zeroed;
use ::std::option::Option;
use ::std::ops::BitOr;
use ::std::ops::BitOrAssign;
use ::std::ops::BitAnd;
use ::std::ops::BitAndAssign;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::cmp::PartialEq;
use ::std::cmp::Eq;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_uint;
use ::libc::c_ulong;
use ::libc::c_void;

use ::libc::cpu_set_t;
use ::libc::FILE;
use ::libc::off_t;
use ::libc::sockaddr;
use ::libc::socklen_t;

#[link(name = "ucx", kind = "static-nobundle")]
extern "C"
{
}

include!("constants.rs");
include!("enums.rs");
include!("functions.rs");
include!("statics.rs");
include!("structs.rs");
include!("types.rs");
include!("uses.rs");
include!("opaques.rs");
