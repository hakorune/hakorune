#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-mirbuilder-post-hard-authority-pilot-next-seam-selection"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/3328-MIRBUILDER-POST-HARD-AUTHORITY-PILOT-NEXT-SEAM-SELECTION-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-post-hard-authority-pilot-next-seam-selection-v0.json"
PILOT_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-hard-authority-pilot-boolrecipe-compare-semantic-command-v0.json"
RHS_INTENT_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-materialization-intent-parity-v0.json"
RHS_INTENT_IMPL="lang/src/compiler/mirbuilder/compare_rhs_materialization_intent_snapshot.hako"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$FIXTURE" "$PILOT_FIXTURE" \
  "$RHS_INTENT_FIXTURE" "$RHS_INTENT_IMPL" "$STATE" "$TASK_ORDER" "$INDEX"

python3 - "$CARD" "$FIXTURE" "$PILOT_FIXTURE" "$RHS_INTENT_FIXTURE" "$RHS_INTENT_IMPL" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
pilot_path = Path(sys.argv[3])
rhs_fixture_path = Path(sys.argv[4])
rhs_impl_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
index_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
pilot = json.loads(pilot_path.read_text(encoding="utf-8"))
rhs = json.loads(rhs_fixture_path.read_text(encoding="utf-8"))
rhs_impl = rhs_impl_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")


def need(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-POST-HARD-AUTHORITY-PILOT-NEXT-SEAM-SELECTION-001"
output_contract = "rust-lifecycle-mirbuilder-post-hard-authority-pilot-next-seam-selection-v0"
blocker = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
selected_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-MATERIALIZATION-INTENT-001"
selected_card_path = "docs/development/current/main/phases/phase-296x/3329-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-MATERIALIZATION-INTENT-001.md"
second_follow_on_card = "MIRBUILDER-POST-RHS-MATERIALIZATION-INTENT-NEXT-SEAM-SELECTION-001"
second_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3330-MIRBUILDER-POST-RHS-MATERIALIZATION-INTENT-NEXT-SEAM-SELECTION-001.md"
third_follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-RESOLUTION-PLAN-001"
third_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3331-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-RESOLUTION-PLAN-001.md"
fourth_follow_on_card = "MIRBUILDER-POST-RHS-VALUEID-PLAN-NEXT-SEAM-SELECTION-001"
fourth_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3332-MIRBUILDER-POST-RHS-VALUEID-PLAN-NEXT-SEAM-SELECTION-001.md"
fifth_follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-REQUEST-ABI-001"
fifth_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3333-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-REQUEST-ABI-001.md"
sixth_follow_on_card = "MIRBUILDER-POST-RHS-VALUEID-REQUEST-ABI-NEXT-SEAM-SELECTION-001"
sixth_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3334-MIRBUILDER-POST-RHS-VALUEID-REQUEST-ABI-NEXT-SEAM-SELECTION-001.md"
seventh_follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001"
seventh_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3335-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001.md"
eighth_follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001"
eighth_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3336-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001.md"
ninth_follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001"
ninth_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3337-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001.md"
selected_candidate = "CompareRhsMaterializationIntentBoundary"

need(f"# 3328 - {token}" in card, "card token drift")
need(output_contract in card, "card output contract drift")
need(selected_candidate in card, "card selected candidate drift")
need(selected_card in card, "card selected next drift")

need(fixture.get("kind") == "MirBuilderPostHardAuthorityPilotNextSeamSelectionV1", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need(fixture.get("output_contract") == output_contract, "fixture output contract drift")
need((fixture.get("current_state") or {}).get("latest_card") == token, "fixture latest card drift")
need((fixture.get("current_state") or {}).get("current_blocker_token") == blocker, "fixture blocker drift")

need((pilot.get("claims") or {}).get("hard_authority_pilot_implemented") == 1, "first pilot evidence missing")
need((pilot.get("claims") or {}).get("source_selfhost_claim") == 0, "first pilot must not claim Source Selfhost")

need(rhs.get("owner") == "CompareRhsMaterializationIntentSnapshotBox", "rhs owner drift")
need(rhs.get("output_contract") == "CompareRhsMaterializationIntentSnapshotV1", "rhs output drift")
need((rhs.get("claims") or {}).get("compare_rhs_materialization_intent_parity") == 1, "rhs parity not green")
need((rhs.get("claims") or {}).get("rhs_value_id_resolution") == 0, "rhs value-id resolution must remain 0")
need((rhs.get("claims") or {}).get("source_selfhost_claim") == 0, "rhs Source Selfhost claim drift")
need(len(rhs_impl.splitlines()) < 800, "rhs intent source exceeds 800-line source limit")
for needle in [
    "build_intent_from_command(command): MapBox",
    '"rhs_materialization_intent_ready" => 1',
    '"rhs_value_id_resolution" => 0',
    '"rhs_runtime_materialization" => 0',
    '"value_id_allocation" => 0',
]:
    need(needle in rhs_impl, f"rhs implementation missing token: {needle}")

selected = fixture.get("selected_next_seam") or {}
need(selected.get("candidate_id") == selected_candidate, "selected candidate drift")
need(selected.get("owner_id") == "CompareRhsMaterializationIntentSnapshotBox", "selected owner drift")
need(selected.get("input_surface") == "CompareLoweringSymbolicCommandSnapshotV1", "input surface drift")
need(selected.get("output_surface") == "CompareRhsMaterializationIntentSnapshotV1", "output surface drift")
need(selected.get("downstream_consumer") == "CompareRhsValueIdResolutionPlanSnapshotBox", "consumer drift")
for key in ["rust_oracle_available", "hako_impl_available", "aot_guard_available", "eligible_as_next_hard_authority_seam"]:
    need(selected.get(key) == 1, f"selected positive drift: {key}")
for key in [
    "mutation_required",
    "route_selection_required",
    "runtime_switch_required",
    "source_selfhost_claim_required",
    "support_lane_projector",
    "string_only_facade",
]:
    need(selected.get(key) == 0, f"selected forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectNextHardAuthoritySeam", "decision kind drift")
need(decision.get("reason_token") == "CompareRhsMaterializationIntentIsDirectReadOnlyDownstreamSeam", "reason token drift")
need(decision.get("selected_next_card") == selected_card, "selected next drift")

claims = fixture.get("claims") or {}
for key in [
    "post_hard_authority_pilot_next_seam_selected",
    "first_hard_authority_pilot_evidence_consumed",
    "compare_rhs_materialization_intent_selected",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
    "next_seam_implemented",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "native_seed_materialization",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "mir_mutation",
    "id_allocation",
    "new_backend_route",
    "new_abi",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") in [token, selected_card, second_follow_on_card, third_follow_on_card, fourth_follow_on_card, fifth_follow_on_card, sixth_follow_on_card, seventh_follow_on_card, eighth_follow_on_card, ninth_follow_on_card], "CURRENT_STATE latest card drift")
need(state.get("latest_card_path") in [str(card_path), selected_card_path, second_follow_on_card_path, third_follow_on_card_path, fourth_follow_on_card_path, fifth_follow_on_card_path, sixth_follow_on_card_path, seventh_follow_on_card_path, eighth_follow_on_card_path, ninth_follow_on_card_path], "CURRENT_STATE latest path drift")
need(state.get("current_blocker_token") == blocker, "CURRENT_STATE blocker drift")

for needle in [
    token,
    output_contract,
    selected_candidate,
    "post_hard_authority_pilot_next_seam_selected = 1",
    "next_seam_implemented = 0",
    "source_selfhost_claim = 0",
    selected_card,
]:
    need(needle in task_order, f"task-order missing {needle}")

need("tools/checks/rust_lifecycle_mirbuilder_post_hard_authority_pilot_next_seam_selection_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("decision=SelectNextHardAuthoritySeam")
print("reason_token=CompareRhsMaterializationIntentIsDirectReadOnlyDownstreamSeam")
print(f"selected_candidate={selected_candidate}")
print(f"selected_next_card={selected_card}")
print("post_hard_authority_pilot_next_seam_selected=1")
print("next_seam_implemented=0")
print("source_selfhost_claim=0")
print("runtime_route_switch=0")
print("summary=ok")
PY
