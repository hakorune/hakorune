#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_core_context_scalar_counter_vocabulary_inventory.py --check-reference

cat <<'REPORT'
output_contract=rust-mirbuilder-core-context-scalar-counter-vocabulary-v0
core_context_scalar_counter_vocabulary_recorded=1
subject=CoreContext
consultation_only=1
nightly_rustc_adapter=0
route_selection=0
summary=ok
REPORT
