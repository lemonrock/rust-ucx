# This file is part of ucx-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx-sys/master/COPYRIGHT. No part of ucx-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of ucx-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/ucx-sys/master/COPYRIGHT.


compile_library_name='ucx'

compile_library()
{
	compile_autoreconf()
	{
		cd "$rootOutputFolderPath" 1>/dev/null 2>/dev/null

			autoreconf -ivf

		cd - 1>/dev/null 2>/dev/null
	}

	compile_configure()
	{
		cd "$rootOutputFolderPath" 1>/dev/null 2>/dev/null

			# 			CFLAGS="-D_GNU_SOURCE -isystem${DEP_LIBNUMA_ROOT}/include -isystem${DEP_RDMA_CORE_ROOT}/include -L${DEP_LIBNUMA_ROOT}/lib -L${DEP_RDMA_CORE_ROOT}/lib" \

			# -rpath or -rpath-link for rdmacm - can't find libibverbs, so need LIBS=  - config.log

			# --with-rte(=SYSROOT) - probably needed for Mellanox stuff, ie DPDK.
			# --disable-openmp
			# --enable-tuning
			# --with-ugni(=SYSROOT)
			CPPFLAGS="-D_GNU_SOURCE -isystem${DEP_LIBNUMA_ROOT}/include -isystem${DEP_RDMA_CORE_ROOT}/include" \
			CFLAGS="-mavx -msse4.1 -msse4.2" \
			LDFLAGS="-L${DEP_LIBNUMA_ROOT}/lib -L${DEP_RDMA_CORE_ROOT}/lib" \
			LIBS="-libverbs" \
			./contrib/configure-release-mt --prefix=/usr --host="$configureHost" --disable-shared --enable-static --disable-dependency-tracking --disable-silent-rules --enable-fast-install \
				--enable-compiler-opt=3 \
				--disable-backtrace-detail \
				--disable-symbol-override \
				--disable-cma \
				--disable-frame-pointer \
				--disable-fault-injection \
				--disable-debug-data \
				--disable-doxygen-doc \
				--disable-doxygen-man \
				--disable-doxygen-html \
				--disable-doxygen-pdf \
				--with-sse41 \
				--with-sse42 \
				--with-cache-line-size=128 \
				--with-allocator=ptmalloc286 \
				--with-verbs="$DEP_RDMA_CORE_ROOT" \
				--with-rc \
				--with-ud \
				--with-dc \
				--with-cm \
				--with-mlx5-hw \
				--with-ib-hw-tm \
				--with-dm \
				--with-rdmacm="$DEP_RDMA_CORE_ROOT" \
				--with-mpi=no \
				--with-java=no \
				--with-cuda=no \
				--with-gdrcopy=no \
				--with-rocm=no \
				--with-knem=no \
				--with-xpmem=no

		cd - 1>/dev/null 2>/dev/null
	}

	compile_make()
	{
		cd "$rootOutputFolderPath" 1>/dev/null 2>/dev/null

			make -j "$numberOfMakeJobs" 1>&2
			make -j "$numberOfMakeJobs" install DESTDIR="$rootOutputFolderPath" 1>&2

		cd - 1>/dev/null 2>/dev/null
	}

	if [ -z "${DEP_RDMA_CORE_ROOT+is_unset}" ]; then
		compile_fail 'Please specify the environment variable DEP_RDMA_CORE_ROOT which must point to a sys-root folder path containing an include and a lib folder'
	else
		local rdmaCoreSystemRootFolderPath="$DEP_RDMA_CORE_ROOT"
	fi

	if [ -z "${DEP_LIBNUMA_ROOT+is_unset}" ]; then
		compile_fail 'Please specify the environment variable DEP_LIBNUMA_ROOT which must point to a sys-root folder path containing an include and a lib folder'
	else
		local numaSystemRootFolderPath="$DEP_LIBNUMA_ROOT"
	fi

	compile_autoreconf 2>&1

	compile_configure 2>&1

	compile_make 2>&1
}

cargo_key_value_pairs()
{
	cargo_key_value_pairs_link_lib 'static-nobundle' ucm
	cargo_key_value_pairs_link_lib 'static-nobundle' ucs
	cargo_key_value_pairs_link_lib 'static-nobundle' ucp
	cargo_key_value_pairs_link_lib 'static-nobundle' uct

	# Search path
	cargo_key_value_pairs_search 'native' "$OUT_DIR"/root/usr/lib

	# Not used by us, but potentially used by downstream crates.
	cargo_key_value_pairs_other 'root' "$OUT_DIR"/root
	cargo_key_value_pairs_other 'include' "$OUT_DIR"/root/usr/include
	cargo_key_value_pairs_other 'libdir' "$OUT_DIR"/root/usr/lib
}
