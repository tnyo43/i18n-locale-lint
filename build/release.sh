#!/bin/bash

RELEASE=true sh build/build.sh

echo run "npm pack"
npm pack

echo if it is ok to publish, run "npm publish"
