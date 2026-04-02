#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2100/shared] SSOT relative inference rep..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2211/ssot_relative_unique_canary_vm.sh'
