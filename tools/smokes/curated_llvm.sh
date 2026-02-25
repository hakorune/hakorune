#!/usr/bin/env bash
set -euo pipefail

# Curated LLVM smoke runner (llvmlite harness)
if [[ "${NYASH_CLI_VERBOSE:-0}" == "1" ]]; then set -x; fi

# Usage:
#   tools/smokes/curated_llvm.sh [--phi-off] [--with-if-merge] [--with-loop-prepass]
# Notes:
#   - Default is PHI-on (MIR14) with harness on.
#   - `--phi-off` switches to the legacy edge-copy mode.
#   - Flags are independent and can be combined.

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
BIN="$ROOT_DIR/target/release/hakorune"

# Ensure release binary with LLVM harness feature.
if ! [ -x "$BIN" ]; then
  echo "[curated-llvm] building hakorune (release, features=llvm)" >&2
  cargo build --release -p nyash-rust --features llvm --bin hakorune >/dev/null
fi

if ! [ -x "$ROOT_DIR/target/release/ny-llvmc" ]; then
  echo "[curated-llvm] building ny-llvmc (release)" >&2
  cargo build --release -p nyash-llvm-compiler >/dev/null
fi

if ! [ -x "$ROOT_DIR/target/release/nyash_kernel" ]; then
  echo "[curated-llvm] building nyash_kernel (release)" >&2
  cargo build --release -p nyash_kernel >/dev/null
fi

export NYASH_LLVM_USE_HARNESS=1

# Defaults
export NYASH_MIR_NO_PHI=${NYASH_MIR_NO_PHI:-0}
export NYASH_VERIFY_ALLOW_NO_PHI=${NYASH_VERIFY_ALLOW_NO_PHI:-0}
unset NYASH_LLVM_PREPASS_IFMERGE || true
unset NYASH_LLVM_PREPASS_LOOP || true

WITH_IFMERGE=0
WITH_LOOP=0

# Parse flags
for arg in "$@"; do
  case "$arg" in
    --phi-off)
      export NYASH_MIR_NO_PHI=1
      export NYASH_VERIFY_ALLOW_NO_PHI=1
      echo "[curated-llvm] PHI-off (edge-copy legacy) enabled" >&2
      ;;
    --phi-on)
      export NYASH_MIR_NO_PHI=0
      export NYASH_VERIFY_ALLOW_NO_PHI=0
      echo "[curated-llvm] PHI-on (SSA builder) enforced" >&2
      ;;
    --with-if-merge)
      WITH_IFMERGE=1
      export NYASH_LLVM_PREPASS_IFMERGE=1
      echo "[curated-llvm] if-merge prepass enabled" >&2
      ;;
    --with-loop-prepass)
      WITH_LOOP=1
      export NYASH_LLVM_PREPASS_LOOP=1
      echo "[curated-llvm] loop prepass enabled" >&2
      ;;
    -h|--help)
      echo "Usage: $0 [--phi-off] [--with-if-merge] [--with-loop-prepass]"; exit 0 ;;
  esac
done

if [[ "${NYASH_MIR_NO_PHI}" == "0" ]]; then
  echo "[curated-llvm] PHI-on (SSA builder) running" >&2
else
  echo "[curated-llvm] PHI-off (edge-copy legacy) active" >&2
fi

run() {
  local path="$1"
  echo "[curated-llvm] RUN --backend llvm: ${path}"
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

# Core minimal (existing harness sample)
run "$ROOT_DIR/examples/llvm11_core_smoke.hako"

# Async/await (LLVM only)
run "$ROOT_DIR/apps/tests/async-await-min/main.hako"
run "$ROOT_DIR/apps/tests/async-spawn-instance/main.hako"
NYASH_AWAIT_MAX_MS=100 run "$ROOT_DIR/apps/tests/async-await-timeout-fixed/main.hako"

# Control-flow: loop+if phi pattern and representative nested-loop depth1 case
#
# NOTE: legacy fixtures `nested_loop_inner_{break,continue}_isolated.hako` are no longer accepted
# by the current planner (generic loop v0 has no nested-loop plan). Use the phase29bq
# minimal cases that are kept green by the fast gate instead.
run "$ROOT_DIR/apps/tests/loop_if_phi.hako"
run "$ROOT_DIR/apps/tests/phase29bq_loop_cond_break_continue_nested_loop_depth1_min.hako"

# Peek expression
run "$ROOT_DIR/apps/tests/peek_expr_block.hako"

# Try/cleanup control-flow:
# Currently not supported by JoinIR lowering on this branch (fail-fast freeze).

# Optional: if-merge (ret-merge) tests
if [[ "$WITH_IFMERGE" == "1" ]]; then
  run "$ROOT_DIR/apps/tests/ternary_basic.hako"
  run "$ROOT_DIR/apps/tests/ternary_nested.hako"
fi

echo "[curated-llvm] OK"
