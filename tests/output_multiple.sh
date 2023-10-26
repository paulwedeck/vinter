#!/bin/bash

TESTS=${@:2}

for test in $TESTS; do
./output_single.sh $1 ./fs-testing/scripts/vm_winefs.yaml ./fs-testing/scripts/$test.yaml
done
