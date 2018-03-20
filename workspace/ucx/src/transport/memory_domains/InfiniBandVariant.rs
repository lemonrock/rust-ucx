// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


///Wraps up part of UCT's approach to configuration.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum InfiniBandVariant
{
	/// ?
	CM,
	
	/// Unreliable Datagram.
	UnreliableDatagram,
	
	/// Unreliable Datagram (Optimized for Mellanox Series 5 devices).
	UnreliableDatagramOptimizedForMellanoxSeries5Devices,
	
	/// Reliably Connected
	ReliablyConnected,
	
	/// Reliably Connected (Optimized for Mellanox Series 5 devices).
	ReliablyConnectedOptimizedForMellanoxSeries5Devices,
	
	/// Dynamically Connected
	DynamicallyConnected,
	
	/// Dynamically Connected (Optimized for Mellanox Series 5 devices).
	DynamicallyConnectedOptimizedForMellanoxSeries5Devices,
}

impl InfiniBandVariant
{
	/// UCT name for FFI.
	///
	/// No more than 10 characters long (excluding final `\0`).
	#[inline(always)]
	pub fn transport_layer_name(&self) -> &'static CStr
	{
		use self::InfiniBandVariant::*;
		
		match *self
		{
			CM => unsafe { CStr::from_ptr(b"cm\0" as *const u8 as *const c_char) },
			
			UnreliableDatagram => unsafe { CStr::from_ptr(b"ud\0" as *const u8 as *const c_char) },
			
			UnreliableDatagramOptimizedForMellanoxSeries5Devices => unsafe { CStr::from_ptr(b"ud_mlx5\0" as *const u8 as *const c_char) },
			
			ReliablyConnected => unsafe { CStr::from_ptr(b"rc\0" as *const u8 as *const c_char) },
			
			ReliablyConnectedOptimizedForMellanoxSeries5Devices => unsafe { CStr::from_ptr(b"rc_mlx5\0" as *const u8 as *const c_char) },
			
			DynamicallyConnected => unsafe { CStr::from_ptr(b"dc\0" as *const u8 as *const c_char) },
			
			DynamicallyConnectedOptimizedForMellanoxSeries5Devices => unsafe { CStr::from_ptr(b"dc_mlx5\0" as *const u8 as *const c_char) },
		}
	}
	
	/// UCT configuration prefix.
	#[inline(always)]
	pub fn transport_layer_configuration_prefix(&self) -> &'static CStr
	{
		use self::InfiniBandVariant::*;
		
		match *self
		{
			CM => unsafe { CStr::from_ptr(b"CM_\0" as *const u8 as *const c_char) },
			
			UnreliableDatagram => unsafe { CStr::from_ptr(b"UD_VERBS_\0" as *const u8 as *const c_char) },
			
			UnreliableDatagramOptimizedForMellanoxSeries5Devices => unsafe { CStr::from_ptr(b"UD_MLX5_\0" as *const u8 as *const c_char) },
			
			ReliablyConnected => unsafe { CStr::from_ptr(b"RC_VERBS_\0" as *const u8 as *const c_char) },
			
			ReliablyConnectedOptimizedForMellanoxSeries5Devices => unsafe { CStr::from_ptr(b"RC_MLX5_\0" as *const u8 as *const c_char) },
			
			DynamicallyConnected => unsafe { CStr::from_ptr(b"DC_VERBS_\0" as *const u8 as *const c_char) },
			
			DynamicallyConnectedOptimizedForMellanoxSeries5Devices => unsafe { CStr::from_ptr(b"DC_MLX5_\0" as *const u8 as *const c_char) },
		}
	}
}
