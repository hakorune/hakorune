#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
SYNC_STAMP="${ROOT_DIR}/target/release/.perf_observe_release_sync"

cd "${ROOT_DIR}"

echo "[perf-observe] building nyash_kernel static runtime"
cargo build --release -p nyash_kernel --features perf-observe

echo "[perf-observe] building hakorune driver"
cargo build --release --bin hakorune --features perf-observe

touch "${SYNC_STAMP}"

echo "[perf-observe] aligned artifacts:"
stat -c '%y %n' \
  "${ROOT_DIR}/target/release/libnyash_kernel.a" \
  "${ROOT_DIR}/target/release/hakorune" \
  "${SYNC_STAMP}"
