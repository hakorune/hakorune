#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-003-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_strict_converter_emission_native_seed_candidate_selection_rerun_003.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())

def die(message: str) -> None:
    print(f"[strict-candidate-selection-rerun-003-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("kind") != "MirBuilderStrictConverterEmissionNativeSeedCandidateSelectionRerun003V1":
    die("fixture kind mismatch")
if data.get("token") != "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-003":
    die("fixture token mismatch")

pool = data.get("candidate_pool") or {}
expected = {
    "input_owner_edge_count": 3,
    "already_hako_adopted_count": 1,
    "bridge_eligible_remaining_count": 2,
    "bridge_blocked_remaining_count": 0,
    "selected_candidate_count": 1,
}
for key, value in expected.items():
    if pool.get(key) != value:
        die(f"candidate pool drift: {key}")

rows = data.get("candidate_rows") or []
if len(rows) != 3:
    die("candidate rows must be 3")
adopted = [row for row in rows if row.get("already_hako_adopted")]
if len(adopted) != 1 or adopted[0].get("owner_edge_id") != "hakorune_mir_builder::direct_state_plan_refresh":
    die("direct_state_plan_refresh must be the only adopted row")
for row in rows:
    if row.get("owner_edge_id") == "hakorune_mir_builder::direct_state_plan_refresh":
        if row.get("selection_eligible_after_adoption") is not False:
            die("adopted row must not remain selectable")
        if row.get("blocked_by_after_adoption") != ["AlreadyHakoAdopted"]:
            die("adopted row blocker mismatch")
    else:
        if row.get("selection_eligible_after_adoption") is not True:
            die("non-adopted bridge row must remain selectable")
        if not row.get("next_card", "").endswith("-HAKO-NATIVE-SOURCE-SEED-001"):
            die("remaining row next card must be native seed")

decision = data.get("decision") or {}
if decision.get("kind") != "SelectNativeSeedCandidate":
    die("decision kind mismatch")
if decision.get("reason_token") != "PostDirectStateAdoptionStrictEmissionCandidateSelected":
    die("decision reason mismatch")
if decision.get("selected_owner_edge_id") != "hakorune_mir_builder::record_packed_layout_refresh":
    die("selected owner must follow stable lexical priority after direct_state adoption")
if decision.get("selected_next_card") != "MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-HAKO-NATIVE-SOURCE-SEED-001":
    die("selected next card mismatch")

claims = data.get("claims") or {}
for key in [
    "previous_rerun_consumed",
    "direct_state_plan_refresh_adoption_consumed",
]:
    if claims.get(key) != 1:
        die(f"{key} must be 1")
for key in [
    "manual_family_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "seed_eligibility_from_forbidden_nonclaim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_canonical_mir_instruction",
    "new_python_semantic_projector",
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runner_semantic_owner",
]:
    if claims.get(key) != 0:
        die(f"claim must remain 0: {key}")

print("[strict-candidate-selection-rerun-003-guard] OK")
PY
