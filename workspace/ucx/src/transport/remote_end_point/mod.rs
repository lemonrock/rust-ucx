// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::super::attributes::*;
use super::super::ffi_helpers::*;
use super::super::handle_drop_safeties::*;
use super::super::sockets::*;
use super::super::status::*;
use super::communication_interface_context::*;
use super::ActiveMessageIdentifier;
use super::CompletionHandleHelper;
use ::libc::c_uint;
use ::libc::c_void;
use ::libc::size_t;
use ::libc::ssize_t;
use ::libc::uint64_t;
use ::std::mem::size_of_val;
use ::std::mem::uninitialized;
use ::std::ptr::NonNull;
use ::std::sync::Arc;
use ::ucx_sys::*;

include!("RemoteEndPoint.rs");
