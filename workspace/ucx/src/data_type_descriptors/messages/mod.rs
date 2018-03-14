// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::*;
use super::super::buffers::ByteBuffer;
use ::std::fmt::Debug;
use ::std::mem::size_of;
use ::std::ptr::NonNull;
use ::std::sync::Arc;


include!("ContiguousMessage.rs");
include!("GenericMessage.rs");
include!("IoVec.rs");
include!("IoVecMessage.rs");
include!("Message.rs");
