#!/bin/bash

rm -f ../results/test6

for i in `seq 5`; do
./simple_multi.sh "$TEST_ROOT/vinter_opt2/" $i 2>> ../results/test6
done
