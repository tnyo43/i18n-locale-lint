#!/bin/bash

RELEASE=true sh build/build.sh || { exit 1; }

echo run "npm pack"
npm pack

echo if it is ok to publish, run "npm publish"
