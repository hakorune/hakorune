#!/usr/bin/env bash
set -euo pipefail

# Stable historical entrypoint. The two call-name predicate surfaces were
# atomically replaced by the neutral CallNameClassification policy.
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
bash "$ROOT_DIR/tools/checks/mir_builder_calltarget_owner_guard.sh"

echo "call_lowering_policy_subcluster=transported_to_call_name_classification"
