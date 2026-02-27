#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC="docs/development/current/main/phases/phase-29cc/29cc-203-wsm-p10-min10-native-promotion-closeout-lock-ssot.md"
SMOKE="tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min10_native_promotion_closeout_lock_vm.sh"
DEV_GATE="tools/checks/dev_gate.sh"

if [ ! -f "$DOC" ]; then
  echo "[wsm-p10-min10-guard] missing lock doc: $DOC" >&2
  exit 1
fi

for needle in \
  "WSM-P10-min10" \
  "wsm.p10.main_loop_extern_call.warn.fixed4.v0" \
  "wsm.p10.main_loop_extern_call.info.fixed4.v0" \
  "wsm.p10.main_loop_extern_call.error.fixed4.v0" \
  "wsm.p10.main_loop_extern_call.debug.fixed4.v0" \
  "warn.fixed3.inventory.v0" \
  "info.fixed3.inventory.v0" \
  "error.fixed3.inventory.v0" \
  "debug.fixed3.inventory.v0" \
  "monitor-only"; do
  if ! rg -Fq "$needle" "$DOC"; then
    echo "[wsm-p10-min10-guard] missing keyword in lock doc: $needle" >&2
    exit 1
  fi
done

for needle in \
  "phase29cc_wsm_p10_native_promotion_closeout_guard.sh" \
  "WSM-P10 native promotion closeout guard"; do
  if ! rg -Fq "$needle" "$DEV_GATE"; then
    echo "[wsm-p10-min10-guard] dev_gate missing min10 closeout step: $needle" >&2
    exit 1
  fi
done

if [ ! -x "$SMOKE" ]; then
  echo "[wsm-p10-min10-guard] missing or not executable: $SMOKE" >&2
  exit 1
fi

bash "$SMOKE"
echo "[wsm-p10-min10-guard] ok"
