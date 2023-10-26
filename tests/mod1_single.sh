#!/bin/bash

TEMPDIR=$(mktemp -d -p $TEST_ROOT/temp)

NAME=${3##*/}
NAME=${NAME%%.*}
#echo $NAME

OUT=$(cd $1; ./target/release/vinter_trace2img analyze --output-dir $TEMPDIR $2 $3 | grep benchout)

echo $NAME ${OUT#* }

rm -rf $TEMPDIR
