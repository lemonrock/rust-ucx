// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[derive(Debug, Clone, Default)]
pub(crate) struct TheirRemotelyAccessibleApplicationContext
{
	remotely_accessible_worker_end_point_addresses: HashMap<WorkerName, Rc<TheirRemotelyAccessibleWorkerEndPointAddress>>,
	remotely_accessible_server_end_point_addresses: HashMap<(WorkerName, ServerName), Rc<TheirRemotelyAccessibleServerEndPointAddress>>,
	remotely_accessible_memory_addresses: HashMap<MemoryName, Rc<TheirRemotelyAccessibleMemoryAddress>>,
}

impl TheirRemotelyAccessibleApplicationContext
{
	#[inline(always)]
	pub(crate) fn get_remotely_accessible_worker_end_point_address<'worker>(&'worker self, worker_name: &WorkerName) -> Option<&'worker Rc<TheirRemotelyAccessibleWorkerEndPointAddress>>
	{
		self.remotely_accessible_worker_end_point_addresses.get(worker_name)
	}
	
	#[inline(always)]
	pub(crate) fn set_remotely_accessible_worker_end_point_address(&mut self, worker_name: WorkerName, worker_address: TheirRemotelyAccessibleWorkerEndPointAddress)
	{
		self.remotely_accessible_worker_end_point_addresses.insert(worker_name, Rc::new(worker_address));
	}
	
	#[inline(always)]
	pub(crate) fn get_remotely_accessible_server_end_point_address<'worker>(&'worker self, worker_and_server_name: &(WorkerName, ServerName)) -> Option<&'worker Rc<TheirRemotelyAccessibleServerEndPointAddress>>
	{
		self.remotely_accessible_server_end_point_addresses.get(worker_and_server_name)
	}
	
	#[inline(always)]
	pub(crate) fn set_remotely_accessible_server_end_point_address(&mut self, worker_and_server_name: (WorkerName, ServerName), server_address: TheirRemotelyAccessibleServerEndPointAddress)
	{
		self.remotely_accessible_server_end_point_addresses.insert(worker_and_server_name, Rc::new(server_address));
	}
	
	#[inline(always)]
	pub(crate) fn get_remotely_accessible_memory_address<'worker>(&'worker self, memory_name: &MemoryName) -> Option<&'worker Rc<TheirRemotelyAccessibleMemoryAddress>>
	{
		self.remotely_accessible_memory_addresses.get(memory_name)
	}
	
	#[inline(always)]
	pub(crate) fn set_remotely_accessible_memory_address(&mut self, memory_name: MemoryName, memory_address: TheirRemotelyAccessibleMemoryAddress)
	{
		self.remotely_accessible_memory_addresses.insert(memory_name, Rc::new(memory_address));
	}
}
