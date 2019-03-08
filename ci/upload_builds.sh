#!/usr/bin/env bash

BUILD_DIR=target/debug/

gcloud auth activate-service-account --key-file ci/auth.json
for TEST in str_repeat
do
    TRGT=${BUILD_DIR}${TEST}.tar.gz
    tar zcf ${TRGT} ${BUILD_DIR}${TEST}
    gsutil cp ${TRGT} gs://builds.bughunt.appspot.com/${TEST}/${TEST}-${TRAVIS_BUILD_NUMBER}.tar.gz
done
