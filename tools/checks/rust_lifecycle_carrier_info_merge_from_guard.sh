#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
facts = json.loads((base / "carrier-info-merge-from-facts-v0.json").read_text())
plan = json.loads((base / "carrier-info-merge-from-plan-v0.json").read_text())
oracle = json.loads((base / "carrier-info-merge-from-oracle-vectors-v0.json").read_text())

assert facts["schema_version"] == 0
assert facts["kind"] == "RustLifecycleFacts"
assert facts["subject"].endswith("CarrierInfo.merge_from")
assert "variable-context-carrier-snapshot-facts-v0.json" in facts["base_facts"]
assert "variable-context-explicit-carrier-snapshot-facts-v0.json" in facts["base_facts"]

method = facts["method_fact"]
assert method["id"] == "CarrierInfo::merge_from"
assert method["operation"] == "OwnedCarrierInfoMerge"
assert method["receiver"]["ownership"] == "OwnedCarrierInfo"
assert method["receiver"]["access"] == "mutable"
assert method["receiver"]["escapes"] is False
assert method["input"]["borrow_view"] == "ReadOnlyBorrowView"
assert method["input"]["escapes"] is False
assert method["mutation"]["adds_missing_carriers"] is True
assert method["mutation"]["deduplicates_by_name"] is True
assert method["mutation"]["sorts_carriers"] is True
assert method["mutation"]["copies_trim_helper_when_present"] is True
assert method["mutation"]["deduplicates_promoted_body_locals"] is True
assert method["output"]["mutates_receiver"] is True
assert method["output"]["publishes_variable_map"] is False
assert method["output"]["join_id_producer"] is False

for item in ["join_id producer", "general resolver"]:
    assert item in set(facts["denied_followups"])

entry = plan["plans"][0]
assert entry["id"] == "CarrierInfo::merge_from"
assert entry["plan_kind"] == "OwnedCarrierInfoMerge"
assert entry["mutation_policy"] == "mutate_receiver_only"
assert entry["publication_policy"] == "does_not_publish_variable_map"
assert entry["merge_policy"]["carriers"] == "append_missing_by_name_then_sort"
assert entry["merge_policy"]["trim_helper"] == "clone_if_other_has_some"
assert entry["merge_policy"]["promoted_body_locals"] == "append_missing_owned_strings"
assert entry["output_policy"]["receiver_mutated"] is True
assert entry["output_policy"]["other_mutated"] is False
assert entry["output_policy"]["join_id"] == "not_produced"
assert "receiver.ownership=OwnedCarrierInfo" in entry["required_facts"]
assert "input.borrow_view=ReadOnlyBorrowView" in entry["required_facts"]

behavior = plan["behavior"]
assert behavior["general_resolver_implemented"] is False
assert behavior["converter_emission_added"] is False
assert behavior["phi_join_id_claim"] is False
assert behavior["full_variable_context_claim"] is False

vectors = {row["id"]: row for row in oracle["vectors"]}
append = vectors["append_missing_carrier_and_sort"]
assert [row["name"] for row in append["expect"]["carriers"]] == ["count", "sum"]
assert append["expect"]["receiver_mutated"] is True
assert append["expect"]["other_mutated"] is False
assert "sort_after_merge" in append["requires"]

dup = vectors["duplicate_carrier_is_not_added"]
assert [row["name"] for row in dup["expect"]["carriers"]] == ["sum"]
assert dup["expect"]["trim_helper"] == "cloned_from_other"
assert dup["expect"]["promoted_body_locals"] == ["digit_pos", "ch"]
assert dup["expect"]["join_id_produced"] is False

scope = oracle["promotion_scope"]
assert scope["hako_authority"] == "CarrierInfo::merge_from owned mutation only"
assert scope["phi_join_id_claim"] is False
assert scope["full_variable_context_claim"] is False
assert scope["mirbuilder_wide_claim"] is False
PY

cat <<'REPORT'
output_contract=rust-lifecycle-carrier-info-merge-from-v0
carrier_info_merge_from_facts_fixture=green
carrier_info_merge_from_plan_fixture=green
carrier_info_merge_from_oracle_vectors=green
requires_owned_receiver=green
requires_readonly_other_borrow=green
deduplicate_by_name=green
sort_after_merge=green
join_id_producer=0
general_resolver_implemented=0
summary=ok
REPORT
