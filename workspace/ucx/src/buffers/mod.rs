// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use ::ring::aead::seal_in_place;
use ::ring::aead::SealingKey;
use ::ring::rand::SecureRandom;
use ::ring::rand::SystemRandom;
use ::std::mem::uninitialized;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::NonNull;
use ::std::slice::from_raw_parts;


include!("ByteBuffer.rs");
