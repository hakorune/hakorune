#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC="docs/development/current/main/phases/phase-29cc/29cc-195-wsm-p10-min2-loop-extern-matcher-inventory-lock-ssot.md"
SMOKE="tools/smokes/v2/profiles/integration/phase29cc_wsm/p10/phase29cc_wsm_p10_min2_loop_extern_matcher_inventory_lock_vm.sh"
DEV_GATE="tools/checks/dev_gate.sh"
SHAPE_TABLE="src/backend/wasm/shape_table.rs"

if [ ! -f "$DOC" ]; then
  echo "[wsm-p10-min2-guard] missing lock doc: $DOC" >&2
  exit 1
fi

for needle in "WSM-P10-min2" "analysis-only" "wsm.p10.main_loop_extern_call.v0" "WSM-P10-min3"; do
  if ! rg -q "$needle" "$DOC"; then
    echo "[wsm-p10-min2-guard] missing keyword in lock doc: $needle" >&2
    exit 1
  fi
done

for needle in "detect_p10_loop_extern_call_candidate" "P10_LOOP_EXTERN_CANDIDATE_ID"; do
  if ! rg -q "$needle" "$SHAPE_TABLE"; then
    echo "[wsm-p10-min2-guard] shape table contract missing: $needle" >&2
    exit 1
  fi
done

if ! rg -q "phase29cc_wsm_p10_loop_extern_matcher_inventory_guard.sh" "$DEV_GATE"; then
  echo "[wsm-p10-min2-guard] dev_gate missing p10 min2 guard step" >&2
  exit 1
fi

if [ ! -x "$SMOKE" ]; then
  echo "[wsm-p10-min2-guard] missing or not executable: $SMOKE" >&2
  exit 1
fi

bash "$SMOKE"
echo "[wsm-p10-min2-guard] ok"
