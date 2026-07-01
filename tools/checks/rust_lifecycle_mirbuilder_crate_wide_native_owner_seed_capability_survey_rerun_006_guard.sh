#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-006-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_crate_wide_native_owner_seed_capability_survey_rerun_006.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/1973-MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-006.md"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-006"
need(fixture.get("kind") == "MirBuilderCrateWideNativeOwnerSeedCapabilitySurveyRerunV6", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

delta = fixture.get("adoption_delta") or {}
need(delta.get("family_id") == "hakorune_mir_builder::type_context", "adoption delta family drift")
need(delta.get("decision") == "Adopt", "adoption delta decision drift")
need(delta.get("hako_adopted") == 1, "type_context must be adopted")
need(delta.get("source_selfhost_claim") == 0, "Source Selfhost must remain unclaimed")

pool = fixture.get("candidate_pool") or {}
need(pool.get("verified_hako_family_ir_count") == 47, "verified count drift")
need(pool.get("bridge_eligible_count") == 0, "eligible count drift")
need(pool.get("already_adopted_count") == 12, "already adopted count drift")

selected = fixture.get("selected_candidate") or {}
need(selected.get("owner_edge_id") is None, "selected owner must be null")
need(selected.get("selected_next_card") == "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001", "bad selected next")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "bad decision kind")
need(decision.get("selected_owner_edge_id") is None, "bad decision owner")
need(decision.get("selected_next_card") == "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001", "bad decision next")
need(decision.get("reason_token") == "NoBridgeEligibleStrictEmissionNativeSeedCandidateAfterTypeContextAdoption", "bad reason")

claims = fixture.get("claims") or {}
for key in ["type_context_adoption_consumed", "bridge_policy_consumed", "strict_converter_emission_probe_consumed"]:
    need(claims.get(key) == 1, f"{key} must be 1")
for key in [
    "manual_family_selection",
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"{key} must be 0")

print("output_contract=rust-lifecycle-mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-006")
print("type_context_hako_adopted=1")
print("bridge_eligible_count=0")
print("decision=KeepStopped")
print("selected_next_card=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001")
print("manual_family_selection=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
