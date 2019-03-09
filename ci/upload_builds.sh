#!/usr/bin/env bash

source common.sh

BUILD_DIR=fuzz/target/x86_64-apple-darwin/debug/

gcloud auth activate-service-account --key-file ci/auth.json
for TEST in ${TESTS}
do
    TRGT=${BUILD_DIR}${TEST}.tar.gz
    tar zcf ${TRGT} ${BUILD_DIR}${TEST}
    gsutil cp ${TRGT} gs://builds.bughunt.appspot.com/${TEST}/${TEST}-${TRAVIS_BUILD_NUMBER}.tar.gz
done
