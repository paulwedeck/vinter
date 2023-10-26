TEMPDIR=$(mktemp -d -p $TEST_ROOT/temp)

cd $1;
realpath ./fs-testing/scripts/test_* | xargs -n1 -P 32 ./target/release/vinter_trace2img analyze --output-dir ${TEMPDIR} ./fs-testing/scripts/vm_winefs.yaml

rm -rf ${TEMPDIR}
