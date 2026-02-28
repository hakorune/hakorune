#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

LOCK_DOC="docs/development/current/main/phases/phase-29cc/29cc-216-runtime-v0-abi-slice-lock-ssot.md"
CUTOVER_SSOT="docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md"
ABI_MATRIX="docs/reference/abi/ABI_BOUNDARY_MATRIX.md"
DEV_GATE="tools/checks/dev_gate.sh"
REGISTRY_FILE="lang/src/vm/boxes/abi_adapter_registry.hako"
HANDLER_FILE="lang/src/vm/boxes/mir_call_v1_handler.hako"
ARRAY_CORE_FILE="lang/src/runtime/collections/array_core_box.hako"
STRING_CORE_FILE="lang/src/runtime/collections/string_core_box.hako"

for file in \
  "$LOCK_DOC" \
  "$CUTOVER_SSOT" \
  "$ABI_MATRIX" \
  "$DEV_GATE" \
  "$REGISTRY_FILE" \
  "$HANDLER_FILE" \
  "$ARRAY_CORE_FILE" \
  "$STRING_CORE_FILE"; do
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

# Code-side adapter route contract (Step-3 anchor)
if ! rg -F -q 'me._put("StringBox", "length", "nyash.string.len_h"' "$REGISTRY_FILE"; then
  echo "[runtime-v0-abi-slice-guard] registry missing StringBox.length -> nyash.string.len_h" >&2
  exit 1
fi
if ! rg -F -q 'me._put("ArrayBox", "get",    "nyash.array.get_h"' "$REGISTRY_FILE"; then
  echo "[runtime-v0-abi-slice-guard] registry missing ArrayBox.get adapter mapping" >&2
  exit 1
fi
if ! rg -F -q 'me._put("ArrayBox", "set",    "nyash.array.set_h"' "$REGISTRY_FILE"; then
  echo "[runtime-v0-abi-slice-guard] registry missing ArrayBox.set adapter mapping" >&2
  exit 1
fi
if ! rg -F -q 'StringCoreBox.len_i64(' "$HANDLER_FILE"; then
  echo "[runtime-v0-abi-slice-guard] handler missing StringCoreBox len route" >&2
  exit 1
fi
if ! rg -F -q 'ArrayCoreBox.get_i64(' "$HANDLER_FILE"; then
  echo "[runtime-v0-abi-slice-guard] handler missing ArrayCoreBox get route" >&2
  exit 1
fi
if ! rg -F -q 'ArrayCoreBox.set_i64(' "$HANDLER_FILE"; then
  echo "[runtime-v0-abi-slice-guard] handler missing ArrayCoreBox set route" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.string.len_h"' "$STRING_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] string core missing nyash.string.len_h extern route" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.array.get_hi"' "$ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] array core missing nyash.array.get_hi extern route" >&2
  exit 1
fi
if ! rg -F -q 'externcall "nyash.array.set_hii"' "$ARRAY_CORE_FILE"; then
  echo "[runtime-v0-abi-slice-guard] array core missing nyash.array.set_hii extern route" >&2
  exit 1
fi

echo "[runtime-v0-abi-slice-guard] ok"
