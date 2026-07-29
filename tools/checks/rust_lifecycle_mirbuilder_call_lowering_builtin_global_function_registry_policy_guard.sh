#!/usr/bin/env bash
set -euo pipefail

# Stable historical entrypoint. Builtin-global membership is now one projection
# of the neutral CallNameClassification policy.
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
bash "$ROOT_DIR/tools/checks/mir_builder_calltarget_owner_guard.sh"

echo "builtin_global_registry=transported_to_call_name_classification"
