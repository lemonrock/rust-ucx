// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Features requested of UCX but are not necessarily supported.
///
/// It is recommended for applications only to request the features that are required for an optimal functionality.
///
/// Initialisation will fail if this features aren't available.
#[allow(missing_docs)]
#[serde(deny_unknown_fields, default)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RequestedFeatures
{
	pub remote_memory_access: bool,
	pub atomic_operations_32_bits_wide: bool,
	pub atomic_operations_64_bits_wide: bool,
	pub interrupt_notification_ie_wake_up: bool,
	pub stream_support: bool,
}

impl Default for RequestedFeatures
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			remote_memory_access: true,
			atomic_operations_32_bits_wide: true,
			atomic_operations_64_bits_wide: true,
			interrupt_notification_ie_wake_up: false,
			stream_support: false,
		}
	}
}

impl RequestedFeatures
{
	#[inline(always)]
	fn ucp_feature(&self, tag_sender_mask: Option<TagSenderMask>) -> ucp_feature
	{
		let mut features = ucp_feature(0);
		
		if tag_sender_mask.is_some()
		{
			features |= ucp_feature::TAG
		}
		
		if self.remote_memory_access
		{
			features |= ucp_feature::RMA
		}
		
		if self.atomic_operations_32_bits_wide
		{
			features |= ucp_feature::AMO32
		}
		
		if self.atomic_operations_64_bits_wide
		{
			features |= ucp_feature::AMO64
		}
		
		if self.interrupt_notification_ie_wake_up
		{
			features |= ucp_feature::WAKEUP
		}
		
		if self.stream_support
		{
			features |= ucp_feature::STREAM
		}
		
		features
	}
}
