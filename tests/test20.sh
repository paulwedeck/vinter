#!/bin/bash

rm -f ../results/test20


for i in `seq 72`; do

SIZE=$(bc -l <<< "e(l(10)*($i/8))" )
SIZE=$(printf "%.0f" $SIZE)
#echo $SIZE

OUT=$(./panda2_run.sh "mount -t winefs -o init /dev/pmem0 /mnt; fallocate -l ${SIZE} /mnt/out; fs-dump --contents /mnt/out" 2>&1)

echo $SIZE $OUT >> ../results/test20
done
