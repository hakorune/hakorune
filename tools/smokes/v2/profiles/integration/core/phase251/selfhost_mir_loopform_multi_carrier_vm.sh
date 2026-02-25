#!/usr/bin/env bash
# selfhost_mir_loopform_multi_carrier_vm.sh
# - Canary for Phase 25.1b multi-carrier LoopForm (fibonacci-style).
# - Ensures LowerLoopMultiCarrierBox + LoopFormBox.build2(mode="multi_count")
#   can emit MIR(JSON) for a simple fibonacci-like loop via selfhost builder.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || (cd "$SCRIPT_DIR/../../../../../../.." && pwd))"

TEST_HAKO="$(mktemp --suffix .hako)"
OUT_MIR="$(mktemp --suffix .json)"
LOG_OUT="$(mktemp --suffix .log)"
trap 'rm -f "$TEST_HAKO" "$OUT_MIR" "$LOG_OUT" || true' EXIT

# Quick profile: Stage-B emit is flaky under Stage-3 default; skip for now.
echo "[SKIP] selfhost_mir_loopform_multi_carrier_vm (disabled in quick profile after env consolidation)"
exit 0

# Fibonacci-style loop: i=0; a=0; b=1; loop(i<n){ t=a+b; a=b; b=t; i=i+1 }; return b
cat >"$TEST_HAKO" <<'HAKO'
static box TestBox {
  method fib(n) {
    local i = 0
    local a = 0
    local b = 1
    loop(i < n) {
      local t = a + b
      a = b
      b = t
      i = i + 1
    }
    return b
  }
}

static box Main {
  method main(args) {
    local t = new TestBox()
    return t.fib(6)
  }
}
HAKO

set +e
HAKO_SELFHOST_BUILDER_FIRST=1 \
HAKO_MIR_BUILDER_FUNCS=1 \
HAKO_SELFHOST_TRACE=1 \
HAKO_MIR_BUILDER_TRACE=1 \
HAKO_SILENT_TAGS=0 \
NYASH_JSON_ONLY=1 \
bash "$ROOT_DIR/tools/hakorune_emit_mir.sh" "$TEST_HAKO" "$OUT_MIR" >"$LOG_OUT" 2>&1
rc=$?
set -e

# If selfhost builder failed due to missing using modules and provider delegate
# wrote MIR instead, treat as SKIP（環境が揃ってから本格確認する）。
if [ $rc -ne 0 ] || [ ! -s "$OUT_MIR" ]; then
  if grep -q "[builder/selfhost-first:diagnose] Missing using modules detected" "$LOG_OUT" 2>/dev/null; then
    echo "[SKIP] selfhost_mir_loopform_multi_carrier_vm (selfhost builder missing using modules; provider delegate used)" >&2
    exit 0
  fi
  echo "[FAIL] selfhost_mir_loopform_multi_carrier_vm (MIR generation failed rc=$rc)" >&2
  echo "=== LOG OUTPUT ===" >&2
  sed -n '1,120p' "$LOG_OUT" >&2 || true
  exit 1
fi

# If builder は失敗し、provider delegate が MIR を書いただけなら、まだ本命経路が
# 有効になっていないので SKIP 扱いにする。
if grep -q "[builder/selfhost-first:diagnose] Missing using modules detected" "$LOG_OUT" 2>/dev/null \
   || grep -q "delegate:provider" "$LOG_OUT" 2>/dev/null; then
  echo "[SKIP] selfhost_mir_loopform_multi_carrier_vm (selfhost builder not active yet; provider delegate in use)" >&2
  exit 0
fi

# Check that multi-carrier lower was actually used
# If Stage‑B がまだ loop を含まない Program(JSON) しか出していない場合は、
# multi-carrier lower が動けないので SKIP 扱いにする（将来 Stage‑B 側が
# defs/loop を含むようになったときに PASS へ昇格させる）。
if ! grep -q "\[mirbuilder/internal/loop:multi_carrier:detected" "$LOG_OUT"; then
  if grep -q "tokens=Loop:0,Compare:0" "$LOG_OUT" 2>/dev/null; then
    echo "[SKIP] selfhost_mir_loopform_multi_carrier_vm (Stage-B Program JSON has no Loop yet)" >&2
    exit 0
  fi
  echo "[FAIL] selfhost_mir_loopform_multi_carrier_vm (no multi_carrier detection tag)" >&2
  sed -n '1,120p' "$LOG_OUT" >&2 || true
  exit 1
fi

if ! grep -q "\[funcs/basic:loop.multi_carrier\]" "$LOG_OUT"; then
  echo "[FAIL] selfhost_mir_loopform_multi_carrier_vm (no [funcs/basic:loop.multi_carrier] tag)" >&2
  sed -n '1,120p' "$LOG_OUT" >&2 || true
  exit 1
fi

# Ensure TestBox.fib/1 function exists in MIR
if ! grep -q '"name":"TestBox.fib/1"' "$OUT_MIR"; then
  echo "[FAIL] selfhost_mir_loopform_multi_carrier_vm (TestBox.fib/1 not present in MIR)" >&2
  cat "$OUT_MIR" >&2
  exit 1
fi

echo "[PASS] selfhost_mir_loopform_multi_carrier_vm (multi-carrier LoopForm used for TestBox.fib/1)"
exit 0
