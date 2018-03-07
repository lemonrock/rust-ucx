// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


use ::libc::c_void;
use ::libc::EOF;
use ::libc::FILE;
use ::libc::fdopen;
use ::libc::fclose;
use ::libc::fflush;
use ::libc::free;
use ::libc_extra::android_linux::stdio::open_memstream;
use ::libc_extra::stderr;
use ::libc_extra::stdout;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::fs::File;
use ::std::io;
use ::std::mem::uninitialized;
use ::std::os::unix::io::IntoRawFd;



include!("PrintInformation.rs");
