#!/bin/bash

TEMPDIR=$(mktemp -d -p $TEST_ROOT/temp)

NAME=${3##*/}
NAME=${NAME%%.*}
#echo $NAME

SCRIPT=$(realpath ./simple_measure.sh)
cd $1
$SCRIPT "./target/release/vinter_trace2img analyze --output-dir $TEMPDIR $2 $3" $NAME

rm -rf $TEMPDIR
