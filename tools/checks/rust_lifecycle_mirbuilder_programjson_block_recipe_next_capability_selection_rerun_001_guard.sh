#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-block-recipe-next-capability-selection-rerun-001-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-block-recipe-next-capability-selection-rerun-001-v0.json"
PREVIOUS_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_stmt_only_block_recipe_snapshot_retire_rust_astnode_projector_candidate_guard.sh"
EXISTING_REDUCER="$ROOT_DIR/lang/src/compiler/lib/stmt_only_block_recipe.hako"
IMPLEMENTATION_OWNER="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_stmt_only_block_recipe_snapshot.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PREVIOUS_GUARD" "$EXISTING_REDUCER" "$IMPLEMENTATION_OWNER"

PREVIOUS_OUT="$(bash "$PREVIOUS_GUARD")"
for required in \
  '^summary=ok$' \
  '^retire_candidate=ProgramJsonStmtOnlyBlockRecipeSnapshotV1$' \
  '^covered_rows=4$' \
  '^recipe_root_traversal_used=1$' \
  '^stmt_only_reducer_called=1$'
do
  if ! grep -q "$required" <<<"$PREVIOUS_OUT"; then
    printf '%s\n' "$PREVIOUS_OUT" >&2
    guard_fail "$TAG" "previous ProgramJSON StmtOnly bridge retire-candidate drift: $required"
  fi
done

python3 - "$FIXTURE" "$ROOT_DIR" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
root = Path(sys.argv[2])

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonBlockRecipeNextCapabilitySelectionRerun001V1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-NEXT-CAPABILITY-SELECTION-RERUN-001", "bad token")

policy = fixture.get("selection_policy") or {}
need(policy.get("capability_batch_required") is True, "capability batch policy missing")
need(policy.get("source_code_line_cap") == 800, "bad line cap")

candidates = fixture.get("candidates") or []
selected = [row for row in candidates if row.get("decision") == "selected"]
need(len(selected) == 1, "expected exactly one selected candidate")
need(selected[0].get("name") == "ProgramJsonStmtOnlyBlockRecipeLoopNoExitSnapshotV1", "bad selected candidate")

cap = fixture.get("selected_capability") or {}
need(cap.get("name") == "ProgramJsonStmtOnlyBlockRecipeLoopNoExitSnapshotV1", "bad selected capability")
need(cap.get("target_existing_reducer") == "lang/src/compiler/lib/stmt_only_block_recipe.hako", "bad reducer")
need((root / cap.get("target_existing_reducer", "")).exists(), "missing reducer")
owner = root / (cap.get("implementation_owner") or "")
need(owner.exists(), "missing implementation owner")
need(owner.read_text(encoding="utf-8").count("\n") + 1 <= 800, "source line cap exceeded")
need(cap.get("source_rows") == ["local_loop_no_exit"], "bad source rows")
need(cap.get("next_card") == "MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-LOOP-NO-EXIT-SNAPSHOT-PARITY-001", "bad next card")

acceptance = fixture.get("acceptance") or {}
for key in [
    "must_consume_programjson_structure",
    "must_traverse_recipe_root",
    "must_call_existing_stmt_only_reducer",
    "implementation_card_required",
    "parity_gate_required",
]:
    need(acceptance.get(key) == 1, f"missing acceptance: {key}")
need(acceptance.get("minimum_parity_row_count") == 1, "bad minimum row count")
need(acceptance.get("token_snapshot_only") == 0, "token snapshot only forbidden")
need(acceptance.get("string_only_facade") == 0, "string-only facade forbidden")

stops = fixture.get("stop_conditions") or {}
for key in [
    "prebuilt_token_snapshot_input",
    "source_contains_or_regex_proof",
    "rust_astnode_projector_used_as_target_input",
    "mir_mutation_or_lowering_added",
    "route_selection_added",
    "recipe_bodies_materialized",
    "recipe_matcher_execution_added",
    "unsupported_shape_silently_ignored",
]:
    need(stops.get(key) == 1, f"missing stop condition: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectProgramJsonBlockRecipeCapability", "bad decision kind")
need(decision.get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-LOOP-NO-EXIT-SNAPSHOT-PARITY-001", "bad selected next card")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    need(value == 0, f"forbidden claim drift: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-block-recipe-next-capability-selection-rerun-001-guard-v0
token=MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-NEXT-CAPABILITY-SELECTION-RERUN-001
selected_capability=ProgramJsonStmtOnlyBlockRecipeLoopNoExitSnapshotV1
selected_next_card=MIRBUILDER-PROGRAMJSON-STMT-ONLY-BLOCK-RECIPE-LOOP-NO-EXIT-SNAPSHOT-PARITY-001
source_rows=local_loop_no_exit
must_traverse_recipe_root=1
must_call_existing_stmt_only_reducer=1
token_snapshot_only=0
string_only_facade=0
source_code_line_cap=800
implementation_done=0
parity_gate_green=0
source_selfhost_claim=0
summary=ok
REPORT
