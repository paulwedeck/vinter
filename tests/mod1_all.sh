#!/bin/bash

for i in `realpath $1/fs-testing/scripts/test_*`; do
./mod1_single.sh $1 ./fs-testing/scripts/vm_winefs.yaml $i
done
