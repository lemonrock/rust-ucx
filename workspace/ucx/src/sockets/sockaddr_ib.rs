// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// `sockaddr_ib`.
/// The `sib_family` field is private as it should never be changed and does not need to ever be examined.
#[repr(C)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct sockaddr_ib
{
	sib_family: u16,
	
	/// Always big-endian.
	pub sib_pkey: BigEndian<u16>,
	
	/// Always big-endian.
	pub sib_flowinfo: BigEndian<u32>,
	
	/// Infiniband address.
	/// Can also be an IPv6 address.
	pub sib_addr: ib_addr,
	
	/// Contains a `RdmaPortSpace` in bits 16 - 31 if `sib_sid_mask` has the flag `RDMA_IB_IP_PS_MASK` set.
	/// Contains a `u16` port number in bits 0 - 15 if `sib_sid_mask` has flag `RDMA_IB_IP_PORT_MASK` set.
	/// Not other flags are defined as this time.
	/// Defined as big-endian in Linux kernel definitions but not obviously treated as big-endian.
	pub sib_sid: BigEndian<u64>,
	
	/// Defined as big-endian in Linux kernel definitions but not obviously treated as big-endian.
	pub sib_sid_mask: BigEndian<u64>,
	
	/// Always local-endian.
	pub sib_scope_id: u64,
}

impl sockaddr_ib
{
	/// Creates new instance.
	#[inline(always)]
	pub fn new(sib_pkey: BigEndian<u16>, sib_flowinfo: BigEndian<u32>, sib_addr: ib_addr, sib_sid: BigEndian<u64>, sib_sid_mask: BigEndian<u64>, sib_scope_id: u64) -> Self
	{
		Self
		{
			sib_family: AF_IB,
			sib_pkey,
			sib_flowinfo,
			sib_addr,
			sib_sid,
			sib_sid_mask,
			sib_scope_id,
		}
	}
	
	/// Creates a new instance of TCP over IP v6 over InfiniBand.
	#[inline(always)]
	pub fn tcp_over_ip_v6_over_infiniband(address: in6_addr, port: u16) -> Self
	{
		let rdma_port_space = TCP.shifted_to_big_endian();
		let port = BigEndian::from_native_endian_value(port as u64);
		let sid = rdma_port_space | port;
		
		Self::new(BigEndian::Zero, BigEndian::Zero, unsafe { transmute(address) }, sid, (SibSidMask::RdmaPortSpaceSet | SibSidMask::PortSet).to_big_endian_u64(), 0)
	}
	
	/// Creates a new instance of UDP over IP v6 over InfiniBand.
	#[inline(always)]
	pub fn udp_over_ip_v6_over_infiniband(address: in6_addr, port: u16) -> Self
	{
		let rdma_port_space = UDP.shifted_to_big_endian();
		let port = BigEndian::from_native_endian_value(port as u64);
		let sid = rdma_port_space | port;
		
		Self::new(BigEndian::Zero, BigEndian::Zero, unsafe { transmute(address) }, sid, (SibSidMask::RdmaPortSpaceSet | SibSidMask::PortSet).to_big_endian_u64(), 0)
	}
	
	/// Creates a new instance of IP v6 over InfiniBand.
	/// Very similar in behaviour to `udp_over_ip_v6_over_infiniband`; it's not clear if the port is used (it seems to be).
	#[inline(always)]
	pub fn ip_v6_over_infiniband(address: in6_addr, port: u16) -> Self
	{
		let rdma_port_space = IPOIB.shifted_to_big_endian();
		let port = BigEndian::from_native_endian_value(port as u64);
		let sid = rdma_port_space | port;
		
		Self::new(BigEndian::Zero, BigEndian::Zero, unsafe { transmute(address) }, sid, (SibSidMask::RdmaPortSpaceSet | SibSidMask::PortSet).to_big_endian_u64(), 0)
	}
	
	/// Creates a new instance over InfiniBand.
	#[inline(always)]
	pub fn infiniband(address: ib_addr) -> Self
	{
		let sid = IB.shifted_to_big_endian();
		
		Self::new(BigEndian::Zero, BigEndian::Zero, address, sid, SibSidMask::RdmaPortSpaceSet.to_big_endian_u64(), 0)
	}
	
	/// Creates a new instance.
	#[inline(always)]
	pub fn wild() -> Self
	{
		Self::new(BigEndian::Zero, BigEndian::Zero, ib_addr::Any, BigEndian::Zero, BigEndian::Zero, 0)
	}
}
