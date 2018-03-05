#@IgnoreInspection BashAddShebang
# This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


bindingsName='ucx-sys'
rootIncludeFileName='ucx-sys.h'
macosXHomebrewPackageNames='clang-format'
alpineLinuxPackageNames='rsync make gcc linux-headers libunwind-dev linux-grsec-dev'
bindgenAdditionalArguments="--no-prepend-enum-name"
clangAdditionalArguments='-D_GNU_SOURCE'
headersFolderPath='usr/include'
libFolderPath='usr/include'
link='ucx'
link_kind='static-nobundle'

final_chance_to_tweak()
{
	sed -i -e 's/#\[derive(Debug, Default, Copy)\]/#[derive(Copy)]/g' "$outputFolderPath"/structs/uct_md_attr.rs
	
	_fix_bitfield()
	{
		local constant_prefix="$1"
		local bitfield_struct="$2"
		
		sed -i -e 's/(pub /(/g' "$outputFolderPath"/structs/"$bitfield_struct".rs
		
		{
			printf '\nimpl %s\n' "$bitfield_struct"
			printf '{\n'
			
			grep "^pub const $constant_prefix" "$outputFolderPath"/constants/miscellany.rs | \
				sed \
					-e 's/^pub const '"$constant_prefix"'/\tpub const /g' \
					-e 's/: '"$bitfield_struct"'/: Self/g' \
			
			printf '}\n'
		} >>"$outputFolderPath"/structs/"$bitfield_struct".rs
		
		grep -v "^pub const $constant_prefix" "$outputFolderPath"/constants/miscellany.rs >"$outputFolderPath"/constants/miscellany.rs.tmp
		rm "$outputFolderPath"/constants/miscellany.rs
		mv "$outputFolderPath"/constants/miscellany.rs.tmp "$outputFolderPath"/constants/miscellany.rs
		
		# TODO: Add BitOr operations
	}
	
	_fix_bitfield UCM_EVENT_ ucm_event
	_fix_bitfield UCP_ATTR_FIELD_ ucp_attr_field
	_fix_bitfield UCP_EP_PARAM_FIELD_ ucp_ep_params_field
	_fix_bitfield UCP_FEATURE_ ucp_feature
	_fix_bitfield UCP_ATTR_FIELD_REQUEST_SIZE ucp_context_attr_field
	_fix_bitfield UCP_LISTENER_PARAM_FIELD_ ucp_listener_params_field
	_fix_bitfield UCP_MEM_ADVISE_PARAM_FIELD_ ucp_mem_advise_params_field
	_fix_bitfield UCP_MEM_ATTR_FIELD_ ucp_mem_attr_field
	_fix_bitfield UCP_PARAM_FIELD_ ucp_params_field
	_fix_bitfield UCP_WAKEUP_ ucp_wakeup_event_types
	_fix_bitfield UCP_WORKER_ATTR_FIELD_ ucp_worker_attr_field
	_fix_bitfield UCP_WORKER_PARAM_FIELD_ ucp_worker_params_field
	_fix_bitfield UCS_CONFIG_PRINT_ ucs_config_print_flags_t
	_fix_bitfield UCT_CB_FLAG_ uct_cb_flags
	_fix_bitfield UCT_CB_PARAM_FLAG_ uct_cb_param_flags
	_fix_bitfield UCT_EVENT_ uct_iface_event_types
	_fix_bitfield UCT_FLUSH_FLAG_ uct_flush_flags
	_fix_bitfield UCT_IFACE_OPEN_MODE_ uct_iface_open_mode
	_fix_bitfield UCT_MD_MEM_ACCESS_ uct_md_mem_flags
	_fix_bitfield UCT_PROGRESS_ uct_progress_types
	_fix_bitfield UCT_SEND_FLAG_ uct_msg_flags
}
