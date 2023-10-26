#!/bin/bash
rm -f $TEST_ROOT/results/test10/*
mkdir -p $TEST_ROOT/results/test10


./simple_repeat.sh "./mod1_all.sh $TEST_ROOT/vinter_pre_mod1/" 15 $TEST_ROOT/results/test10
