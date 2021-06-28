#!/bin/bash
#set -x #echo on

# All targets
TARGETS=$(ls src/bin/ | awk '{print substr($$0, 1, length($$0)-3)}')

# Exit upon seeing the first crash (default: false)
HFUZZ_ARGS="-N 400000 --run_time 4 --exit_upon_crash"


# List all targets
for target in $TARGETS
do
	# show the target
	echo $target

	# dry-run 
	eval 'export HFUZZ_RUN_ARGS=$HFUZZ_ARGS'
	cargo +nightly hfuzz run $target
done
