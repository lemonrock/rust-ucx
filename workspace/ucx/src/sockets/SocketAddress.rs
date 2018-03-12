// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A struct to abstract away the libc `sockaddr` type and to provide creation options for Infiniband and other `sockaddr` types not commonly supported if required.
pub enum SocketAddress
{
	/// An InfiniBand socket.
	InfiniBand(sockaddr_ib),
	
	/// An internet IP v4 address.
	InternetIpV4(sockaddr_in),
	
	/// An internet IP v6 address.
	InternetIpV6(sockaddr_in6),
	
	/// An Unix domain socket.
	Unix(sockaddr_un),
	
	/// A Linux netlink socket.
	#[cfg(any(target_os = "android", target_os = "linux"))] Netlink(sockaddr_nl),
	
	/// An iOS / Mac OS X SysControl socket.
	#[cfg(any(target_os = "ios", target_os = "macos"))] SysControl(sockaddr_ctl),
	
	/// A `data link` address, such as aa raw Ethernet socket using a MAC address.
	/// On Linux, this has an address family (`AF_`) of packet, and on the BSDs, it is link.
	#[cfg(any(target_os = "android", target_os = "linux", ))] DataLink(sockaddr_ll),
	
	/// A `data link` address, such as aa raw Ethernet socket using a MAC address.
	/// On Linux, this has an address family (`AF_`) of packet, and on the BSDs, it is link.
	#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "openbsd"))]  DataLink(sockaddr_dl),
}

impl Debug for SocketAddress
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		use self::SocketAddress::*;
		
		match *self
		{
			InfiniBand(ref socket_address) => write!(f, "InfiniBand({:?})", socket_address),
			
			// TODO: Use getnameinfo(), eg http://www.microhowto.info/howto/convert_an_ip_address_to_a_human_readable_string_in_c.html
			InternetIpV4(ref _socket_address) => write!(f, "InternetIpV4()"),
			
			// TODO: Use getnameinfo(), eg http://www.microhowto.info/howto/convert_an_ip_address_to_a_human_readable_string_in_c.html
			InternetIpV6(ref _socket_address) => write!(f, "InternetIpV6()"),
			
			Unix(ref _socket_address) => write!(f, "Unix()"),
			
			#[cfg(any(target_os = "android", target_os = "linux"))] Netlink(ref _socket_address) => write!(f, "Netlink()"),
			
			#[cfg(any(target_os = "ios", target_os = "macos"))] SysControl(ref _socket_address) => write!(f, "SysControl()"),
			
			#[cfg(any(target_os = "android", target_os = "linux"))] DataLink(ref _socket_address) => write!(f, "DataLink()"),
			
			#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "openbsd"))] DataLink(ref _socket_address) => write!(f, "DataLink()"),
		}
	}
}

impl Clone for SocketAddress
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		#[inline(always)]
		fn memory_copy<T>(source: *const T) -> T
		{
			let mut copy = unsafe { uninitialized() };
			unsafe { copy_nonoverlapping(source, &mut copy, 1) };
			copy
		}
		
		use self::SocketAddress::*;
		
		match *self
		{
			InfiniBand(ref socket_address) => InfiniBand(socket_address.clone()),
			
			InternetIpV4(ref socket_address) => InternetIpV4(memory_copy(socket_address)),
			
			InternetIpV6(ref socket_address) => InternetIpV6(memory_copy(socket_address)),
			
			Unix(ref socket_address) => Unix(memory_copy(socket_address)),
			
			#[cfg(any(target_os = "android", target_os = "linux"))] Netlink(ref socket_address) => Netlink(memory_copy(socket_address)),
			
			#[cfg(any(target_os = "ios", target_os = "macos"))] SysControl(ref socket_address) => SysControl(memory_copy(socket_address)),
			
			#[cfg(any(target_os = "android", target_os = "linux"))] DataLink(ref socket_address) => DataLink(memory_copy(socket_address)),
			
			#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "openbsd"))] DataLink(ref socket_address) => DataLink(memory_copy(socket_address)),
		}
	}
}

impl PartialEq for SocketAddress
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		let (self_socket_address, self_length) = self.suitable_for_ffi();
		let (other_socket_address, other_length) = other.suitable_for_ffi();
		
		self_socket_address == other_socket_address && self_length == other_length
	}
}

impl Eq for SocketAddress
{
}

impl Hash for SocketAddress
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		#[inline(always)]
		fn hash_bytes<H: Hasher, T>(state: &mut H, source: *const T)
		{
			let byte_pointer = source as *const u8;
			let length = size_of::<T>();
			(unsafe { from_raw_parts(byte_pointer, length) }).hash(state)
		}
		
		use self::SocketAddress::*;
		
		match *self
		{
			InfiniBand(ref socket_address) => hash_bytes(state, socket_address),
			
			InternetIpV4(ref socket_address) => hash_bytes(state, socket_address),
			
			InternetIpV6(ref socket_address) => hash_bytes(state, socket_address),
			
			Unix(ref socket_address) => hash_bytes(state, socket_address),
			
			#[cfg(any(target_os = "android", target_os = "linux"))] Netlink(ref socket_address) => hash_bytes(state, socket_address),
			
			#[cfg(any(target_os = "ios", target_os = "macos"))] SysControl(ref socket_address) => hash_bytes(state, socket_address),
			
			#[cfg(any(target_os = "android", target_os = "linux"))] DataLink(ref socket_address) => hash_bytes(state, socket_address),
			
			#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "openbsd"))] DataLink(ref socket_address) => hash_bytes(state, socket_address),
		}
	}
}

impl SocketAddress
{
	/// A reference suitable for FFI.
	/// Be very careful - `&self` needs to live as long as `*const sockaddr` is in use for.
	#[inline(always)]
	pub fn suitable_for_ffi(&self) -> (*const sockaddr, socklen_t)
	{
		use self::SocketAddress::*;
		
		let (socket_address, raw_length) = match *self
		{
			InfiniBand(ref socket_address) => (socket_address as *const _ as *const sockaddr, size_of::<sockaddr_ib>()),
			
			InternetIpV4(ref socket_address) => (socket_address as *const _ as *const sockaddr, size_of::<sockaddr_in>()),
			
			InternetIpV6(ref socket_address) => (socket_address as *const _ as *const sockaddr, size_of::<sockaddr_in6>()),
			
			Unix(ref socket_address) => (socket_address as *const _ as *const sockaddr, size_of::<sockaddr_un>()),
			
			#[cfg(any(target_os = "android", target_os = "linux"))] Netlink(ref socket_address) => (socket_address as *const _ as *const sockaddr, size_of::<sockaddr_nl>()),
			
			#[cfg(any(target_os = "ios", target_os = "macos"))] SysControl(ref socket_address) => (socket_address as *const _ as *const sockaddr, size_of::<sockaddr_ctl>()),
			
			#[cfg(any(target_os = "android", target_os = "linux"))] DataLink(ref socket_address) => (socket_address as *const _ as *const sockaddr, size_of::<sockaddr_ll>()),
			
			#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "openbsd"))] DataLink(ref socket_address) => (socket_address as *const _ as *const sockaddr, size_of::<sockaddr_dl>()),
		};
		
		(socket_address, raw_length as socklen_t)
	}
}
