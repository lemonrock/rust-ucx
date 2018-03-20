// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


bitflags!
{
	/// Flags indicating what features are supported by an interface.
	pub struct InterfaceFeaturesSupported: u64
	{
		// Active message capabilities
		
		/// Short active message.
		const AM_SHORT = 1 << 0;
		
		/// Buffered active message.
		const AM_BCOPY = 1 << 1;
		
		/// Zero-copy active message.
		const AM_ZCOPY = 1 << 2;
		
		
		// Pending operations.
		/// Does not seem to be tested for.
		const PENDING = 1 << 3;
		
		
		// PUT capabilities
		
		/// Short put.
		const PUT_SHORT = 1 << 4;
		
		/// Buffered put.
		const PUT_BCOPY = 1 << 5;
		
		/// Zero-copy put.
		const PUT_ZCOPY = 1 << 6;
		
		
		// GET capabilities
		
		/// Short get.
		const GET_SHORT = 1 << 8;
		
		/// Buffered get.
		const GET_BCOPY = 1 << 9;
		
		/// Zero-copy get.
		const GET_ZCOPY = 1 << 10;
		
		
		// Atomic operations capabilities
		
		/// 32bit atomic add.
		const ATOMIC_ADD32 = 1 << 16;
		
		/// 64bit atomic add.
		const ATOMIC_ADD64 = 1 << 17;
		
		/// 32bit atomic fetch-and-add.
		const ATOMIC_FADD32 = 1 << 18;
		
		/// 64bit atomic fetch-and-add.
		const ATOMIC_FADD64 = 1 << 19;
		
		/// 32bit atomic swap.
		const ATOMIC_SWAP32 = 1 << 20;
		
		/// 64bit atomic swap.
		const ATOMIC_SWAP64 = 1 << 21;
		
		/// 32bit atomic compare-and-swap.
		const ATOMIC_CSWAP32 = 1 << 22;
		
		/// 64bit atomic compare-and-swap.
		const ATOMIC_CSWAP64 = 1 << 23;
		
		
		// Atomic operations domain
		
		/// Atomic communications are consistent with respect to CPU operations.
		/// This is only true for shared memory.
		const ATOMIC_CPU = 1 << 30;
		
		/// Atomic communications are consistent only with respect to other atomics on the same device.
		/// This is only true for InfiniBand-like transports.
		const ATOMIC_DEVICE = 1 << 31;
		
		
		// Error handling capabilities
		
		/// Invalid buffer for short operation.
		const ERRHANDLE_SHORT_BUF = 1 << 32;
		
		/// Invalid buffer for buffered operation.
		const ERRHANDLE_BCOPY_BUF = 1 << 33;
		
		/// Invalid buffer for zero copy operation.
		const ERRHANDLE_ZCOPY_BUF = 1 << 34;
		
		/// Invalid AM id on remote.
		const ERRHANDLE_AM_ID = 1 << 35;
		
		/// Remote memory access.
		const ERRHANDLE_REMOTE_MEM = 1 << 36;
		
		/// Invalid length for buffered operation.
		const ERRHANDLE_BCOPY_LEN = 1 << 37;
		
		/// Remote peer failures/outage.
		const ERRHANDLE_PEER_FAILURE = 1 << 38;
		
		/// Endpoint check.
		const EP_CHECK = 1 << 39;
		
		
		// Connection establishment
		
		/// Supports connecting to interface.
		const CONNECT_TO_IFACE = 1 << 40;
		
		/// Supports connecting to specific endpoint.
		const CONNECT_TO_EP = 1 << 41;
		
		/// Supports connecting to sockaddr.
		const CONNECT_TO_SOCKADDR = 1 << 42;
		
		
		// Special transport flags
		
		/// Active messages may be received with duplicates.
		/// This happens if the transport does not keep enough information to detect retransmissions.
		/// True only for the `CM` InfiniBand-like transport.
		const AM_DUP = 1 << 43;
		
		
		// Callback invocation
		
		/// Interface supports setting a callback which is invoked only from the calling context of uct_worker_progress().
		const CB_SYNC = 1 << 44;
		
		/// Interface supports setting a callback which will be invoked within a reasonable amount of time if `uct_worker_progress()` is not being called.
		/// The callback can be invoked from any progress context and it may also be invoked when `uct_worker_progress()` is called.
		const CB_ASYNC = 1 << 45;
		
		
		// Event notification
		
		/// Event notification of send completion is supported.
		const EVENT_SEND_COMP = 1 << 46;
		
		/// Event notification of tag and active message receive is supported.
		const EVENT_RECV = 1 << 47;
		
		/// Event notification of signaled tag and active message is supported.
		const EVENT_RECV_SIG = 1 << 48;
		
		
		// Tag matching operations
		
		/// Hardware tag matching short eager support.
		const TAG_EAGER_SHORT = 1 << 50;
		
		/// Hardware tag matching bcopy eager support.
		const TAG_EAGER_BCOPY = 1 << 51;
		
		/// Hardware tag matching zcopy eager support.
		const TAG_EAGER_ZCOPY = 1 << 52;
		
		/// Hardware tag matching rendezvous zcopy support.
		const TAG_RNDV_ZCOPY = 1 << 53;
	}
}
