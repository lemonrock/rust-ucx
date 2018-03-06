// This file is part of ucx. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT. No part of ucx, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of ucx. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx/master/COPYRIGHT.


pub type uct_alloc_method_t = u32;

pub const UCT_ALLOC_METHOD_DEFAULT: uct_alloc_method_t = 5;
pub const UCT_ALLOC_METHOD_HEAP: uct_alloc_method_t = 2;
pub const UCT_ALLOC_METHOD_HUGE: uct_alloc_method_t = 4;
pub const UCT_ALLOC_METHOD_MD: uct_alloc_method_t = 1;
pub const UCT_ALLOC_METHOD_MMAP: uct_alloc_method_t = 3;
pub const UCT_ALLOC_METHOD_THP: uct_alloc_method_t = 0;
