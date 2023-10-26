#!/bin/bash

rm -f ../results/test18


for i in `seq 32`; do

SIZE=$(bc -l <<< "e(l(10)*($i/8))" )
SIZE=$(printf "%.0f" $SIZE)
#echo $SIZE

OUT=$(./panda_run.sh "mount -t winefs -o init /dev/pmem0 /mnt; fallocate -l ${SIZE} /mnt/out; fs-dump --contents /mnt/out" 2>&1)

echo $SIZE $OUT >> ../results/test18
done
