#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

root = Path(".")

types = (root / "src/mir/join_ir/lowering/carrier_info/types.rs").read_text()
impls = (root / "src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs").read_text()
carrier = (root / "src/mir/loop_route_detection/support/body_local/carrier.rs").read_text()
condition = (root / "src/mir/loop_route_detection/support/body_local/condition.rs").read_text()
trim = (root / "src/mir/loop_route_detection/support/trim.rs").read_text()

assert "pub trim_helper: Option<" in types
assert "pub struct TrimLoopHelper" in trim
assert "pub original_var: String" in trim
assert "pub carrier_name: String" in trim
assert "pub whitespace_chars: Vec<String>" in trim

assert "trim_helper: None" in impls
assert "carrier_info.trim_helper = Some(TrimLoopHelper::from_route_info(self));" in carrier
assert "carrier_info.trim_helper.is_some()" in condition
assert "self.trim_helper = other.trim_helper.clone();" in impls
assert "pub fn trim_helper(" in impls
assert "self.trim_helper.as_ref()" in impls

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")

resolver = json.loads((base / "hako-lifecycle-resolver-readonly-diagnostics-v0.json").read_text())
deny = {row["id"]: row for row in resolver["deny"]}
trim_deny = deny["CarrierInfo.trim_helper.lifecycle_owner"]
assert trim_deny["decision"] == "DenyUnresolvedBoundary"
assert trim_deny["reason"] == "route_specific_metadata_owner_not_selected"

verifier = json.loads((base / "carrier-info-merge-from-verifier-result-v0.json").read_text())
assert "trim_helper lifecycle owner" in set(verifier["denied_boundaries"])
assert verifier["claims"]["emission_allowed"] is False

emitter = json.loads((base / "carrier-info-merge-from-emitter-verifier-result-v0.json").read_text())
assert "trim_helper lifecycle owner" in set(emitter["denied_boundaries"])
assert emitter["claims"]["emission_allowed"] is True
assert emitter["claims"]["emission_scope"] == "CarrierInfo::merge_from only"

surface = (base / "carrier-info-merge-from-emitter-surface-v0.hako").read_text()
assert "Verified plan: clone trim_helper only as existing route metadata." in surface
assert "Denied boundary: no trim_helper lifecycle owner is claimed here." in surface
assert "Verified boundary: trim_helper lifecycle owner" not in surface

inventory = Path("docs/development/current/main/design/trim-helper-carrier-lifecycle-inventory.md").read_text()
assert "TRIM-HELPER-CARRIER-LIFECYCLE-PROBE-001" in inventory
assert "trim_helper_lifecycle_owner_selected=0" in inventory
assert "merge_from_claims_trim_owner=0" in inventory
PY

cat <<'REPORT'
output_contract=rust-lifecycle-trim-helper-inventory-v0
trim_helper_field_present=1
trim_loop_helper_payload_present=1
generic_carrier_constructors_trim_none=present
trim_route_producer_present=1
merge_from_clones_existing_trim_metadata=1
resolver_denies_trim_owner=green
verifier_denies_trim_owner=green
emitter_denies_trim_owner=green
trim_lifecycle_owner_selected=0
implementation_started=0
summary=ok
REPORT
