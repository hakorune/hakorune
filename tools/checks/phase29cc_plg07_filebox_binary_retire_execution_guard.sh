#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC="docs/development/current/main/phases/phase-29cc/29cc-204-plg07-min7-filebox-retire-execution-lock-ssot.md"
TARGET="tools/vm_plugin_smoke.sh"
SMOKE="tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_retire_execution_lock_vm.sh"
DEV_GATE="tools/checks/dev_gate.sh"

if [ ! -f "$DOC" ]; then
  echo "[plg07-retire-exec-guard] missing lock doc: $DOC" >&2
  exit 1
fi

for needle in \
  "PLG-07-min7" \
  "retire execution" \
  "phase29cc_plg07_filebox_binary_hako_route_vm.sh" \
  "NYASH_PLG07_COMPAT_RUST" \
  "NYASH_PLG07_DUALRUN"; do
  if ! rg -q "$needle" "$DOC"; then
    echo "[plg07-retire-exec-guard] missing keyword in lock doc: $needle" >&2
    exit 1
  fi
done

if ! rg -q "phase29cc_plg07_filebox_binary_hako_route_vm.sh" "$TARGET"; then
  echo "[plg07-retire-exec-guard] missing default .hako route in $TARGET" >&2
  exit 1
fi

if rg -q "NYASH_PLG07_COMPAT_RUST" "$TARGET"; then
  echo "[plg07-retire-exec-guard] compat toggle still present in $TARGET" >&2
  exit 1
fi

if rg -q "NYASH_PLG07_DUALRUN" "$TARGET"; then
  echo "[plg07-retire-exec-guard] dual-run toggle still present in $TARGET" >&2
  exit 1
fi

if ! rg -q "phase29cc_plg07_filebox_binary_retire_execution_guard.sh" "$DEV_GATE"; then
  echo "[plg07-retire-exec-guard] dev_gate missing retire execution guard step" >&2
  exit 1
fi

if [ ! -x "$SMOKE" ]; then
  echo "[plg07-retire-exec-guard] missing or not executable: $SMOKE" >&2
  exit 1
fi

bash tools/checks/phase29cc_plg07_filebox_binary_default_switch_guard.sh
echo "[plg07-retire-exec-guard] ok"
