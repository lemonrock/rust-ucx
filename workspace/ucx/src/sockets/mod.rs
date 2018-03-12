// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use self::RdmaPortSpace::*;
use super::ffi_helpers::*;
use ::libc::in6_addr;
use ::libc::sockaddr;
#[cfg(any(target_os = "ios", target_os = "macos"))] use ::libc::sockaddr_ctl;
#[cfg(any(target_os = "dragonfly",  target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "openbsd"))] use ::libc::sockaddr_dl;
use ::libc::sockaddr_in;
use ::libc::sockaddr_in6;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::sockaddr_ll;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::sockaddr_nl;
use ::libc::sockaddr_un;
use ::libc::socklen_t;
use ::std::cmp::Eq;
use ::std::cmp::PartialEq;
use ::std::cmp::Ord;
use ::std::cmp::Ordering;
use ::std::cmp::PartialOrd;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::ptr::copy_nonoverlapping;
use ::std::slice::from_raw_parts;


include!("AF_IB.rs");
include!("ib_addr.rs");
include!("PF_IB.rs");
include!("RdmaPortSpace.rs");
include!("SibSidMask.rs");
include!("sockaddr_ib.rs");
include!("SocketAddress.rs");
