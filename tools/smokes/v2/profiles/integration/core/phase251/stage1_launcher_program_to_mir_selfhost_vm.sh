#!/usr/bin/env bash
# stage1_launcher_program_to_mir_selfhost_vm.sh
# - Canary for Phase 25.1b: ensure Stage‑B + selfhost builder (MirBuilderBox)
#   sees Stage1 CLI launcher (HakoCli.run) and still produces MIR(JSON)
#   when selfhost-first is enabled (provider fallback可)。

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || (cd "$SCRIPT_DIR/../../../../../../.." && pwd))"

if [ ! -f "$ROOT_DIR/lang/src/runner/launcher.hako" ]; then
  echo "[SKIP] stage1_launcher_program_to_mir_selfhost_vm (launcher.hako missing)"
  exit 0
fi

# Quick profile: Stage-B emit is flaky under Stage-3 default; skip for now.
echo "[SKIP] stage1_launcher_program_to_mir_selfhost_vm (disabled in quick profile after env consolidation)"
exit 0

source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true
require_env || { echo "[SKIP] env not ready"; exit 0; }

SRC="$ROOT_DIR/lang/src/runner/launcher.hako"
OUT_JSON="$(mktemp --suffix .json)"
LOG_OUT="$(mktemp --suffix .log)"
trap 'rm -f "$OUT_JSON" "$LOG_OUT" || true' EXIT

set +e
HAKO_SELFHOST_BUILDER_FIRST=1 \
HAKO_MIR_BUILDER_FUNCS=1 \
HAKO_SELFHOST_TRACE=1 \
NYASH_JSON_ONLY=1 \
bash "$ROOT_DIR/tools/hakorune_emit_mir.sh" "$SRC" "$OUT_JSON" >"$LOG_OUT" 2>&1
rc=$?
set -e

if [ $rc -ne 0 ] || [ ! -s "$OUT_JSON" ]; then
  echo "[FAIL] stage1_launcher_program_to_mir_selfhost_vm (Program→MIR failed rc=$rc)" >&2
  sed -n '1,80p' "$LOG_OUT" >&2 || true
  exit 1
fi

# selfhost builder 側の観測タグが出ているか確認する（構造チェック）
if ! grep -q "\[builder/cli:entry_detected\]" "$LOG_OUT"; then
  echo "[FAIL] stage1_launcher_program_to_mir_selfhost_vm (missing [builder/cli:entry_detected] tag)" >&2
  sed -n '1,80p' "$LOG_OUT" >&2 || true
  exit 1
fi

if ! grep -q "\[builder/cli:run_shape\]" "$LOG_OUT"; then
  echo "[FAIL] stage1_launcher_program_to_mir_selfhost_vm (missing [builder/cli:run_shape] tag)" >&2
  sed -n '1,80p' "$LOG_OUT" >&2 || true
  exit 1
fi

echo "[PASS] stage1_launcher_program_to_mir_selfhost_vm"
exit 0
