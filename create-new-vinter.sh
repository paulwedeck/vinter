#!/bin/bash
cp -ra vinter_base vinter_$1
cd vinter_$1
git checkout $1
git submodule init
git submodule update
./build-panda.sh
./build-vinter.sh
cp -ra ../WineFS/Linux-5.1 fs-testing/linux/winefs
./fs-testing/linux/build-kernel.sh winefs
