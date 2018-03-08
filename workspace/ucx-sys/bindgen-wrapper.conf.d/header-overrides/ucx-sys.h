// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#include "uct/api/uct.h"
#include "ucm/api/ucm.h"
#include "ucp/api/ucp.h"
#include "ucs/config/global_opts.h"

// Included contents of "ucm/config/ucm_config.h"
typedef struct ucm_config {
    ucs_log_level_t log_level;
    int             enable_events;
    int             enable_mmap_reloc;
    int             enable_malloc_hooks;
    int             enable_malloc_reloc;
    int             enable_dynamic_mmap_thresh;
#if HAVE_CUDA
    int             enable_cuda_hooks;
#endif
    size_t          alloc_alignment;
} ucm_config_t;


extern ucm_config_t ucm_global_config;
