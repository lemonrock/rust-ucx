// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Additional reasons to wake up.
#[serde(deny_unknown_fields, default)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WakeUp
{
	/// Wake up on receive, and if using a tag sender mask, on tag transmit.
	pub wake_up_on_transmit: bool,
	
	/// Wake up on receive, and if using a tag sender mask, on tag receive.
	pub wake_up_on_receive: bool,

	/// Edge polling?
	pub edge: bool,
}

impl Default for WakeUp
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			wake_up_on_transmit: true,
			wake_up_on_receive: true,
			edge: false,
		}
	}
}

impl WakeUp
{
	#[inline(always)]
	pub(crate) fn wake_up_events(&self, tag_sender_mask: Option<TagSenderMask>, mut flags: ucp_wakeup_event_types) -> ucp_wakeup_event_types
	{
		if self.wake_up_on_transmit
		{
			flags |= ucp_wakeup_event_types::TX;
			if tag_sender_mask.is_some()
			{
				flags |= ucp_wakeup_event_types::TAG_SEND
			}
		}
		
		if self.wake_up_on_receive
		{
			flags |= ucp_wakeup_event_types::RX;
			if tag_sender_mask.is_some()
			{
				flags |= ucp_wakeup_event_types::TAG_RECV
			}
		}
		
		if self.edge
		{
			flags |= ucp_wakeup_event_types::EDGE;
		}
		
		flags
	}
}
