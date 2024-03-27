#!/bin/bash

set -ex

BASEDIR=$(cd $(dirname $0); pwd)

$BASEDIR/run_catbuffer_generator.sh
$BASEDIR/run_testvectors_generator.sh
cargo test --release
cargo test --release --all-features