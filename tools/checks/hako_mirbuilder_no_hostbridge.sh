#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

TARGET_DIRS="lang/src/compiler/mirbuilder lang/src/mir/builder/loop_recipe"

for TARGET_DIR in $TARGET_DIRS; do
  if [ ! -d "$TARGET_DIR" ]; then
    echo "[hako-mirbuilder-no-hostbridge] ERROR: target dir not found: $TARGET_DIR" >&2
    exit 1
  fi
done

if rg -n --glob '*.hako' 'hostbridge|selfhost\.shared\.host_bridge' $TARGET_DIRS >/tmp/hako_mirbuilder_no_hostbridge_hits.$$; then
  echo "[hako-mirbuilder-no-hostbridge] ERROR: forbidden hostbridge reference found under $TARGET_DIRS" >&2
  cat /tmp/hako_mirbuilder_no_hostbridge_hits.$$ >&2
  rm -f /tmp/hako_mirbuilder_no_hostbridge_hits.$$
  exit 1
fi

rm -f /tmp/hako_mirbuilder_no_hostbridge_hits.$$
echo "[hako-mirbuilder-no-hostbridge] OK"
