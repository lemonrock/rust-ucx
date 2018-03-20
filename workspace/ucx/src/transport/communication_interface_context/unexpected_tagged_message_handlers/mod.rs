// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use super::super::super::buffers::*;
use super::super::super::local_to_remote_address_translations::RemoteAddress;
use super::super::super::tagged_messages::TagValue;
use ::std::ptr::NonNull;


include!("DoNothingUnexpectedTaggedMessageHandler.rs");
include!("UnexpectedTaggedMessageHandler.rs");
