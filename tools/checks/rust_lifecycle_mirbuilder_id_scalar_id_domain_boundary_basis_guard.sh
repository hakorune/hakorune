#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-id-domain-boundary-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_id_domain_boundary_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2034-MIRBUILDER-ID-SCALAR-ID-DOMAIN-BOUNDARY-BASIS-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-ID-SCALAR-ID-DOMAIN-BOUNDARY-BASIS-001"
next_card = "MIRBUILDER-ID-SCALAR-STATE-MUTATION-FRAME-BASIS-001"

need(fixture.get("kind") == "MirBuilderIdScalarIdDomainBoundaryBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

previous = fixture.get("previous_state") or {}
need(previous.get("selected_component_id") == "IdDomainBoundary", "previous selected component drift")
need(previous.get("selected_next_card") == token, "previous next-card drift")
need(previous.get("native_seed_file_boundary_derivable_count") == 2, "boundary count drift")

policy = fixture.get("boundary_policy") or {}
need(policy.get("nominal_transport_required") is True, "nominal transport drift")
for key in ["raw_i64_interchangeability", "cross_domain_assignment", "sentinel_semantics_inferred", "reserved_id_semantics_inferred"]:
    need(policy.get(key) is False, f"policy drift: {key}")
need(policy.get("invalid_id_behavior_declared") is True, "invalid behavior drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("bounded_owner_count") == 2, "bounded owner drift")
need(pool.get("id_domain_boundary_count") == 3, "domain count drift")
need(pool.get("directable_row_count") == 11, "directable row drift")
need(pool.get("raw_i64_interchangeability_count") == 0, "raw i64 count drift")
need(pool.get("cross_domain_assignment_count") == 0, "cross-domain count drift")

domains = {row["canonical_id_type"]: row for row in fixture.get("domain_boundaries") or []}
for canonical in ["BasicBlockId", "BindingId", "ValueId"]:
    row = domains[canonical]
    need(row["nominal_transport"] == f"{canonical}AsI64", f"transport drift: {canonical}")
    need(row["raw_i64_interchangeability"] is False, f"raw i64 drift: {canonical}")
    need(row["cross_domain_assignment"] is False, f"cross-domain drift: {canonical}")
    need(row["invalid_or_missing_id_behavior"] == "DenyInvalidOrMissingId", f"invalid behavior drift: {canonical}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "IdDomainBoundaryBasisDefined", "decision kind drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_owner_selection",
    "source_plan_materialization",
    "behavior_recipe_materialization",
    "verifier_result_materialization",
    "derived_artifact_seed_draft_materialization",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
for needle in [
    token,
    "id_domain_boundary_count = 3",
    "selected_next_card = MIRBUILDER-ID-SCALAR-STATE-MUTATION-FRAME-BASIS-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-id-domain-boundary-basis")
print("id_domain_boundary_count=3")
print("selected_next_card=MIRBUILDER-ID-SCALAR-STATE-MUTATION-FRAME-BASIS-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY
