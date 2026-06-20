#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

root = Path(".")
base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")

trim_source = (root / "src/mir/loop_route_detection/support/body_local/carrier.rs").read_text()
digitpos_source = (root / "src/mir/loop_route_detection/support/body_local/digitpos.rs").read_text()
assert ".promoted_body_locals" in trim_source
assert ".push(self.var_name.clone())" in trim_source
assert ".promoted_body_locals" in digitpos_source
assert ".push(detection.var_name.clone())" in digitpos_source
assert "join_id: None" in digitpos_source
assert "CarrierRole::ConditionOnly" in digitpos_source
assert "CarrierInit::BoolConst(false)" in digitpos_source

facts = json.loads((base / "promoted-body-locals-producer-facts-v0.json").read_text())
plan = json.loads((base / "promoted-body-locals-producer-plan-v0.json").read_text())
oracle = json.loads((base / "promoted-body-locals-producer-oracle-vectors-v0.json").read_text())

assert facts["schema_version"] == 0
assert facts["kind"] == "RustLifecycleFacts"
producers = {row["id"]: row for row in facts["producer_facts"]}
trim = producers["TrimRouteInfo::to_carrier_info"]
digitpos = producers["DigitPosPromoter::try_promote"]
assert trim["records"] == "self.var_name"
assert trim["target"] == "CarrierInfo.promoted_body_locals"
assert trim["operation"] == "PromotedBodyLocalNameRecord"
assert trim["join_id_producer"] is False
assert trim["route_lowering_claim"] is False
assert digitpos["records"] == "detection.var_name"
assert digitpos["target"] == "CarrierInfo.promoted_body_locals"
assert digitpos["operation"] == "PromotedBodyLocalNameRecord"
assert digitpos["join_id_producer"] is False
assert digitpos["route_lowering_claim"] is False
assert "promoted name resolution" in facts["denied_followups"]

plans = {row["id"]: row for row in plan["plans"]}
trim_plan = plans["TrimRouteInfo::to_carrier_info.promoted_body_local"]
digitpos_plan = plans["DigitPosPromoter::try_promote.promoted_body_local"]
assert trim_plan["plan_kind"] == "PromotedBodyLocalNameRecord"
assert trim_plan["record_policy"] == "append_original_name"
assert digitpos_plan["plan_kind"] == "PromotedBodyLocalNameRecord"
assert digitpos_plan["record_policy"] == "append_original_name"
assert "join_id producer" in plan["denied"]
assert "promoted name resolution" in plan["denied"]
assert plan["behavior"]["general_resolver_implemented"] is False
assert plan["behavior"]["converter_emission_added"] is False
assert plan["behavior"]["join_id_producer"] is False
assert plan["behavior"]["route_lowering_claim"] is False
assert plan["behavior"]["promoted_name_resolution_claim"] is False

vectors = {row["id"]: row for row in oracle["vectors"]}
trim_vec = vectors["trim_records_original_var"]
digitpos_vec = vectors["digitpos_records_original_var"]
assert trim_vec["expect"]["promoted_body_locals"] == ["ch"]
assert trim_vec["expect"]["join_id_produced"] is False
assert trim_vec["expect"]["route_lowering_claim"] is False
assert digitpos_vec["expect"]["promoted_body_locals"] == ["digit_pos"]
assert digitpos_vec["expect"]["join_id_produced"] is False
assert digitpos_vec["expect"]["route_lowering_claim"] is False
assert "promoted_name_resolution" in oracle["denied_vectors"]
assert oracle["promotion_scope"]["hako_authority"] == "CarrierInfo.promoted_body_locals producer records only"
assert oracle["promotion_scope"]["join_id_producer"] is False
assert oracle["promotion_scope"]["route_lowering_claim"] is False
assert oracle["promotion_scope"]["promoted_name_resolution_claim"] is False
PY

cat <<'REPORT'
output_contract=rust-lifecycle-promoted-body-locals-producer-v0
trim_records_promoted_body_local=1
digitpos_records_promoted_body_local=1
producer_facts_fixture=green
producer_plan_fixture=green
producer_oracle_vectors=green
join_id_producer=0
route_lowering_claim=0
general_resolver_implemented=0
summary=ok
REPORT
