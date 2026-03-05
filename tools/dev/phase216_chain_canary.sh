#!/usr/bin/env bash
# Phase 21.6 chain canary — Stage‑B → MirBuilder → ny‑llvmc(crate) → EXE
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$ROOT"

run_case() {
  local fixture="$1"
  local expected_rc="$2"
  local label="$3"
  if [[ ! -f "$fixture" ]]; then
    echo "[FAIL] missing fixture ($label): $fixture" >&2
    exit 1
  fi

  local tmp_json
  local out_exe
  tmp_json=$(mktemp --suffix .json)
  out_exe=$(mktemp --suffix .exe)

  # Emit MIR(JSON)
  HAKO_SELFHOST_BUILDER_FIRST=1 \
  NYASH_USE_NY_COMPILER=0 HAKO_DISABLE_NY_COMPILER=1 \
  NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
  NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
    bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-mainline --timeout-secs "${HAKO_BUILD_TIMEOUT:-60}" --out "$tmp_json" --input "$fixture" >/dev/null

  # Build EXE (crate)
  NYASH_LLVM_BACKEND=crate NYASH_LLVM_SKIP_BUILD=1 \
  NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
  NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
    bash "$ROOT/tools/ny_mir_builder.sh" --in "$tmp_json" --emit exe -o "$out_exe" --quiet >/dev/null

  set +e
  "$out_exe"; local rc=$?
  set -e

  rm -f "$tmp_json" "$out_exe" 2>/dev/null || true

  if [[ "$rc" != "$expected_rc" ]]; then
    echo "[FAIL] $label rc=$rc (expect $expected_rc)" >&2
    exit 1
  fi
  echo "[PASS] $label rc=$expected_rc"
}

FIXTURE_BASE="$ROOT/apps/tests/phase216_mainline_loop_undefined_value_blocker_min.hako"
FIXTURE_NONSYM="$ROOT/apps/tests/phase216_mainline_loop_count_param_nonsym_min.hako"

run_case "$FIXTURE_BASE" "10" "phase216_chain_canary/base"
run_case "$FIXTURE_NONSYM" "14" "phase216_chain_canary/nonsym"
echo "[PASS] phase216_chain_canary all cases"
exit 0
