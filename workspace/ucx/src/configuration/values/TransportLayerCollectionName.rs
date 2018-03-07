// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Transport layer collection name.
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TransportLayerCollectionName
{
	/// Everything
	all,
	
	/// All shared memory transports; sm and shm are identical for some reason
	/// ie mm, knem, sysv, posix, cma and xpmem
	sm,
	
	/// All shared memory transports; sm and shm are identical for some reason
	/// ie mm, knem, sysv, posix, cma and xpmem
	shm,
	
	/// Selects sysv and posix shared memory transports
	mm,
	
	/// A specific shared memory transport
	knem,
	
	/// A specific shared memory transport
	sysv,
	
	/// A specific shared memory transport
	posix,
	
	/// A specific shared memory transport
	cma,
	
	/// A specific shared memory transport
	xpmem,
	
	/// Selects all Cray Aries transports
	/// ie ugni_smsg, ugni_udt and ugni_rdma
	ugni,
	
	/// Crag Aries ?Short Message?
	ugni_smsg,
	
	/// Cray Aries ?unreliable datagram?
	ugni_udt,
	
	/// Cray Aries RDMA
	ugni_rdma,
	
	/// Selects most InfiniBand / IB verbs / RDMA transports
	/// ie rc, ud, rc_mlx5 and ud_mlx5
	ib,
	
	/// Selects itself (!) and ud
	rc,
	
	/// Unreliable datagram, unaccelerated verbs library
	ud,
	
	/// Reliable connection, accelerated verbs library; selects additional transports
	/// ie rc_mlx5 and ud_mlx5
	rc_x,
	
	/// A specific accelerated transport for Mellanox ConnectX hardware
	rc_mlx5,
	
	/// Unreliable datagram, accelerated verbs library; selects additional transports
	/// ie ud_mlx5
	ud_x,
	
	/// Unreliable datagram, accelerated verbs library, for Mellanox ConnectX hardware
	ud_mlx5,
	
	/// Dynamic connection, accelerated verbs library; selects additional transports
	/// ie dc_mlx5
	dc_x,
	
	/// Dynamic connection, accelerated verbs library, for Mellanox ConnectX hardware
	dc_mlx5,
	
	/// The transport layer(s) that this device implements.
	Device(DeviceName),
}

impl ConfigurationValueJoin for TransportLayerCollectionName
{
	#[inline(always)]
	fn push(&self, string: &mut String)
	{
		string.push_str(&self.to_str())
	}
}

impl ConfigurationValueConverter for TransportLayerCollectionName
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		CString::new(self.to_str().to_string()).unwrap()
	}
}

impl TransportLayerCollectionName
{
	#[inline(always)]
	fn to_str(&self) -> Cow<str>
	{
		use self::TransportLayerCollectionName::*;
		use self::DeviceName::Specific;
		
		let value = match *self
		{
			all => "all",
			sm => "sm",
			shm => "shm",
			mm => "mm",
			knem => "knem",
			sysv => "sysv",
			posix => "posix",
			cma => "cma",
			xpmem => "xpmem",
			ugni => "ugni",
			ugni_smsg => "ugni_smsg",
			ugni_udt => "ugni_udt",
			ugni_rdma => "ugni_rdma",
			ib => "ib",
			rc => "rc",
			ud => "ud",
			rc_x => "rc_x",
			rc_mlx5 => "rc_mlx5",
			ud_x => "ud_x",
			ud_mlx5 => "ud_mlx5",
			dc_x => "dc_x",
			dc_mlx5 => "dc_mlx5",
			Device(ref value) => match *value
			{
				DeviceName::all => "all",
				Specific(ref value) =>
				{
					let mut s = "\\\\".to_owned();
					s.push_str(value);
					return Cow::Owned(s)
				}
			},
		};
		
		Cow::Borrowed(value)
	}
}
