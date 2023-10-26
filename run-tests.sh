#!/bin/bash
export TEST_ROOT=$(realpath .)

mkdir results
cd tests
for test in `ls test*.sh`;
do
./$test
done
