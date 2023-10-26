#!/bin/bash
rm -f $TEST_ROOT/results/test1/*
mkdir -p $TEST_ROOT/results/test1


./simple_repeat.sh "./simple_all.sh $TEST_ROOT/vinter_pre/" 20 $TEST_ROOT/results/test1
