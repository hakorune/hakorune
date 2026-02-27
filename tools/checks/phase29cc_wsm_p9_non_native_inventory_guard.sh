#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC_MIN0="docs/development/current/main/phases/phase-29cc/29cc-189-wsm-p9-min0-non-native-inventory-lock-ssot.md"
DOC_MIN1="docs/development/current/main/phases/phase-29cc/29cc-190-wsm-p9-min1-const-binop-native-shape-lock-ssot.md"
SMOKE_MIN0="tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p9_min0_non_native_inventory_lock_vm.sh"
SMOKE_MIN1="tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p9_min1_const_binop_native_lock_vm.sh"
DEV_GATE="tools/checks/dev_gate.sh"

for file in "$DOC_MIN0" "$DOC_MIN1"; do
  if [ ! -f "$file" ]; then
    echo "[wsm-p9-inventory-guard] missing lock doc: $file" >&2
    exit 1
  fi
done

for needle in \
  "WSM-P9-min0" \
  "BridgeRustBackend" \
  "WSM-P9-min1" \
  "wsm.p9.main_return_i32_const_binop.v0"; do
  if ! rg -q "$needle" "$DOC_MIN0" "$DOC_MIN1"; then
    echo "[wsm-p9-inventory-guard] missing keyword in lock docs: $needle" >&2
    exit 1
  fi
done

if ! rg -q "phase29cc_wsm_p9_non_native_inventory_guard.sh" "$DEV_GATE"; then
  echo "[wsm-p9-inventory-guard] dev_gate missing p9 guard step" >&2
  exit 1
fi

for smoke in "$SMOKE_MIN0" "$SMOKE_MIN1"; do
  if [ ! -x "$smoke" ]; then
    echo "[wsm-p9-inventory-guard] missing or not executable: $smoke" >&2
    exit 1
  fi
done

bash "$SMOKE_MIN0"
bash "$SMOKE_MIN1"
echo "[wsm-p9-inventory-guard] ok"
