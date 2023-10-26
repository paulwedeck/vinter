#!/bin/bash

rm -f ../results/test7

for i in `seq 5`; do
./core_multi.sh "$TEST_ROOT/vinter_opt2/" $i 40 2>> ../results/test7
done
