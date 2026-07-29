#!/usr/bin/env bash
set -euo pipefail

# Stable historical entrypoint. Extern membership is now one projection of the
# neutral CallNameClassification policy.
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
bash "$ROOT_DIR/tools/checks/mir_builder_calltarget_owner_guard.sh"

echo "extern_interface_registry=transported_to_call_name_classification"
