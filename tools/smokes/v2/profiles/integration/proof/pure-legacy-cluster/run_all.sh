#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

bash "$ROOT/tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh"
bash "$ROOT/tools/smokes/v2/profiles/archive/pure-historical/run_pure_historical.sh"
bash "$ROOT/tools/smokes/v2/profiles/integration/proof/vm-adapter-legacy/run_vm_adapter_legacy_cluster.sh"
bash "$ROOT/tools/smokes/v2/profiles/integration/proof/native-reference/run_native_reference_bucket.sh"

echo "[pure-legacy-cluster] legacy cluster done."
