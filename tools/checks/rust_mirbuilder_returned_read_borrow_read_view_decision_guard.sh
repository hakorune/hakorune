#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_returned_read_borrow_read_view_decision_inventory.py --check-reference

cat <<'REPORT'
output_contract=rust-mirbuilder-returned-read-borrow-read-view-decision-v0
returned_read_borrow_read_view_decision_recorded=1
subject=VariableContext
consultation_only=1
route_selection=0
nightly_rustc_adapter=0
summary=ok
REPORT
