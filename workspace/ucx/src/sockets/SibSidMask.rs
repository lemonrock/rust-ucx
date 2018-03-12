// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


bitflags!
{
	/// Bit flags for settings of `sib_sid` in a `sockaddr_ib`.
	pub struct SibSidMask: u64
	{
		/// Also known as `RDMA_IB_IP_PS_MASK`.
		const RdmaPortSpaceSet = 0xFFFFFFFFFFFF0000;
		
		/// Also known as `RDMA_IB_IP_PORT_MASK`.
		const PortSet = 0x000000000000FFFF;
	}
}

impl SibSidMask
{
	#[inline(always)]
	fn to_big_endian_u64(self) -> BigEndian<u64>
	{
		BigEndian::from_native_endian_value(self.bits)
	}
}
