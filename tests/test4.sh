#!/bin/bash
rm -f $TEST_ROOT/results/test4/*
mkdir -p $TEST_ROOT/results/test4


./simple_repeat.sh "./simple_all.sh $TEST_ROOT/vinter_opt1/" 5 $TEST_ROOT/results/test4
