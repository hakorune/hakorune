#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TARGET_DIR="${ROOT_DIR}/target/release"
STAMP="${TARGET_DIR}/.perf_release_sync"

cd "${ROOT_DIR}"

echo "[perf-release] building nyash_kernel static runtime"
cargo build --release -p nyash_kernel

echo "[perf-release] building ny-llvmc release binary"
cargo build --release --bin ny-llvmc -p nyash-llvm-compiler

echo "[perf-release] building boundary FFI library"
bash tools/build_hako_llvmc_ffi.sh

echo "[perf-release] building hakorune release binary"
cargo build --release --bin hakorune -p nyash-rust

touch "${STAMP}"
echo "[perf-release] sync stamp updated: ${STAMP}"
