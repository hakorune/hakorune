#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

SMOKE="tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_dualrun_vm.sh"

if [ ! -x "$SMOKE" ]; then
  echo "[plg07-dualrun-guard] missing or not executable: $SMOKE" >&2
  exit 1
fi

echo "[plg07-dualrun-guard] run dual-run parity smoke"
bash "$SMOKE"
echo "[plg07-dualrun-guard] ok"
