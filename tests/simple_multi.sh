#!/bin/bash

TEMPDIR=$(mktemp -d -p $TEST_ROOT/temp)

SCRIPT=$(realpath ./simple_measure.sh)
cd $1;
$SCRIPT "./target/release/vinter_trace2img analyze --output-dir $TEMPDIR ./fs-testing/scripts/vm_winefs.yaml $(echo $(realpath ./fs-testing/scripts/test_*))" $2

rm -rf $TEMPDIR
