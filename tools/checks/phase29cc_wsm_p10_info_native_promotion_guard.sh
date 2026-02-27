#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC="docs/development/current/main/phases/phase-29cc/29cc-200-wsm-p10-min7-info-native-promotion-lock-ssot.md"
SMOKE="tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min7_info_native_promotion_lock_vm.sh"
DEV_GATE="tools/checks/dev_gate.sh"
SHAPE_TABLE="src/backend/wasm/shape_table.rs"
WASM_MOD="src/backend/wasm/mod.rs"
WRITER="src/backend/wasm/binary_writer.rs"

if [ ! -f "$DOC" ]; then
  echo "[wsm-p10-min7-guard] missing lock doc: $DOC" >&2
  exit 1
fi

for needle in \
  "WSM-P10-min7" \
  "wsm.p10.main_loop_extern_call.info.fixed4.v0" \
  "LoopExternImport::ConsoleInfo" \
  "info.fixed3.inventory.v0" \
  "WSM-P10-min8"; do
  if ! rg -Fq "$needle" "$DOC"; then
    echo "[wsm-p10-min7-guard] missing keyword in lock doc: $needle" >&2
    exit 1
  fi
done

for needle in \
  "detect_p10_min7_info_native_promotable_shape" \
  "P10_MIN7_INFO_NATIVE_SHAPE_ID"; do
  if ! rg -Fq "$needle" "$SHAPE_TABLE"; then
    echo "[wsm-p10-min7-guard] shape table contract missing: $needle" >&2
    exit 1
  fi
done

for needle in \
  "detect_p10_min7_info_native_promotable_shape" \
  "build_loop_extern_call_skeleton_module_with_import" \
  "LoopExternImport::ConsoleInfo"; do
  if ! rg -Fq "$needle" "$WASM_MOD"; then
    echo "[wsm-p10-min7-guard] wasm mod contract missing: $needle" >&2
    exit 1
  fi
done

for needle in \
  "enum LoopExternImport" \
  "ConsoleInfo" \
  "build_loop_extern_call_skeleton_module_with_import"; do
  if ! rg -Fq "$needle" "$WRITER"; then
    echo "[wsm-p10-min7-guard] writer contract missing: $needle" >&2
    exit 1
  fi
done

if ! rg -Fq "phase29cc_wsm_p10_info_native_promotion_guard.sh" "$DEV_GATE"; then
  echo "[wsm-p10-min7-guard] dev_gate missing p10 min7 guard step" >&2
  exit 1
fi

if [ ! -x "$SMOKE" ]; then
  echo "[wsm-p10-min7-guard] missing or not executable: $SMOKE" >&2
  exit 1
fi

bash "$SMOKE"
echo "[wsm-p10-min7-guard] ok"
