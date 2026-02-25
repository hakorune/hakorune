#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2051] Running selfhost pipeline v2 (v0) S1/S2 repeat..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2051/selfhost_v0_s1s2_repeat_canary_vm.sh'

echo "[phase2051] Running selfhost pipeline v2 (v0) Core exec..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2051/selfhost_v0_core_exec_rc42_canary_vm.sh'

echo "[phase2051] Running selfhost pipeline v2 (v1 minimal) PRIMARY..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2051/selfhost_v1_primary_rc42_canary_vm.sh'

echo "[phase2051] Running selfhost pipeline v2 (v1 provider) PRIMARY..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2051/selfhost_v1_provider_primary_rc42_canary_vm.sh'

echo "[phase2051] Done."
