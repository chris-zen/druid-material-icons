#!/bin/bash

set -e

pushd generate-material-icons
cargo run
popd
mv generate-material-icons/icons.rs src/icons.rs.in
cargo check
