#!/usr/bin/env bash

source ci/common.sh

BUILD_DIR=fuzz/target/x86_64-unknown-linux-gnu/debug/

gcloud auth activate-service-account --key-file ci/auth.json
for TEST in ${TESTS}
do
    TRGT=${BUILD_DIR}${TEST}.tar.gz
    SRCMAP=${BUILD_DIR}${TEST}-${TRAVIS_BUILD_NUMBER}.srcmap.json
    echo "{\"rustc\": {\"type\": \"git\", \"url\": \"https://github.com/rust-lang/rust.git\", \"rev\": \"`rustc --version | awk '{print substr(\$3,2)}'`\"}}" | python -m json.tool > ${SRCMAP}
    tar zcf ${TRGT} ${BUILD_DIR}${TEST} ${SRCMAP}
    gsutil cp ${TRGT} gs://builds.bughunt.appspot.com/${TEST}/${TEST}-${TRAVIS_BUILD_NUMBER}.tar.gz
done
