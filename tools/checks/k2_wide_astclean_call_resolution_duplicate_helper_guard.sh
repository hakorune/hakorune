#!/usr/bin/env bash
set -euo pipefail

# Stable historical entrypoint. Duplicate call-name helpers are now retired by
# the neutral CallNameClassification policy guard.
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
bash "$ROOT_DIR/tools/checks/mir_builder_calltarget_owner_guard.sh"

echo "astclean_call_resolution_helpers=retired"
