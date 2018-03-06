// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#![deny(missing_docs)]


//! # ucx
//!
//! This is a crate to provide mid-level wrappers around OpenUCX.
//!


extern crate ucx_sys;


/// Wrapper around UCX services (ucs) component of UCX
pub mod ucx_services;
