#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC="docs/development/current/main/phases/phase-29cc/29cc-185-wsm-p7-min2-default-hako-only-guard-lock-ssot.md"
SMOKE="tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p7_min2_default_hako_only_guard_vm.sh"
DEV_GATE="tools/checks/dev_gate.sh"

if [ ! -f "$DOC" ]; then
  echo "[wsm-p7-default-hako-only-guard] missing lock doc: $DOC" >&2
  exit 1
fi

for needle in \
  "WSM-P7-min2" \
  "default-only" \
  "NYASH_WASM_ROUTE_POLICY" \
  "allowed: default" \
  "wasm_hako_default_lane_trace_"; do
  if ! rg -q "$needle" "$DOC"; then
    echo "[wsm-p7-default-hako-only-guard] missing keyword in lock doc: $needle" >&2
    exit 1
  fi
done

if ! rg -q "phase29cc_wsm_p7_default_hako_only_guard.sh" "$DEV_GATE"; then
  echo "[wsm-p7-default-hako-only-guard] dev_gate missing p7 guard step" >&2
  exit 1
fi

if [ ! -x "$SMOKE" ]; then
  echo "[wsm-p7-default-hako-only-guard] missing or not executable: $SMOKE" >&2
  exit 1
fi

bash "$SMOKE"
echo "[wsm-p7-default-hako-only-guard] ok"
