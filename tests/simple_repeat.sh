#!/bin/bash

for i in `seq $2`; do
$1 2>&1 >> $3/$i 2>&1
done
