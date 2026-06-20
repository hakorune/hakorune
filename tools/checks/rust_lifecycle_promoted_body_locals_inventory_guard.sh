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
scope = (root / "src/mir/join_ir/lowering/scope_manager.rs").read_text()
trim_producer = (root / "src/mir/loop_route_detection/support/body_local/carrier.rs").read_text()
digitpos = (root / "src/mir/loop_route_detection/support/body_local/digitpos.rs").read_text()

assert "pub promoted_body_locals: Vec<String>" in types
assert "promoted_body_locals: Vec::new()" in impls
assert ".promoted_body_locals" in trim_producer
assert ".push(self.var_name.clone())" in trim_producer
assert ".promoted_body_locals" in digitpos
assert ".push(detection.var_name.clone())" in digitpos
assert "for promoted_var in &other.promoted_body_locals" in impls
assert "self.promoted_body_locals.push(promoted_var.clone());" in impls
assert "pub fn resolve_promoted_join_id(&self, original_name: &str)" in impls
assert "promoted_body_locals" in impls
assert "self.carrier_info.resolve_promoted_join_id(name)" in scope

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
resolver = json.loads((base / "hako-lifecycle-resolver-readonly-diagnostics-v0.json").read_text())
deny = {row["id"]: row for row in resolver["deny"]}
promoted_deny = deny["CarrierInfo.promoted_body_locals.lifecycle_owner"]
assert promoted_deny["decision"] == "DenyUnresolvedBoundary"
assert promoted_deny["reason"] == "promotion_owner_not_selected"

verifier = json.loads((base / "carrier-info-merge-from-verifier-result-v0.json").read_text())
assert "promoted_body_locals lifecycle owner" in set(verifier["denied_boundaries"])

emitter = json.loads((base / "carrier-info-merge-from-emitter-verifier-result-v0.json").read_text())
assert "promoted_body_locals lifecycle owner" in set(emitter["denied_boundaries"])

trim_facts = json.loads((base / "trim-helper-producer-facts-v0.json").read_text())
assert trim_facts["method_fact"]["produces"]["promoted_body_local_recorded"] is True
assert "promoted_body_locals lifecycle owner" in trim_facts["denied_followups"]

trim_plan = json.loads((base / "trim-helper-producer-plan-v0.json").read_text())
entry = trim_plan["plans"][0]
assert entry["producer_policy"]["promoted_body_locals"] == "append_original_var"
assert entry["output_policy"]["promoted_body_locals"] == "recorded_only"
assert trim_plan["behavior"]["promoted_body_locals_owner_claim"] is False

inventory = Path("docs/development/current/main/design/promoted-body-locals-lifecycle-inventory.md").read_text()
assert "PROMOTED-BODY-LOCALS-PRODUCER-PROBE-001" in inventory
assert "promoted_body_locals_lifecycle_owner_selected=0" in inventory
assert "join_id_producer=0" in inventory
PY

cat <<'REPORT'
output_contract=rust-lifecycle-promoted-body-locals-inventory-v0
promoted_body_locals_field_present=1
default_constructors_start_empty=present
trim_producer_records_name=1
digitpos_producer_records_name=1
merge_from_deduplicates_names=1
resolve_promoted_join_id_consumes_names=1
resolver_denies_promoted_body_locals_owner=green
verifier_denies_promoted_body_locals_owner=green
emitter_denies_promoted_body_locals_owner=green
join_id_producer=0
implementation_started=0
summary=ok
REPORT
