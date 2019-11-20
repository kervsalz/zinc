#!/bin/bash -Cex

# 'error' | 'warn' | 'info' | 'debug' | 'trace'
export LOG_LEVEL="${1}"

export LIBRARY_NAME='zrust-compiler'
export LIBRARY_NAME_LOG='zrust_compiler'
export BINARY_NAME='zrustc'
export BUILD_MODE='debug'
export RUST_LOG="${LIBRARY_NAME_LOG}=${LOG_LEVEL},${BINARY_NAME}=${LOG_LEVEL}"
export RUST_BACKTRACE=1

# *.zrs
export INPUT="${2}"

# *.zrsb
export OUTPUT="${3}"

cargo fmt --package "${LIBRARY_NAME}"
cargo test --package "${LIBRARY_NAME}"
cargo build --package "${LIBRARY_NAME}"

"./target/${BUILD_MODE}/${BINARY_NAME}" --input "${INPUT}" --output "${OUTPUT}"
