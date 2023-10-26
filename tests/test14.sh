#!/bin/bash

rm -f ../results/test14


for i in `seq 0 500 100000`; do
OUT=$(./panda2_run.sh "mount -t winefs -o init /dev/pmem0 /mnt; fallocate -l $i /mnt/out; fs-dump --contents /mnt/out" 2>&1)

echo $i $OUT >> ../results/test14
done
