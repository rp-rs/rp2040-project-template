#!/bin/bash

bin="$PWD/$1"
shift

cd ../probe-run
cargo run -- "$bin" "$@" --chip RP2040
