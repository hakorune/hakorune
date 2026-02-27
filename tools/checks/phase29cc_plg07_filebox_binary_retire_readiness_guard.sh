#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC="docs/development/current/main/phases/phase-29cc/29cc-183-plg07-min6-filebox-retire-readiness-lock-ssot.md"
SMOKE="tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_retire_readiness_lock_vm.sh"

if [ ! -f "$DOC" ]; then
  echo "[plg07-retire-readiness-guard] missing lock doc: $DOC" >&2
  exit 1
fi

for needle in \
  "PLG-07-min6" \
  "retire readiness" \
  "NYASH_PLG07_COMPAT_RUST" \
  "NYASH_PLG07_DUALRUN"; do
  if ! rg -q "$needle" "$DOC"; then
    echo "[plg07-retire-readiness-guard] missing keyword in lock doc: $needle" >&2
    exit 1
  fi
done

if [ ! -x "$SMOKE" ]; then
  echo "[plg07-retire-readiness-guard] missing or not executable: $SMOKE" >&2
  exit 1
fi

bash tools/checks/phase29cc_plg07_filebox_binary_default_switch_guard.sh
echo "[plg07-retire-readiness-guard] ok"
