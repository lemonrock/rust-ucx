// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Application contexts configuration for each hyper thread (`per_hyper_thread`).
/// If a hyper thread does not have a specific configuration, then `unspecified_default` is used.
#[serde(deny_unknown_fields, default)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HyperThreadsApplicationContextConfigurations
{
	/// Used if `per_hyper_thread` does not contain a configuration.
	pub unspecified_default: ApplicationContextConfiguration,
	
	/// Per hyper thread configurations.
	pub per_hyper_thread: HashMap<ZeroBasedHyperThreadIndex, ApplicationContextConfiguration>,
}

impl HyperThreadsApplicationContextConfigurations
{
	#[inline(always)]
	pub(crate) fn application_context_configuration(&self, hyper_thread_index: ZeroBasedHyperThreadIndex) -> &ApplicationContextConfiguration
	{
		match self.per_hyper_thread.get(&hyper_thread_index)
		{
			Some(application_context_configuration) => application_context_configuration,
			None => &self.unspecified_default,
		}
	}
}
