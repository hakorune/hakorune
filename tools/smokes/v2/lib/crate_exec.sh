#!/usr/bin/env bash
# crate_exec.sh — tiny helpers to build and run ny-llvmc EXE reps with bounded time

set -euo pipefail

: "${HAKO_BUILD_TIMEOUT:=10}"
: "${HAKO_EXE_TIMEOUT:=5}"

crate_build_exe() {
  # Args: in_json out_exe [nyrt_dir]
  local in_json="$1"; shift
  local out_exe="$1"; shift
  local nyrt_dir="${1:-$NYASH_ROOT/target/release}"
  local bin_nyllvmc="${NYASH_NY_LLVM_COMPILER:-$NYASH_ROOT/target/release/ny-llvmc}"
  timeout "$HAKO_BUILD_TIMEOUT" "$bin_nyllvmc" --in "$in_json" --emit exe --nyrt "$nyrt_dir" --out "$out_exe" >/dev/null 2>&1
}

crate_run_exe() {
  # Args: exe_path
  local exe="$1"
  timeout "$HAKO_EXE_TIMEOUT" "$exe" >/dev/null 2>&1
}

