#!/bin/bash
# phase29y_hako_binary_only_selfhost_readiness_vm.sh
# Contract pin (ported, non-gating):
# - repo-outside `hakorune` binary-only route can run a 2-pass MIR emit without stage1 repo dependencies.
# - pass1/pass2 MIR must match under canonical normalization (pre-selfhost N->N+1->N+2 proxy lock).
# - same workdir must run `--backend vm --hako-run` successfully.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/lib/phase29y_binary_only_common.sh"
require_env || exit 2

SMOKE_NAME="phase29y_hako_binary_only_selfhost_readiness_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/phase29y_hako_run_binary_only_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-20}"

cleanup() {
  phase29y_binary_only_cleanup_workdir
}
trap cleanup EXIT

check_stale_binary_only_markers() {
  local output="$1"
  local phase_label="$2"
  if printf '%s\n' "$output" | rg -q '\[stage1-cli\] entry not found:'; then
    phase29y_binary_only_tail_output
    test_fail "$SMOKE_NAME: stale entry-path blocker reappeared ($phase_label)"
    exit 1
  fi
  if printf '%s\n' "$output" | rg -q '\[stage1-cli\] .*timed out after [0-9]+ ms'; then
    phase29y_binary_only_tail_output
    test_fail "$SMOKE_NAME: stale timeout marker reappeared ($phase_label)"
    exit 1
  fi
  if printf '%s\n' "$output" | rg -q "using: failed to read 'lang/src/"; then
    phase29y_binary_only_tail_output
    test_fail "$SMOKE_NAME: stale lang/src read blocker reappeared ($phase_label)"
    exit 1
  fi
}

phase29y_binary_only_require_input_and_bin "$SMOKE_NAME" "$INPUT" || exit 2
phase29y_binary_only_prepare_workdir "$INPUT" "phase29y_hako_binary_only_selfhost_readiness"

# pass1 emit (Stage1 proxy)
phase29y_binary_only_run_in_workdir "$RUN_TIMEOUT_SECS" 1 --hako-emit-mir-json ./stage1.mir ./input.hako
OUTPUT="$PHASE29Y_BINARY_ONLY_OUTPUT"
RC="$PHASE29Y_BINARY_ONLY_RC"
if [ "$RC" -eq 124 ]; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: emit pass1 timeout"
  exit 1
fi
if [ "$RC" -ne 0 ]; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: emit pass1 failed rc=$RC"
  exit 1
fi
check_stale_binary_only_markers "$OUTPUT" "emit-pass1"

# pass2 emit (Stage2 proxy)
phase29y_binary_only_run_in_workdir "$RUN_TIMEOUT_SECS" 1 --hako-emit-mir-json ./stage2.mir ./input.hako
OUTPUT="$PHASE29Y_BINARY_ONLY_OUTPUT"
RC="$PHASE29Y_BINARY_ONLY_RC"
if [ "$RC" -eq 124 ]; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: emit pass2 timeout"
  exit 1
fi
if [ "$RC" -ne 0 ]; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: emit pass2 failed rc=$RC"
  exit 1
fi
check_stale_binary_only_markers "$OUTPUT" "emit-pass2"

if [ ! -s "$PHASE29Y_BINARY_ONLY_WORKDIR/stage1.mir" ]; then
  test_fail "$SMOKE_NAME: stage1 MIR output missing"
  exit 1
fi
if [ ! -s "$PHASE29Y_BINARY_ONLY_WORKDIR/stage2.mir" ]; then
  test_fail "$SMOKE_NAME: stage2 MIR output missing"
  exit 1
fi

CANON_FILTER="$PHASE29Y_BINARY_ONLY_WORKDIR/mir_canon.jq"
cat > "$CANON_FILTER" <<'JQ'
def normalize_blocks:
  map(del(.id, .successors, .reachable, .terminator));
def normalize_function:
  .blocks = ((.blocks // []) | normalize_blocks)
  | del(.entry_block);
del(.cfg)
| .functions = ((.functions // []) | map(normalize_function) | sort_by(.name))
JQ

if ! jq -S -f "$CANON_FILTER" "$PHASE29Y_BINARY_ONLY_WORKDIR/stage1.mir" > "$PHASE29Y_BINARY_ONLY_WORKDIR/stage1.norm"; then
  test_fail "$SMOKE_NAME: canonicalize stage1 failed"
  exit 1
fi
if ! jq -S -f "$CANON_FILTER" "$PHASE29Y_BINARY_ONLY_WORKDIR/stage2.mir" > "$PHASE29Y_BINARY_ONLY_WORKDIR/stage2.norm"; then
  test_fail "$SMOKE_NAME: canonicalize stage2 failed"
  exit 1
fi

if ! cmp -s "$PHASE29Y_BINARY_ONLY_WORKDIR/stage1.norm" "$PHASE29Y_BINARY_ONLY_WORKDIR/stage2.norm"; then
  diff -u "$PHASE29Y_BINARY_ONLY_WORKDIR/stage1.norm" "$PHASE29Y_BINARY_ONLY_WORKDIR/stage2.norm" | tail -n 80 || true
  test_fail "$SMOKE_NAME: canonical MIR mismatch between pass1 and pass2"
  exit 1
fi

# run path sanity
phase29y_binary_only_run_in_workdir "$RUN_TIMEOUT_SECS" 1 --backend vm --hako-run ./input.hako
OUTPUT="$PHASE29Y_BINARY_ONLY_OUTPUT"
RC="$PHASE29Y_BINARY_ONLY_RC"
if [ "$RC" -eq 124 ]; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: run timeout"
  exit 1
fi
if [ "$RC" -ne 0 ]; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: run failed rc=$RC"
  exit 1
fi
check_stale_binary_only_markers "$OUTPUT" "run"

if printf '%s\n' "$OUTPUT" | rg -q '^\[vm-hako/unimplemented\]'; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: unexpected vm-hako unimplemented tag on run"
  exit 1
fi
if printf '%s\n' "$OUTPUT" | rg -q '^\[vm-hako/contract'; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: unexpected vm-hako contract tag on run"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (binary-only selfhost readiness proxy locked)"
