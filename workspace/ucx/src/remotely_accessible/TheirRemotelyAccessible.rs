// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[derive(Debug, Clone, Default)]
pub(crate) struct TheirRemotelyAccessible
{
	by_application_context: HashMap<ApplicationContextName, TheirRemotelyAccessibleApplicationContext>
}

impl TheirRemotelyAccessible
{
	#[inline(always)]
	pub(crate) fn get<'a>(&'a self, application_context_name: &ApplicationContextName) -> Option<&'a TheirRemotelyAccessibleApplicationContext>
	{
		self.by_application_context.get(application_context_name)
	}
	
	#[inline(always)]
	pub(crate) fn get_mut_or_create<'a>(&'a mut self, application_context_name: ApplicationContextName) -> &'a mut TheirRemotelyAccessibleApplicationContext
	{
		self.by_application_context.entry(application_context_name).or_insert_with(TheirRemotelyAccessibleApplicationContext::default)
	}
}
