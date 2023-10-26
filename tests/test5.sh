#!/bin/bash

rm -f ../results/test5

for i in `seq 5`; do
./simple_measure.sh "./xargs_all.sh $TEST_ROOT/vinter_opt1/" $i 2>> ../results/test5
done
