#!/bin/bash

cd $TEST_ROOT/vinter_pre
/usr/bin/time --format="${2} %e %S %U" ./target/release/vinter_trace.py --run "$@" ./fs-testing/scripts/vm_winefs.yaml > /dev/null
