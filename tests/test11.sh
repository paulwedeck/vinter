#!/bin/bash
rm -f $TEST_ROOT/results/test11/*
mkdir -p $TEST_ROOT/results/test11


./simple_repeat.sh "./mod1_all.sh $TEST_ROOT/vinter_opt1_mod1/" 15 $TEST_ROOT/results/test11
