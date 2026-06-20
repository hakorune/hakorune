#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC="docs/development/current/main/design/trim-route-lowering-inventory.md"
CARD="docs/development/current/main/phases/phase-296x/296x-1436-TRIM-ROUTE-LOWERING-INVENTORY-001.md"
TRIM_SRC="src/mir/loop_route_detection/support/trim.rs"
CARRIER_SRC="src/mir/loop_route_detection/support/body_local/carrier.rs"
INFO_SRC="src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs"
SHAPE_SRC="src/mir/builder/control_flow/facts/route_shape_recognizers/skip_whitespace.rs"

python3 - <<'PY' "$DOC" "$CARD" "$TRIM_SRC" "$CARRIER_SRC" "$INFO_SRC" "$SHAPE_SRC"
from pathlib import Path
import sys

doc, card, trim_src, carrier_src, info_src, shape_src = [
    Path(arg).read_text() for arg in sys.argv[1:]
]

assert "trim_route_lowering_inventory=1" in doc
assert "trim_route_lowering_owner_selected=0" in doc
assert "trim_route_lowering_implemented=0" in doc
assert "promoted_name_resolution_still_denied=1" in doc
assert "emitter_surface_does_not_lower_trim=1" in doc
assert "TRIM-ROUTE-LOWERING-DECISION-PROBE-001" in doc
assert "do not implement trim route lowering in this inventory" in doc
assert "do_not_implement_trim_route_lowering=1" in card
assert "trim_route_lowering_implementation_started=0" in card

for token in [
    "pub struct TrimLoopHelper",
    "pub fn carrier_type",
    "pub fn initial_value",
    "pub fn whitespace_count",
    "pub fn is_whitespace",
    "pub fn has_valid_structure",
]:
    assert token in trim_src, token

assert "pub fn to_carrier_info" in carrier_src
assert "carrier_info.trim_helper = Some" in carrier_src
assert "promoted_body_locals" in carrier_src
assert "pub fn trim_helper" in info_src
assert "self.trim_helper.as_ref()" in info_src
assert "resolve_promoted_join_id" in info_src
assert "detect_skip_whitespace_shape" in shape_src
PY

cat <<'REPORT'
output_contract=rust-lifecycle-trim-route-lowering-inventory-v0
trim_route_lowering_boundary_documented=1
trim_route_lowering_implementation_started=0
trim_helper_metadata_owner_preserved=1
promoted_body_locals_boundary_preserved=1
promoted_name_resolution_deny_preserved=1
backend_behavior_changed=0
summary=ok
REPORT
