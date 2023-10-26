#!/bin/bash

rm -f ../results/test12


for i in `seq 20`; do
./panda_run.sh "" 2>> ../results/test12
done
