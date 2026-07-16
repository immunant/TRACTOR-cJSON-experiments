#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
rust_library="$root_dir/rust/target/debug/libcjson.so"
test_build_dir="$root_dir/build-rust-tests"

export RUSTC_BOOTSTRAP=1
cargo build --manifest-path "$root_dir/rust/Cargo.toml"

cmake \
    -S "$root_dir/tests" \
    -B "$test_build_dir" \
    -DCJSON_RUST_LIBRARY="$rust_library"
cmake --build "$test_build_dir" --parallel
ctest --test-dir "$test_build_dir" --output-on-failure
