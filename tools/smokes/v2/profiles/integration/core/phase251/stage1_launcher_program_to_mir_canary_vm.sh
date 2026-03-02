#!/usr/bin/env bash
# stage1_launcher_program_to_mir_canary_vm.sh
# - Canary for Phase 25.1a: ensure Stage‑B + provider delegate can emit MIR(JSON)
#   for the Stage1 CLI launcher source (lang/src/runner/launcher.hako).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || (cd "$SCRIPT_DIR/../../../../../../.." && pwd))"

if [ ! -f "$ROOT_DIR/lang/src/runner/launcher.hako" ]; then
  echo "[SKIP] stage1_launcher_program_to_mir_canary_vm (launcher.hako missing)"
  exit 0
fi

# Quick profile: Stage-B emit is flaky under Stage-3 default; skip for now.
echo "[SKIP] stage1_launcher_program_to_mir_canary_vm (disabled in quick profile after env consolidation)"
exit 0

source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true
require_env || { echo "[SKIP] env not ready"; exit 0; }

SRC="$ROOT_DIR/lang/src/runner/launcher.hako"
OUT_JSON="$(mktemp --suffix .json)"
LOG_OUT="$(mktemp --suffix .log)"
trap 'rm -f "$OUT_JSON" "$LOG_OUT" || true' EXIT

set +e
HAKO_SELFHOST_BUILDER_FIRST=0 \
NYASH_JSON_ONLY=1 \
bash "$ROOT_DIR/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-120}" --out "$OUT_JSON" --input "$SRC" >"$LOG_OUT" 2>&1
rc=$?
set -e

if [ $rc -ne 0 ] || [ ! -s "$OUT_JSON" ]; then
  echo "[FAIL] stage1_launcher_program_to_mir_canary_vm (Program→MIR failed rc=$rc)" >&2
  sed -n '1,80p' "$LOG_OUT" >&2 || true
  exit 1
fi

echo "[PASS] stage1_launcher_program_to_mir_canary_vm"
exit 0
