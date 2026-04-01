#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

bash "$ROOT/tools/smokes/v2/profiles/integration/core/phase2120/run_pure_capi_canaries.sh"
bash "$ROOT/tools/smokes/v2/profiles/integration/core/phase2120/run_vm_adapter_legacy_cluster.sh"

echo "[phase2120] Done."
