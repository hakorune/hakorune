#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2050] Running Flow reps (phi2+branch)..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2050/flow_phi2_*'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2050/flow_phi2_select_by_pred_rc99_primary_canary_vm.sh'

echo "[phase2050] Running String reps (minimal)..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2050/string_*'

echo "[phase2050] Done."
