#!/bin/bash
# phase29y_hako_emit_mir_nested_ternary_debt_probe_vm.sh
# lane-B non-gating debt probe:
# - Rust route (`--emit-mir-json`) must stay green for nested ternary fixture.
# - .hako route is checked in selfhost-first + no-delegate + mainline-only mode.
# - default STRICT=0: known debt (Rust green / .hako NG) is reported as monitor PASS.
# - STRICT=1: any Rust/.hako divergence is treated as FAIL (blocker trigger).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29y_hako_emit_mir_nested_ternary_debt_probe_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/phase29y_hako_emit_mir_nested_ternary_probe_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-25}"
STRICT="${STRICT:-0}"
EMIT_HELPER="$NYASH_ROOT/tools/hakorune_emit_mir.sh"

if ! [[ "$RUN_TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  test_fail "$SMOKE_NAME: timeout must be integer: $RUN_TIMEOUT_SECS"
  exit 2
fi

if [ "$STRICT" != "0" ] && [ "$STRICT" != "1" ]; then
  test_fail "$SMOKE_NAME: STRICT must be 0 or 1 (actual=$STRICT)"
  exit 2
fi

if [ ! -f "$INPUT" ]; then
  test_fail "$SMOKE_NAME: fixture missing: $INPUT"
  exit 2
fi

if [ ! -f "$EMIT_HELPER" ]; then
  test_fail "$SMOKE_NAME: helper missing: $EMIT_HELPER"
  exit 2
fi

TMP_RUST_MIR="$(mktemp /tmp/phase29y_nested_ternary_rust.XXXXXX.json)"
TMP_HAKO_MIR="$(mktemp /tmp/phase29y_nested_ternary_hako.XXXXXX.json)"
TMP_RUST_SIG="$(mktemp /tmp/phase29y_nested_ternary_rust_sig.XXXXXX.json)"
TMP_HAKO_SIG="$(mktemp /tmp/phase29y_nested_ternary_hako_sig.XXXXXX.json)"

cleanup() {
  rm -f "$TMP_RUST_MIR" "$TMP_HAKO_MIR" "$TMP_RUST_SIG" "$TMP_HAKO_SIG"
}
trap cleanup EXIT

RUST_BASE_ENV=(
  NYASH_DISABLE_PLUGINS=1
  NYASH_VM_USE_FALLBACK=0
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0
  NYASH_JOINIR_DEV=0
  NYASH_JOINIR_STRICT=0
  HAKO_JOINIR_STRICT=0
  HAKO_JOINIR_PLANNER_REQUIRED=0
)

HAKO_BASE_ENV=(
  NYASH_DISABLE_PLUGINS=1
  NYASH_VM_USE_FALLBACK=0
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0
  NYASH_JOINIR_DEV=0
  NYASH_JOINIR_STRICT=0
  HAKO_JOINIR_STRICT=0
  HAKO_JOINIR_PLANNER_REQUIRED=0
  HAKO_SELFHOST_BUILDER_FIRST=1
  HAKO_SELFHOST_NO_DELEGATE=1
  HAKO_EMIT_MIR_MAINLINE_ONLY=1
)

set +e
RUST_OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  "${RUST_BASE_ENV[@]}" \
  "$NYASH_BIN" --emit-mir-json "$TMP_RUST_MIR" "$INPUT" 2>&1)
RUST_RC=$?
set -e

if [ "$RUST_RC" -eq 124 ]; then
  printf '%s\n' "$RUST_OUTPUT" | tail -n 80 || true
  test_fail "$SMOKE_NAME: rust route timeout"
  exit 1
fi

if [ "$RUST_RC" -ne 0 ]; then
  printf '%s\n' "$RUST_OUTPUT" | tail -n 80 || true
  test_fail "$SMOKE_NAME: rust route failed (rc=$RUST_RC)"
  exit 1
fi

if [ ! -s "$TMP_RUST_MIR" ]; then
  test_fail "$SMOKE_NAME: rust MIR output missing"
  exit 1
fi

set +e
HAKO_OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  "${HAKO_BASE_ENV[@]}" \
  bash "$EMIT_HELPER" "$INPUT" "$TMP_HAKO_MIR" 2>&1)
HAKO_RC=$?
set -e

if [ "$HAKO_RC" -eq 124 ]; then
  printf '%s\n' "$HAKO_OUTPUT" | tail -n 80 || true
  test_fail "$SMOKE_NAME: hako route outer timeout"
  exit 1
fi

hako_known_debt_marker() {
  local output="$1"
  if printf '%s\n' "$output" | rg -q '\[FAIL\] Stage-B failed under mainline-only mode'; then
    return 0
  fi
  if printf '%s\n' "$output" | rg -q '\[FAIL\] selfhost-first failed and delegate disabled'; then
    return 0
  fi
  if printf '%s\n' "$output" | rg -q '\[builder/selfhost-first:fail:'; then
    return 0
  fi
  return 1
}

emit_canonical_main_sig() {
  local input_json="$1"
  local output_json="$2"
  jq -e -S '
    (.functions | map(select(.name=="main")) | if length == 1 then .[0] else error("main missing or duplicated") end) as $m
    | {
        name: $m.name,
        blocks: [
          $m.blocks[] | {
            id,
            instructions: [
              .instructions[] | {
                op,
                operation: (.operation // null),
                target: (.target // null),
                then: (.then // null),
                else: (.else // null),
                callee: (.callee // null),
                method: (.method // null),
                argc: ((.args // []) | length),
                incoming_count: ((.incoming // []) | length),
                value_const: (if .op == "const" then (.value // null) else null end)
              }
            ]
          }
        ]
      }
  ' "$input_json" >"$output_json"
}

report_debt_or_fail() {
  local reason="$1"
  local detail="$2"
  if [ "$STRICT" = "1" ]; then
    test_fail "$SMOKE_NAME: strict parity lock failed ($reason)"
    printf '%s\n' "$detail" | tail -n 80 || true
    exit 1
  fi
  echo "[INFO] lane-b parity debt observed: $reason"
  if [ -n "$detail" ]; then
    printf '%s\n' "$detail" | tail -n 40 || true
  fi
  test_pass "$SMOKE_NAME: PASS (monitor-only debt observed: $reason)"
  exit 0
}

if [ "$HAKO_RC" -ne 0 ]; then
  if hako_known_debt_marker "$HAKO_OUTPUT"; then
    report_debt_or_fail "rust_green_hako_ng" "$HAKO_OUTPUT"
  fi
  printf '%s\n' "$HAKO_OUTPUT" | tail -n 80 || true
  test_fail "$SMOKE_NAME: hako route failed without known debt marker (rc=$HAKO_RC)"
  exit 1
fi

if [ ! -s "$TMP_HAKO_MIR" ]; then
  report_debt_or_fail "hako_mir_missing" "$HAKO_OUTPUT"
fi

if ! printf '%s\n' "$HAKO_OUTPUT" | rg -q '\[OK\] MIR JSON written \(selfhost-first\):'; then
  report_debt_or_fail "hako_not_selfhost_first" "$HAKO_OUTPUT"
fi

if ! emit_canonical_main_sig "$TMP_RUST_MIR" "$TMP_RUST_SIG"; then
  test_fail "$SMOKE_NAME: rust canonical main signature generation failed"
  exit 1
fi

if ! emit_canonical_main_sig "$TMP_HAKO_MIR" "$TMP_HAKO_SIG"; then
  report_debt_or_fail "hako_canonicalize_failed" "$HAKO_OUTPUT"
fi

if ! jq -e '.blocks[].instructions[] | select(.op=="branch")' "$TMP_RUST_SIG" >/dev/null; then
  test_fail "$SMOKE_NAME: rust main signature missing branch op"
  exit 1
fi

HASH_RUST="$(sha256sum "$TMP_RUST_SIG" | awk '{print $1}')"
HASH_HAKO="$(sha256sum "$TMP_HAKO_SIG" | awk '{print $1}')"

if [ "$HASH_RUST" != "$HASH_HAKO" ]; then
  DIFF_OUTPUT="$(diff -u "$TMP_RUST_SIG" "$TMP_HAKO_SIG" || true)"
  report_debt_or_fail "canonical_signature_mismatch" "$DIFF_OUTPUT"
fi

test_pass "$SMOKE_NAME: PASS (nested ternary parity locked)"
