#!/usr/bin/env bash
set -euo pipefail

# Curated LLVM Stage-3 acceptance smoke (llvmlite harness)
# Usage: tools/smokes/curated_llvm_stage3.sh [--phi-off]

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
BIN="$ROOT_DIR/target/release/hakorune"

# Ensure release binary with LLVM harness feature.
if ! [ -x "$BIN" ]; then
  echo "[curated-llvm-stage3] building hakorune (release, features=llvm)" >&2
  cargo build --release -p nyash-rust --features llvm --bin hakorune >/dev/null
fi

if ! [ -x "$ROOT_DIR/target/release/ny-llvmc" ]; then
  echo "[curated-llvm-stage3] building ny-llvmc (release)" >&2
  cargo build --release -p nyash-llvm-compiler >/dev/null
fi

if ! [ -x "$ROOT_DIR/target/release/nyash_kernel" ]; then
  echo "[curated-llvm-stage3] building nyash_kernel (release)" >&2
  cargo build --release -p nyash_kernel >/dev/null
fi

export NYASH_LLVM_USE_HARNESS=1
# Accept Stage-3 surface in Rust parser for these inputs
export NYASH_FEATURES=stage3
# Lower try/throw using Result-style structured blocks (MVP path)
export NYASH_TRY_RESULT_MODE=1

if [[ "${1:-}" == "--phi-off" ]]; then
  export NYASH_MIR_NO_PHI=1
  export NYASH_VERIFY_ALLOW_NO_PHI=1
  echo "[curated-llvm-stage3] PHI-off (edge-copy) enabled" >&2
fi

run_ok() {
  local path="$1"
  echo "[curated-llvm-stage3] RUN --backend llvm: ${path}"
  # Program return value becomes the process exit code; non-zero does not mean failure.
  # Treat it as PASS if the LLVM harness completed successfully.
  local output=""
  output=$(timeout 10s "$BIN" --backend llvm "$path" 2>&1 || true)
  if echo "$output" | grep -q "LLVM (harness) execution completed"; then
    return 0
  fi
  echo "$output" >&2
  return 1
}

# A) try/catch/cleanup without actual throw
run_ok "$ROOT_DIR/apps/tests/stage3_try_finally_basic.hako"

# B) throw in dead branch (acceptance only)
run_ok "$ROOT_DIR/apps/tests/stage3_throw_dead_branch.hako"

# C) repeat with trap disabled (robustness; should be no-op here)
NYASH_LLVM_TRAP_ON_THROW=0 run_ok "$ROOT_DIR/apps/tests/stage3_try_finally_basic.hako"
NYASH_LLVM_TRAP_ON_THROW=0 run_ok "$ROOT_DIR/apps/tests/stage3_throw_dead_branch.hako"

echo "[curated-llvm-stage3] OK"
