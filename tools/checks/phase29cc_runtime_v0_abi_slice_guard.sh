#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

LOCK_DOC="docs/development/current/main/phases/phase-29cc/29cc-216-runtime-v0-abi-slice-lock-ssot.md"
CUTOVER_SSOT="docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md"
ABI_MATRIX="docs/reference/abi/ABI_BOUNDARY_MATRIX.md"
DEV_GATE="tools/checks/dev_gate.sh"

for file in "$LOCK_DOC" "$CUTOVER_SSOT" "$ABI_MATRIX" "$DEV_GATE"; do
  if [ ! -f "$file" ]; then
    echo "[runtime-v0-abi-slice-guard] missing file: $file" >&2
    exit 1
  fi
done

for keyword in "string_len" "array_get_i64" "array_set_i64" "args borrowed / return owned"; do
  if ! rg -F -q "$keyword" "$LOCK_DOC" "$CUTOVER_SSOT"; then
    echo "[runtime-v0-abi-slice-guard] missing keyword: $keyword" >&2
    exit 1
  fi
done

if ! rg -q "Runtime V0 helper slice" "$ABI_MATRIX"; then
  echo "[runtime-v0-abi-slice-guard] ABI matrix missing V0 helper slice row" >&2
  exit 1
fi

if ! rg -q "phase29cc_runtime_v0_abi_slice_guard.sh" "$DEV_GATE"; then
  echo "[runtime-v0-abi-slice-guard] dev_gate missing runtime-v0-abi-slice guard wiring" >&2
  exit 1
fi

echo "[runtime-v0-abi-slice-guard] ok"
