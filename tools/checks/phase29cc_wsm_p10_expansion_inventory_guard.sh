#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC="docs/development/current/main/phases/phase-29cc/29cc-198-wsm-p10-min5-expansion-inventory-lock-ssot.md"
SMOKE="tools/smokes/v2/profiles/integration/phase29cc_wsm/p10/phase29cc_wsm_p10_min5_expansion_inventory_lock_vm.sh"
DEV_GATE="tools/checks/dev_gate.sh"
SHAPE_TABLE="src/backend/wasm/shape_table/p10.rs"
WASM_MOD="src/backend/wasm/mod.rs"

if [ ! -f "$DOC" ]; then
  echo "[wsm-p10-min5-guard] missing lock doc: $DOC" >&2
  exit 1
fi

for needle in \
  "WSM-P10-min5" \
  "wsm.p10.main_loop_extern_call.warn.fixed3.inventory.v0" \
  "wsm.p10.main_loop_extern_call.info.fixed3.inventory.v0" \
  "wsm.p10.main_loop_extern_call.error.fixed3.inventory.v0" \
  "wsm.p10.main_loop_extern_call.debug.fixed3.inventory.v0" \
  "analysis-only" \
  "WSM-P10-min6"; do
  if ! rg -Fq "$needle" "$DOC"; then
    echo "[wsm-p10-min5-guard] missing keyword in lock doc: $needle" >&2
    exit 1
  fi
done

for needle in \
  "detect_p10_min5_expansion_inventory_shape" \
  "P10_MIN5_WARN_INVENTORY_ID" \
  "P10_MIN5_INFO_INVENTORY_ID" \
  "P10_MIN5_ERROR_INVENTORY_ID" \
  "P10_MIN5_DEBUG_INVENTORY_ID"; do
  if ! rg -Fq "$needle" "$SHAPE_TABLE"; then
    echo "[wsm-p10-min5-guard] shape table contract missing: $needle" >&2
    exit 1
  fi
done

if ! rg -Fq "detect_p10_min5_expansion_inventory_shape" "$WASM_MOD"; then
  echo "[wsm-p10-min5-guard] wasm mod analysis hook missing" >&2
  exit 1
fi

if ! rg -Fq "phase29cc_wsm_p10_expansion_inventory_guard.sh" "$DEV_GATE"; then
  echo "[wsm-p10-min5-guard] dev_gate missing p10 min5 guard step" >&2
  exit 1
fi

if [ ! -x "$SMOKE" ]; then
  echo "[wsm-p10-min5-guard] missing or not executable: $SMOKE" >&2
  exit 1
fi

bash "$SMOKE"
echo "[wsm-p10-min5-guard] ok"
