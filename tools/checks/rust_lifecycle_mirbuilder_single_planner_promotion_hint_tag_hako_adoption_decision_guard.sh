#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-single-planner-promotion-hint-tag-hako-adoption-decision-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

DECISION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-single-planner-promotion-hint-tag-hako-adoption-decision-v0.json"
ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-single-planner-promotion-hint-tag-rust-oracle-v0.json"
HAKO_SOURCE="$ROOT_DIR/lang/src/compiler/lib/single_planner_promotion_hint_tag.hako"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_single_planner_promotion_hint_tag_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$DECISION" "$ORACLE" "$HAKO_SOURCE" "$PARITY_GATE"

python3 - "$DECISION" "$ORACLE" <<'PY'
import hashlib
import json
import sys
from pathlib import Path

decision = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
oracle = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(decision.get("schema_version") == 0, "bad schema_version")
need(decision.get("kind") == "MirBuilderSinglePlannerPromotionHintTagHakoAdoptedDecisionV1", "bad kind")
need(decision.get("token") == "MIRBUILDER-SINGLE-PLANNER-PROMOTION-HINT-TAG-HAKOADOPTED-DECISION-001", "bad token")

state = decision.get("input_state") or {}
hako = Path(state.get("hako_source") or "")
oracle_path = Path(state.get("rust_oracle_fixture") or "")
gate = Path(state.get("parity_gate") or "")
need(str(hako) == "lang/src/compiler/lib/single_planner_promotion_hint_tag.hako", "bad hako source")
need(str(oracle_path) == "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-single-planner-promotion-hint-tag-rust-oracle-v0.json", "bad oracle")
need(str(gate) == "tools/checks/rust_lifecycle_mirbuilder_single_planner_promotion_hint_tag_parity_gate.sh", "bad gate")

def sha256(path):
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()

need(sha256(hako) == state.get("hako_source_hash"), "hako hash drift")
need(sha256(oracle_path) == state.get("rust_oracle_fixture_hash"), "oracle hash drift")
need(sha256(gate) == state.get("parity_gate_hash"), "gate hash drift")

scope = decision.get("adoption_scope") or {}
need(scope.get("adopted_owner") == "single_planner_promotion_hint_tag.authority_facade", "bad owner")
need(scope.get("input_contract") == "BackendSafeSinglePlannerPromotionHintTagTokenSnapshotV1", "bad input contract")

owned = set(scope.get("owned_semantics") or [])
for field in [
    "promotion_shape_token",
    "promotion_hint_tag",
    "skip_without_promotion_shape",
    "unsupported_token_reject_reason",
]:
    need(field in owned, f"missing owned semantic: {field}")

excluded = set(scope.get("excluded_semantics") or [])
for field in [
    "promotion_shape_extraction",
    "log_emission",
    "route_execution",
    "backend_lowering",
    "MIR_mutation",
    "ID_allocation",
]:
    need(field in excluded, f"missing excluded semantic: {field}")

parity = decision.get("parity") or {}
need(parity.get("gate_status") == "Green", "parity must be Green")
need(parity.get("oracle_row_count") == 4, "row count must be 4")
rows = {row.get("case_id"): row for row in oracle.get("rows") or []}
need(rows["skip_no_promotion_shape"]["expected_summary"].endswith("no_promotion_shape"), "skip row drift")
need(rows["emit_trim_seg_tag"]["expected_summary"].endswith("[plan/loop_break/promotion_hint:TrimSeg]"), "trim row drift")
need(rows["emit_digit_pos_tag"]["expected_summary"].endswith("[plan/loop_break/promotion_hint:DigitPos]"), "digit row drift")
need(rows["reject_unknown_shape"]["expected_summary"] == "accepted=0;reason=unsupported_promotion_shape_token", "reject row drift")

decision_row = decision.get("decision") or {}
need(decision_row.get("kind") == "HakoAdoptedScoped", "bad decision kind")
need(decision_row.get("selected_next_card") == "MIRBUILDER-PLAN-TRACK-NEXT-PILOT-SELECTION-006", "bad next card")

claims = decision.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "promotion_shape_extraction_migrated",
    "log_emission_migrated",
    "route_execution_migrated",
    "backend_lowering_migrated",
    "mir_mutation_migrated",
    "id_allocation_migrated",
    "hako_generation",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")
PY

bash "$PARITY_GATE" >/dev/null

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-single-planner-promotion-hint-tag-hako-adoption-decision-guard-v0
token=MIRBUILDER-SINGLE-PLANNER-PROMOTION-HINT-TAG-HAKOADOPTED-DECISION-001
owner=single_planner_promotion_hint_tag.authority_facade
decision=HakoAdoptedScoped
parity_gate=green
oracle_rows=4
source_selfhost_claim=0
promotion_shape_extraction_migrated=0
log_emission_migrated=0
route_execution_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
selected_next_card=MIRBUILDER-PLAN-TRACK-NEXT-PILOT-SELECTION-006
summary=ok
REPORT
