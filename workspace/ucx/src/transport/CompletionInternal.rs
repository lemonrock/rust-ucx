// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


#[derive(Debug)]
#[repr(C)]
struct CompletionInternal<C: CompletionHandler>
{
	completion_handler: C,
	uct_required_field_which_must_be_last_field_in_struct: UnsafeCell<uct_completion>,
}

impl<C: CompletionHandler> CompletionInternal<C>
{
	#[inline(always)]
	fn new(completion_handler: C) -> Rc<Self>
	{
		Rc::new
		(
			Self
			{
				completion_handler,
				uct_required_field_which_must_be_last_field_in_struct: UnsafeCell::new
				(
					uct_completion
					{
						func: Self::callback_by_uct,
						count: 0,
					}
				)
			}
		)
	}
	
	unsafe extern "C" fn callback_by_uct(raw_pointer: *mut uct_completion_t, status: ucs_status_t)
	{
		let this = Self::from_raw_pointer(raw_pointer);
		
		debug_assert!(this.get_count() <= 0, "UCX count was not zero or negative");
		
		use self::Status::*;
		
		match status.parse()
		{
			IsOk => this.completed_ok(),
			
			Error(error_code) => this.completed_with_error(error_code),
			
			unexpected_status @ _ => panic!("Unexpected status '{:?}'", unexpected_status)
		}
	}
	
	#[inline(always)]
	fn callback_by_rust(raw_pointer: *mut uct_completion_t, status: ucs_status_t) -> Result<NonBlockingRequestCompletedOrInProgress<(), ()>, ()>
	{
		use self::Status::*;
		use self::NonBlockingRequestCompletedOrInProgress::*;
		
		match status.parse()
		{
			OperationInProgress => Ok(InProgress(())),
			
			IsOk =>
			{
				Self::as_if_called_by_uct(raw_pointer, |completion_internal| completion_internal.completed_ok());
				
				Ok(Completed(()))
			}
			
			Error(error_code) =>
			{
				Self::callback_by_rust_completed_with_error(raw_pointer, error_code);
				
				Err(())
			}
			
			unexpected_status @ _ =>
			{
				Self::as_if_called_by_uct(raw_pointer, |_completion_internal| {});
				
				panic!("Unexpected status '{:?}'", unexpected_status)
			}
		}
	}
	
	#[inline(always)]
	fn callback_by_rust_completed_with_error(raw_pointer: *mut uct_completion_t, error_code: ErrorCode)
	{
		Self::as_if_called_by_uct(raw_pointer, |completion_internal| completion_internal.completed_with_error(error_code))
	}
	
	#[inline(always)]
	fn as_if_called_by_uct<Action: FnOnce(Rc<Self>)>(raw_pointer: *mut uct_completion_t, action: Action)
	{
		let this = Self::from_raw_pointer(raw_pointer);
		
		let count = this.get_count() - 1;
		this.set_count(count);
		if count <= 0
		{
			action(this);
		}
		else
		{
			forget(this)
		}
	}
	
	#[inline(always)]
	fn completed_ok(&self)
	{
		self.completion_handler.completed_ok()
	}
	
	#[inline(always)]
	fn completed_with_error(&self, error_code: ErrorCode)
	{
		self.completion_handler.completed_with_error(error_code)
	}
	
	#[inline(always)]
	fn increment_count(&self)
	{
		let count = self.corrected_count();
		debug_assert_ne!(count, ::std::i32::MAX as u32, "Maximum count reached");
		self.set_count((count + 1) as i32);
	}
	
	#[inline(always)]
	fn corrected_count(&self) -> u32
	{
		let count = self.get_count();
		if count <= 0
		{
			0
		}
		else
		{
			count as u32
		}
	}
	
	#[inline(always)]
	fn get_count(&self) -> i32
	{
		(unsafe { & * self.uct_required_field_which_must_be_last_field_in_struct.get() }).count
	}
	
	#[inline(always)]
	fn set_count(&self, count: i32)
	{
		(unsafe { &mut * self.uct_required_field_which_must_be_last_field_in_struct.get() }).count = count
	}
	
	#[inline(always)]
	fn to_raw_pointer(reference_counted_copy_kept_in_rust_form: &Rc<Self>) -> *mut uct_completion_t
	{
		let raw = Rc::into_raw(reference_counted_copy_kept_in_rust_form.clone()) as *mut u8;
		unsafe { raw.offset(Self::offset()) as *mut uct_completion }
	}
	
	#[inline(always)]
	fn from_raw_pointer(raw_pointer: *mut uct_completion_t) -> Rc<Self>
	{
		debug_assert!(!raw_pointer.is_null(), "raw_pointer is null");
		unsafe
		{
			let raw = (raw_pointer as *mut u8).offset(-Self::offset());
			Rc::from_raw(raw as *mut Self)
		}
	}
	
	#[inline(always)]
	fn offset() -> isize
	{
		offset_of!(Self, uct_required_field_which_must_be_last_field_in_struct) as isize
	}
}
