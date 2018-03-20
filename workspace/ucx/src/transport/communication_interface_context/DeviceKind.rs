// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// What kind of device provides this communication interface?
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum DeviceKind
{
	/// Network.
	Network = 1,
	
	/// Intra-node shared memory.
	IntraNode = 2,
	
	/// An accelerator, such as GPU; effectively intra-node.
	Accelerator = 3,
	
	/// Ourselves, aka 'self'. Similar to shared memory.
	Loopback = 4,
}
