#!/bin/bash

for i in `realpath $1/fs-testing/scripts/test_*`; do
./simple_single.sh $1 ./fs-testing/scripts/vm_winefs.yaml $i
done
