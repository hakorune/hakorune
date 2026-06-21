#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --all --check
bash tools/checks/rust_lifecycle_variable_context_immutable_borrow_derived_route_selection_guard.sh

cat <<'REPORT'
output_contract=rust-mirbuilder-lightweight-facts-converter-v1
binding_context=green
variable_context_simple_map=green
variable_context_snapshot_restore=green
variable_context_carrier_snapshot=green
variable_context_explicit_carrier_snapshot=green
returned_read_borrow=Deny(ReturnedReadBorrow)
summary=ok
REPORT
