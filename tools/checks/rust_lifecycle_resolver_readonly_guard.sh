#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
from pathlib import Path

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
diag = json.loads((base / "hako-lifecycle-resolver-readonly-diagnostics-v0.json").read_text())

assert diag["schema_version"] == 0
assert diag["kind"] == "HakoLifecycleResolverDiagnostics"
assert diag["mode"] == "read_only"
assert diag["selection_owner"] is False
assert diag["converter_emission_added"] is False
assert diag["backend_behavior_changed"] is False
assert diag["verifier_promotion"] is False

allow = diag["allow"]
assert len(allow) == 7
expected_plans = {
    "binding-context-plan-v0.json",
    "variable-context-simple-map-plan-v0.json",
    "variable-context-immutable-borrow-plan-v0.json",
    "variable-context-snapshot-restore-plan-v0.json",
    "variable-context-carrier-snapshot-plan-v0.json",
    "variable-context-explicit-carrier-snapshot-plan-v0.json",
    "carrier-info-merge-from-plan-v0.json",
}
seen_plans = {row["plan_fixture"] for row in allow}
assert seen_plans == expected_plans

for row in allow:
    assert row["decision"] == "AllowPlan"
    plan_path = base / row["plan_fixture"]
    assert plan_path.exists(), row["plan_fixture"]
    plan = json.loads(plan_path.read_text())
    assert plan["kind"] == "HakoLifecyclePlan"
    behavior = plan.get("behavior", {})
    assert behavior.get("converter_emission_added") is not True
    assert behavior.get("general_resolver_implemented") is not True
    assert behavior.get("full_variable_context_claim") is not True

deny = {row["id"]: row for row in diag["deny"]}
assert deny["CarrierVar.join_id.production_lifecycle"]["decision"] == "DenyUnresolvedBoundary"
assert deny["CarrierVar.join_id.production_lifecycle"]["reason"] == "no_production_Some_ValueId_producer"
assert deny["CarrierInfo.trim_helper.lifecycle_owner"]["decision"] == "DenyUnresolvedBoundary"
assert deny["CarrierInfo.promoted_body_locals.lifecycle_owner"]["decision"] == "DenyUnresolvedBoundary"

claims = diag["claims"]
assert claims["join_id_dependent_paths_allowed"] is False
assert claims["full_variable_context_parity"] is False
assert claims["mirbuilder_wide_lifecycle"] is False
assert claims["resolver_selection_owner"] is False
PY

cat <<'REPORT'
output_contract=rust-lifecycle-resolver-readonly-v0
resolver_mode=read_only
allow_plan_count=7
deny_unresolved_boundary_count=3
selection_owner=0
converter_emission_added=0
backend_behavior_changed=0
verifier_promotion=0
join_id_dependent_paths_allowed=0
summary=ok
REPORT
