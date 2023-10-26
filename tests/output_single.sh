#!/bin/bash

TEMPDIR=$(mktemp -d -p $TEST_ROOT/temp)

NAME=${3##*/}
NAME=${NAME%%.*}
#echo $NAME

TEMPFILE=$(mktemp -p $TEST_ROOT/temp)

SCRIPT=$(realpath ./output_measure.sh)

out=$(cd $1; $SCRIPT "./target/release/vinter_trace2img analyze --output-dir $TEMPDIR $2 $3" $NAME 2>$TEMPFILE)
err=$(<$TEMPFILE)

out=${out##*, }
out=${out%% *}

echo ${err} ${out}

rm -rf $TEMPDIR $TEMPFILE
