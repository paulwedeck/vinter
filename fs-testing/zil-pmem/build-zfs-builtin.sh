#!/usr/bin/env bash

set -eux

kernel=linux
build=${kernel}_build
kernel_abs=$PWD/$kernel
build_abs=$PWD/$build
zfs=openzfs

# see https://github.com/openzfs/zfs/issues/10450#issuecomment-643654436

make -C"$kernel" O="../$build" defconfig

# Enable ZFS and additional dependencies.
cat >>"$build/.config" <<EOF
CONFIG_CRYPTO_DEFLATE=y
CONFIG_ZLIB_DEFLATE=y
CONFIG_KALLSYMS=y
CONFIG_EFI_PARTITION=y
CONFIG_ZFS=y

CONFIG_DAX=y
CONFIG_DEV_DAX=y
CONFIG_DEV_DAX_PMEM=y
CONFIG_FS_DAX=y
CONFIG_LIBNVDIMM=y
CONFIG_BLK_DEV_PMEM=y
CONFIG_X86_PMEM_LEGACY=y

CONFIG_RANDOMIZE_BASE=n
CONFIG_SMP=n
CONFIG_CPU_FREQ=n
CONFIG_RETPOLINE=n
EOF

make -C"$kernel" O="../$build" prepare

# Configure ZFS and add it to the kernel tree.
pushd "$zfs"
[[ -f ./configure ]] || sh ./autogen.sh
./configure --enable-linux-builtin --with-linux="$kernel_abs" --with-linux-obj="$build_abs"
./copy-builtin "$kernel_abs"
popd

# Disable modules.
sed -i 's/CONFIG_MODULES=y/CONFIG_MODULES=n/' "$build"/.config

make -C"$kernel" O="../$build" -j$(nproc) bzImage

