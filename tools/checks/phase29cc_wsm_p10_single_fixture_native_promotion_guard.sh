#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC="docs/development/current/main/phases/phase-29cc/29cc-197-wsm-p10-min4-single-fixture-native-promotion-lock-ssot.md"
SMOKE="tools/smokes/v2/profiles/integration/phase29cc_wsm/p10/phase29cc_wsm_p10_min4_single_fixture_native_promotion_lock_vm.sh"
DEV_GATE="tools/checks/dev_gate.sh"
SHAPE_TABLE="src/backend/wasm/shape_table.rs"
WASM_MOD="src/backend/wasm/mod.rs"

if [ ! -f "$DOC" ]; then
  echo "[wsm-p10-min4-guard] missing lock doc: $DOC" >&2
  exit 1
fi

for needle in \
  "WSM-P10-min4" \
  "phase29cc_wsm_p10_min4_loop_extern_native.hako" \
  "wsm.p10.main_loop_extern_call.fixed3.v0" \
  "build_loop_extern_call_skeleton_module(3)" \
  "WSM-P10-min5"; do
  if ! rg -Fq "$needle" "$DOC"; then
    echo "[wsm-p10-min4-guard] missing keyword in lock doc: $needle" >&2
    exit 1
  fi
done

for needle in "detect_p10_min4_native_promotable_shape" "wsm.p10.main_loop_extern_call.fixed3.v0"; do
  if ! rg -Fq "$needle" "$SHAPE_TABLE"; then
    echo "[wsm-p10-min4-guard] shape table contract missing: $needle" >&2
    exit 1
  fi
done

for needle in "detect_p10_min4_native_promotable_shape" "build_loop_extern_call_skeleton_module(3)"; do
  if ! rg -Fq "$needle" "$WASM_MOD"; then
    echo "[wsm-p10-min4-guard] wasm mod contract missing: $needle" >&2
    exit 1
  fi
done

if ! rg -Fq "phase29cc_wsm_p10_single_fixture_native_promotion_guard.sh" "$DEV_GATE"; then
  echo "[wsm-p10-min4-guard] dev_gate missing p10 min4 guard step" >&2
  exit 1
fi

if [ ! -x "$SMOKE" ]; then
  echo "[wsm-p10-min4-guard] missing or not executable: $SMOKE" >&2
  exit 1
fi

bash "$SMOKE"
echo "[wsm-p10-min4-guard] ok"
