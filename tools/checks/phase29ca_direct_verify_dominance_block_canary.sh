#!/usr/bin/env bash
# phase29ca_direct_verify_dominance_block_canary.sh
# Legacy name retained.
# Guard that the former direct-verify dominance/Phi blocker stays resolved.
# This requires a built release binary and is not wired into quick gate.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"
FIXTURE="$ROOT/apps/tests/phase29ca_generic_loop_continue_min.hako"

if [[ ! -x "$BIN" ]]; then
  echo "[FAIL] phase29ca_direct_verify_dominance_block: binary missing/executable: $BIN" >&2
  exit 1
fi

if [[ ! -f "$FIXTURE" ]]; then
  echo "[FAIL] phase29ca_direct_verify_dominance_block: missing fixture: $FIXTURE" >&2
  exit 1
fi

tmp_json="$(mktemp --suffix .json)"
tmp_emit_err="$(mktemp --suffix .emit.err)"
tmp_run_err="$(mktemp --suffix .run.err)"
cleanup() {
  rm -f "$tmp_json" "$tmp_emit_err" "$tmp_run_err" 2>/dev/null || true
}
trap cleanup EXIT

set +e
"$BIN" --emit-mir-json "$tmp_json" "$FIXTURE" >/dev/null 2>"$tmp_emit_err"
emit_rc=$?
set -e

if [[ "$emit_rc" -ne 0 ]]; then
  echo "[FAIL] phase29ca_direct_verify_dominance_block: emit rc=$emit_rc (expect 0)" >&2
  sed -n '1,80p' "$tmp_emit_err" >&2 || true
  exit 1
fi

if ! [[ -s "$tmp_json" ]]; then
  echo "[FAIL] phase29ca_direct_verify_dominance_block: emitted MIR JSON is empty" >&2
  exit 1
fi

set +e
"$BIN" --mir-json-file "$tmp_json" >/dev/null 2>"$tmp_run_err"
run_rc=$?
set -e

if [[ "$run_rc" -ne 4 ]]; then
  echo "[FAIL] phase29ca_direct_verify_dominance_block: run rc=$run_rc (expect 4)" >&2
  sed -n '1,80p' "$tmp_run_err" >&2 || true
  exit 1
fi

if rg -q '\[freeze:contract\]\[emit-mir/direct-verify\]|defined in non-dominating block|without Phi' "$tmp_emit_err"; then
  echo "[FAIL] phase29ca_direct_verify_dominance_block: direct-verify dominance/Phi blocker regressed" >&2
  sed -n '1,80p' "$tmp_emit_err" >&2 || true
  exit 1
fi

if rg -q 'vm step budget exceeded|Invalid value: \[vm\] use of undefined value|undefined value ValueId' "$tmp_run_err"; then
  echo "[FAIL] phase29ca_direct_verify_dominance_block: loop progression regression detected" >&2
  sed -n '1,80p' "$tmp_run_err" >&2 || true
  exit 1
fi

echo "[PASS] phase29ca_direct_verify_dominance_block emit_rc=0 run_rc=4 (dominance/Phi resolved)"
exit 0
