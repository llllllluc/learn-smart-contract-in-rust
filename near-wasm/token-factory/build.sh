#!/bin/bash
set -e

pushd fungible-token
./build.sh
popd

pushd factory
./build.sh
popd
