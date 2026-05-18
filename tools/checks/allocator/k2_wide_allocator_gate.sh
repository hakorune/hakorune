#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

source "$ROOT_DIR/tools/checks/allocator/lib/guard_group.sh"

STEPS_FILE="tools/checks/allocator/k2_wide_allocator_gate.steps"

# allocator-wide stays outside the manifest_runner pilot; only the step inventory
# is externalized so the public gate contract and execution model stay unchanged.
if [[ "${1:-}" == "--list" ]]; then
  bash_guard_group_list "k2-wide-allocator-gate" "$STEPS_FILE"
  exit 0
fi

bash_guard_group_run "k2-wide-allocator-gate" "$STEPS_FILE"
