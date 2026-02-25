#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT_DIR"

echo "[vm-plugin-smoke] delegate: phase29cc CounterBox pilot smoke"
bash tools/smokes/v2/profiles/integration/apps/phase29cc_plg03_counterbox_pilot_vm.sh
