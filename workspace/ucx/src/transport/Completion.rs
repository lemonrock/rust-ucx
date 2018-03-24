// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// Encapsulates a callback that is called (invoked) when a non-blocking operation completes.
///
/// If completion occurs immediately then it also invoked.
///
/// Can be passed more than once to UTC; will only be invoked once (by the non-blocking operation that completes last).
#[derive(Debug)]
pub struct Completion<C: CompletionHandler>
{
	reference_counted_copy_kept_in_raw_form_used_by_uct_completion_callback: *mut uct_completion_t,
	reference_counted_copy_kept_in_raw_form_used_by_uct_completion_callback_has_been_passed_to_uct: Cell<bool>,
	reference_counted_copy_kept_in_rust_form: Rc<CompletionInternal<C>>,
}

impl<C: CompletionHandler> Drop for Completion<C>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.reference_counted_copy_kept_in_raw_form_used_by_uct_has_not_been_passed_to_uct()
		{
			drop(self.reference_counted_copy_used_by_uct_completion_callback());
		}
	}
}

impl<C: CompletionHandler> Completion<C>
{
	/// Create a new completion.
	#[inline(always)]
	pub fn new(completion_handler: C) -> Self
	{
		let reference_counted_copy_kept_in_rust_form = CompletionInternal::new(completion_handler);
		
		Self
		{
			reference_counted_copy_kept_in_raw_form_used_by_uct_completion_callback: CompletionInternal::to_raw_pointer(&reference_counted_copy_kept_in_rust_form),
			reference_counted_copy_kept_in_raw_form_used_by_uct_completion_callback_has_been_passed_to_uct: Cell::new(false),
			reference_counted_copy_kept_in_rust_form,
		}
	}
	
	/// After a completion callback has been invoked, reset this instance so it can be reused.
	///
	/// Will return an error if already passed at least once to UCT and the callback has not yet been invoked.
	#[inline(always)]
	pub fn reuse(&mut self) -> Result<(), ()>
	{
		if self.reference_counted_copy_kept_in_raw_form_used_by_uct_has_not_been_passed_to_uct()
		{
			return Ok(());
		}
		
		if self.callback_already_invoked_by_uct()
		{
			self.reference_counted_copy_kept_in_raw_form_used_by_uct_completion_callback_has_been_passed_to_uct.set(false);
			self.reference_counted_copy_kept_in_raw_form_used_by_uct_completion_callback = CompletionInternal::to_raw_pointer(&self.reference_counted_copy_kept_in_rust_form);
			Ok(())
		}
		else
		{
			Err(())
		}
	}
	
	#[inline(always)]
	pub(crate) fn to_raw_pointer(&self) -> *mut uct_completion_t
	{
		if self.callback_already_invoked_by_uct()
		{
			panic!("Callback has already been invoked by UCT");
		}
		self.reference_counted_copy_kept_in_rust_form.increment_count();
		self.reference_counted_copy_kept_in_raw_form_used_by_uct_completion_callback_has_been_passed_to_uct.set(true);
		self.reference_counted_copy_kept_in_raw_form_used_by_uct_completion_callback
	}
	
	#[inline(always)]
	pub(crate) fn parse_status(&self, status: ucs_status_t) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		CompletionInternal::<C>::callback_by_rust(self.reference_counted_copy_kept_in_raw_form_used_by_uct_completion_callback, status)
	}
	
	#[inline(always)]
	pub(crate) fn completed_with_error(&self, error_code: ErrorCode)
	{
		CompletionInternal::<C>::callback_by_rust_completed_with_error(self.reference_counted_copy_kept_in_raw_form_used_by_uct_completion_callback, error_code)
	}
	
	#[inline(always)]
	fn reference_counted_copy_used_by_uct_completion_callback(&self) -> Rc<CompletionInternal<C>>
	{
		CompletionInternal::from_raw_pointer(self.reference_counted_copy_kept_in_raw_form_used_by_uct_completion_callback)
	}
	
	#[inline(always)]
	fn reference_counted_copy_kept_in_raw_form_used_by_uct_has_not_been_passed_to_uct(&self) -> bool
	{
		!self.reference_counted_copy_kept_in_raw_form_used_by_uct_completion_callback_has_been_passed_to_uct.get()
	}
	
	#[inline(always)]
	fn callback_already_invoked_by_uct(&self) -> bool
	{
		match Rc::strong_count(&self.reference_counted_copy_kept_in_rust_form)
		{
			1 => true,
			2 => false,
			unexpected_strong_count @ _ => panic!("Unexpected strong count '{}'", unexpected_strong_count),
		}
	}
}
