#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_type_context_bounded_map_slice_inventory.py --check-reference

cat <<'REPORT'
output_contract=rust-mirbuilder-type-context-bounded-map-slice-v0
type_context_bounded_map_slice_recorded=1
subject=TypeContext
consultation_only=1
route_selection=0
nightly_rustc_adapter=0
summary=ok
REPORT
