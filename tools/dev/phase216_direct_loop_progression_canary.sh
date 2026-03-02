#!/usr/bin/env bash
# Phase 21.6 direct-route canary:
# Guard against loop-step stalling in direct --emit-mir-json payloads.
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$ROOT"

FIXTURE="$ROOT/apps/tests/phase216_mainline_loop_count_param_nonsym_min.hako"
if [[ ! -f "$FIXTURE" ]]; then
  echo "[FAIL] missing fixture: $FIXTURE" >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "[FAIL] jq is required: phase216_direct_loop_progression_canary" >&2
  exit 1
fi

BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"
if [[ ! -x "$BIN" ]]; then
  echo "[FAIL] binary not found/executable: $BIN" >&2
  exit 1
fi

TMP_JSON=$(mktemp --suffix .json)
TMP_ERR=$(mktemp --suffix .err)
cleanup() {
  rm -f "$TMP_JSON" "$TMP_ERR" 2>/dev/null || true
}
trap cleanup EXIT

"$BIN" --emit-mir-json "$TMP_JSON" "$FIXTURE" >/dev/null

if [[ ! -s "$TMP_JSON" ]]; then
  echo "[FAIL] phase216_direct_loop_progression: emitted MIR missing" >&2
  exit 1
fi

set +e
"$BIN" --mir-json-file "$TMP_JSON" >/dev/null 2>"$TMP_ERR"
rc=$?
set -e

if [[ "$rc" -ne 14 ]]; then
  echo "[FAIL] phase216_direct_loop_progression: rc=$rc (expect 14)" >&2
  head -n 40 "$TMP_ERR" >&2 || true
  exit 1
fi

if rg -q "vm step budget exceeded" "$TMP_ERR"; then
  echo "[FAIL] phase216_direct_loop_progression: step budget exceeded regression" >&2
  head -n 40 "$TMP_ERR" >&2 || true
  exit 1
fi

if rg -q "Invalid value: \\[rust-vm\\] use of undefined value ValueId\\(0\\)" "$TMP_ERR"; then
  echo "[FAIL] phase216_direct_loop_progression: ValueId(0) regression" >&2
  head -n 40 "$TMP_ERR" >&2 || true
  exit 1
fi

phi_dst=$(
  jq -r '
    .functions[]?
    | select(.name=="main")
    | .blocks[]?
    | .instructions[]?
    | select(.op=="phi")
    | .dst
  ' "$TMP_JSON" | head -n 1
)
bin_lhs=$(
  jq -r '
    .functions[]?
    | select(.name=="main")
    | .blocks[]?
    | .instructions[]?
    | select(.op=="binop")
    | .lhs
  ' "$TMP_JSON" | head -n 1
)
step_src=$(
  jq -r --argjson lhs "$bin_lhs" '
    .functions[]?
    | select(.name=="main")
    | .blocks[]?
    | .instructions[]?
    | select(.op=="copy" and .dst==$lhs)
    | .src
  ' "$TMP_JSON" | head -n 1
)

if [[ -z "$phi_dst" || -z "$bin_lhs" || -z "$step_src" ]]; then
  echo "[FAIL] phase216_direct_loop_progression: unable to inspect loop-step MIR shape" >&2
  exit 1
fi

if [[ "$step_src" != "$phi_dst" ]]; then
  echo "[FAIL] phase216_direct_loop_progression: step uses stale source (copy src=$step_src, phi dst=$phi_dst)" >&2
  exit 1
fi

echo "[PASS] phase216_direct_loop_progression rc=14"
exit 0
