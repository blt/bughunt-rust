#!/usr/bin/env bash

BUILD_DIR=target/debug/
STR_REPEAT=${BUILD_DIR}/str_repeat

echo ${TRAVIS_BUILD_NUMBER}
file ${STR_REPEAT}
