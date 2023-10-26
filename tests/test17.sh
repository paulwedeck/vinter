#!/bin/bash

rm -f ../results/test17

TEMP=$(mktemp -p $TEST_ROOT/temp)

qemu-img create -f qcow2 ${TEMP} 1G


(cd $TEST_ROOT/vinter_pre; ./target/release/vinter_trace.py --run ""  --qcow ${TEMP} ./fs-testing/scripts/vm_winefs.yaml)


for i in `seq 20`; do

(cd $TEST_ROOT/vinter_pre; /usr/bin/time --format="%e %S %U" ./target/release/vinter_trace.py --run "" \
	--qcow ${TEMP} --load-snapshot boot \
	./fs-testing/scripts/vm_winefs.yaml) > /dev/null 2>> ../results/test17
done

rm ${TEMP}
