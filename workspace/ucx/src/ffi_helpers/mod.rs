// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use ::libc::c_char;
use ::serde::de::Deserializer;
use ::serde::de::Deserialize;
use ::serde::de::Error as DeserializeError;
use ::serde::de::Visitor;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::marker::PhantomData;
use ::std::ops::BitOr;


include!("BigEndian.rs");
include!("FromCBool.rs");
include!("Integer.rs");
include!("null_or_empty_c_string.rs");
include!("ReservedForFutureUseFlags.rs");
include!("ToCBool.rs");
include!("UnsignedInteger.rs");
