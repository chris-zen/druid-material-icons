#!/bin/bash

set -e

pushd generate-icons
cargo run
popd
cargo check
