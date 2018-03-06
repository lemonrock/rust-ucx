// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


/// A trait to dump information to a file, standard out or standard error.
pub trait PrintInformation
{
	/// Print information to standard out.
	#[inline(always)]
	fn print_information_to_standard_out(&self)
	{
		self.print_information_to_stream(unsafe { stdout } as *mut _)
	}
	
	/// Print information to standard out.
	#[inline(always)]
	fn print_information_to_standard_error(&self)
	{
		self.print_information_to_stream(unsafe { stderr } as *mut _)
	}
	
	/// Print information to a file opened for writing.
	#[inline(always)]
	fn print_information_to_file(&self, file: File) -> Result<(), io::Error>
	{
		let file_descriptor = file.into_raw_fd();
		
		let open_flags = c_str!("a");
		let stream = unsafe { fdopen(file_descriptor, open_flags.as_ptr()) };
		if stream.is_null()
		{
			return Err(io::Error::last_os_error());
		}
		
		self.print_information_to_stream(stream);
		
		match unsafe { fclose(stream) }
		{
			0 => Ok(()),
			EOF => Err(io::Error::last_os_error()),
			unexpected @ _ => panic!("Unexpected result code '{}' from 'fclose'", unexpected),
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn print_information_to_stream(&self, stream: *mut FILE);
}
