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
link='ucm ucs ucp uct'
link_kind='static-nobundle'

final_chance_to_tweak()
{
	sed -i -e 's/#\[derive(Debug, Default, Copy)\]/#[derive(Copy)]/g' "$outputFolderPath"/structs/uct_md_attr.rs


	_fix_type()
	{
		local constant_prefix="$1"
		local constant_type="$2"

		{
			printf '\n'

			grep "^pub const $constant_prefix" "$outputFolderPath"/constants/miscellany.rs
		} >>"$outputFolderPath"/types/"$constant_type".rs

		grep -v "^pub const $constant_prefix" "$outputFolderPath"/constants/miscellany.rs >"$outputFolderPath"/constants/miscellany.rs.tmp
		rm "$outputFolderPath"/constants/miscellany.rs
		mv "$outputFolderPath"/constants/miscellany.rs.tmp "$outputFolderPath"/constants/miscellany.rs
	}

	_fix_type UCP_ATOMIC_POST_OP_ ucp_atomic_post_op_t
	_fix_type UCP_EP_PARAMS_FLAGS_ ucp_ep_params_flags_field
	_fix_type UCT_ALLOC_METHOD_ uct_alloc_method_t
	_fix_type UCT_MD_MEM_TYPE_ uct_memory_type_t
	_fix_type UCT_SOCKADDR_ACC_ uct_sockaddr_accessibility_t


	_fix_last_in_type()
	{
		local _type="$1"

		grep -v "^pub const .*_LAST" "$outputFolderPath"/types/"$_type".rs >"$outputFolderPath"/types/"$_type".rs.tmp
		rm "$outputFolderPath"/types/"$_type".rs
		mv "$outputFolderPath"/types/"$_type".rs.tmp "$outputFolderPath"/types/"$_type".rs
	}

	_fix_last_in_type ucp_atomic_post_op_t
	_fix_last_in_type uct_alloc_method_t
	_fix_last_in_type uct_memory_type_t


	_fix_bitfield()
	{
		local constant_prefix="$1"
		local bitfield_struct="$2"

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
	}

	_fix_bitfield UCM_EVENT_ ucm_event_type
	_fix_bitfield UCP_EP_PARAM_FIELD_ ucp_ep_params_field
	_fix_bitfield UCP_FEATURE_ ucp_feature
	_fix_bitfield UCP_ATTR_FIELD_ ucp_context_attr_field
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
	_fix_bitfield UCP_MEM_MAP_PARAM_FIELD_ ucp_mem_map_params_field
	_fix_bitfield UCS_CALLBACKQ_FLAG_ ucs_callbackq_flags
	_fix_bitfield UCT_MD_FLAG_ _bindgen_ty_1
	_fix_bitfield UCP_MEM_MAP_ _bindgen_ty_2
	_fix_bitfield UCT_MD_MEM_FLAG_ uct_md_mem_flags


	_fix_duplicate_enum_constant()
	{
		local constant_prefix="$1"
		local enum="$2"

		{
			printf '\nimpl %s\n' "$enum"
			printf '{\n'

			grep "^pub const $constant_prefix" "$outputFolderPath"/constants/miscellany.rs | \
				sed \
					-e 's/^pub const '"$constant_prefix"'/\tpub const /g' \
					-e 's/: '"$enum"'/: Self/g' \

			printf '}\n'
		} >>"$outputFolderPath"/enums/"$enum".rs

		grep -v "^pub const $constant_prefix" "$outputFolderPath"/constants/miscellany.rs >"$outputFolderPath"/constants/miscellany.rs.tmp
		rm "$outputFolderPath"/constants/miscellany.rs
		mv "$outputFolderPath"/constants/miscellany.rs.tmp "$outputFolderPath"/constants/miscellany.rs
	}

	_fix_duplicate_enum_constant UCP_DATATYPE_ ucp_dt_type


	_fix_last_in_enum()
	{
		local enum="$1"

		grep -v "^\t.*_LAST" "$outputFolderPath"/enums/"$enum".rs >"$outputFolderPath"/enums/"$enum".rs.tmp
		rm "$outputFolderPath"/enums/"$enum".rs
		mv "$outputFolderPath"/enums/"$enum".rs.tmp "$outputFolderPath"/enums/"$enum".rs
	}

	_fix_last_in_enum ucp_atomic_fetch_op_t
	_fix_last_in_enum ucs_async_mode_t
	_fix_last_in_enum ucs_handle_error_t
	_fix_last_in_enum ucs_log_level_t
	_fix_last_in_enum ucs_stats_formats_t
	_fix_last_in_enum ucs_ternary_value
	_fix_last_in_enum ucs_thread_mode_t
	_fix_last_in_enum uct_am_trace_type
	_fix_last_in_enum uct_device_type_t
}
