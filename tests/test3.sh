#!/bin/bash

rm -f ../results/test3


for i in `seq 5`; do
./simple_measure.sh "./xargs_all.sh $TEST_ROOT/vinter_pre/" $i 2>> ../results/test3
done
