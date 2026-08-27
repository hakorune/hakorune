#!/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --owner-profile integration --suite phase2170-official-owner-pack

# hv1_mircall_* wrappers remain available as legacy hv1_inline proofs,
# but they are weaker duplicates of the stronger rc/flow canaries above and
# are no longer part of the default phase2170 pack.

# dup-key non-increment now enforced

echo "[PASS] phase2170 all"
