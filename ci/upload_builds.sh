#!/usr/bin/env bash

BUILD_DIR=target/debug/
STR_REPEAT=${BUILD_DIR}/str_repeat

echo ${TRAVIS_BUILD_NUMBER}
file ${STR_REPEAT}

gcloud auth activate-service-account --key-file ci/auth.json
gsutil cp ${STR_REPEAT} gs://builds.bughunt.appspot.com/str_repeat/${TRAVIS_BUILD_NUMBER}_str_repeat
