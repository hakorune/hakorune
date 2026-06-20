#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

root = Path(".")
base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")

source = (root / "src/mir/loop_route_detection/support/body_local/carrier.rs").read_text()
trim = (root / "src/mir/loop_route_detection/support/trim.rs").read_text()
assert "pub fn to_carrier_info(&self)" in source
assert "carrier_info.trim_helper = Some(TrimLoopHelper::from_route_info(self));" in source
assert ".promoted_body_locals" in source
assert ".push(self.var_name.clone())" in source
assert "pub fn from_route_info(info: &TrimRouteInfo) -> Self" in trim

facts = json.loads((base / "trim-helper-producer-facts-v0.json").read_text())
plan = json.loads((base / "trim-helper-producer-plan-v0.json").read_text())
oracle = json.loads((base / "trim-helper-producer-oracle-vectors-v0.json").read_text())

assert facts["schema_version"] == 0
assert facts["kind"] == "RustLifecycleFacts"
method = facts["method_fact"]
assert method["id"] == "TrimRouteInfo::to_carrier_info"
assert method["operation"] == "TrimHelperCarrierProducer"
assert method["receiver"]["borrow_view"] == "ReadOnlyBorrowView"
assert method["receiver"]["escapes"] is False
assert method["produces"]["carrier_info"] is True
assert method["produces"]["trim_helper"] is True
assert method["produces"]["trim_helper_source"] == "TrimLoopHelper::from_route_info"
assert method["produces"]["promoted_body_local_recorded"] is True
assert method["produces"]["join_id_producer"] is False
assert method["publication"]["publishes_variable_map"] is False
assert "trim route lowering" in facts["denied_followups"]
assert "promoted_body_locals lifecycle owner" in facts["denied_followups"]

entry = plan["plans"][0]
assert entry["id"] == "TrimRouteInfo::to_carrier_info"
assert entry["plan_kind"] == "TrimHelperCarrierProducer"
assert entry["mutation_policy"] == "create_new_carrier_info"
assert entry["publication_policy"] == "does_not_publish_variable_map"
assert entry["producer_policy"]["loop_var_name"] == "carrier_name"
assert entry["producer_policy"]["loop_var_id"] == "placeholder_ValueId_0"
assert entry["producer_policy"]["carriers"] == "empty"
assert entry["producer_policy"]["trim_helper"] == "Some(TrimLoopHelper::from_route_info)"
assert entry["producer_policy"]["promoted_body_locals"] == "append_original_var"
assert entry["output_policy"]["join_id"] == "not_produced"
assert "trim route lowering" in plan["denied"]
assert "promoted_body_locals lifecycle owner" in plan["denied"]
assert plan["behavior"]["general_resolver_implemented"] is False
assert plan["behavior"]["converter_emission_added"] is False
assert plan["behavior"]["trim_route_lowering_claim"] is False
assert plan["behavior"]["promoted_body_locals_owner_claim"] is False

vectors = {row["id"]: row for row in oracle["vectors"]}
space = vectors["space_tab_trim_info"]
assert space["expect"]["loop_var_name"] == "is_ch_match"
assert space["expect"]["loop_var_id"] == 0
assert space["expect"]["carriers"] == []
assert space["expect"]["trim_helper"]["original_var"] == "ch"
assert space["expect"]["trim_helper"]["whitespace_chars"] == [" ", "\\t"]
assert space["expect"]["promoted_body_locals"] == ["ch"]
assert space["expect"]["join_id_produced"] is False

newline = vectors["newline_trim_info"]
assert newline["expect"]["trim_helper"]["whitespace_chars"] == ["\\n", "\\r"]
assert "trim_route_lowering" in oracle["denied_vectors"]
assert oracle["promotion_scope"]["hako_authority"] == "TrimRouteInfo::to_carrier_info producer only"
assert oracle["promotion_scope"]["trim_route_lowering_claim"] is False
assert oracle["promotion_scope"]["promoted_body_locals_owner_claim"] is False
assert oracle["promotion_scope"]["phi_join_id_claim"] is False
PY

cat <<'REPORT'
output_contract=rust-lifecycle-trim-helper-producer-v0
trim_helper_producer_source_shape=green
trim_helper_producer_facts_fixture=green
trim_helper_producer_plan_fixture=green
trim_helper_producer_oracle_vectors=green
produces_trim_helper=1
records_promoted_body_local=1
join_id_producer=0
trim_route_lowering_claim=0
general_resolver_implemented=0
summary=ok
REPORT
