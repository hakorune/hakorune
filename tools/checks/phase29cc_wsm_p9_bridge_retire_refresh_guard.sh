#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC="docs/development/current/main/phases/phase-29cc/29cc-193-wsm-p9-min4-bridge-retire-refresh-lock-ssot.md"
SMOKE2="tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p9_min2_loop_canvas_primer_bridge_lock_vm.sh"
SMOKE3="tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p9_min3_canvas_advanced_bridge_lock_vm.sh"
DEV_GATE="tools/checks/dev_gate.sh"

if [ ! -f "$DOC" ]; then
  echo "[wsm-p9-bridge-retire-refresh-guard] missing lock doc: $DOC" >&2
  exit 1
fi

for needle in "WSM-P9-min4" "accepted-but-blocked" "loop/canvas" "WSM-P10-min1"; do
  if ! rg -q "$needle" "$DOC"; then
    echo "[wsm-p9-bridge-retire-refresh-guard] missing keyword in lock doc: $needle" >&2
    exit 1
  fi
done

if ! rg -q "phase29cc_wsm_p9_bridge_retire_refresh_guard.sh" "$DEV_GATE"; then
  echo "[wsm-p9-bridge-retire-refresh-guard] dev_gate missing p9-min4 guard step" >&2
  exit 1
fi

for smoke in "$SMOKE2" "$SMOKE3"; do
  if [ ! -x "$smoke" ]; then
    echo "[wsm-p9-bridge-retire-refresh-guard] missing or not executable: $smoke" >&2
    exit 1
  fi
done

bash "$SMOKE3"
echo "[wsm-p9-bridge-retire-refresh-guard] ok"
