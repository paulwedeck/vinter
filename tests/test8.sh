#!/bin/bash
rm -rf $TEST_ROOT/results/test8
mkdir -p $TEST_ROOT/results/test8

./simple_repeat.sh "./output_multiple.sh $TEST_ROOT/vinter_pre/ test_update-middle test_rename-long-name test_touch-long-name test_hello-world" 20 $TEST_ROOT/results/test8
