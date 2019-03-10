#!/usr/bin/env bash
set -o errexit
set -o pipefail
set -o nounset

source ci/common.sh

for TEST in ${TESTS}
do
    # 'cargo fuzz' doesn't have a build and I haven't investigated how to build
    # without the guiding hand of the tool. Instead, fuzz each test for one
    # second which, incidentally, builds the target.
    #
    # This is a silly hack.
    cargo fuzz run ${TEST} -- -max_total_time=1
done
