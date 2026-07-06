#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-canonical-loop-facts-input-snapshot-aot-boundary-design-stop-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-canonical-loop-facts-input-snapshot-aot-boundary-design-stop-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3226-MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-INPUT-SNAPSHOT-AOT-BOUNDARY-DESIGN-STOP-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CONSULTATION_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_input_boundary_consultation_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CONSULTATION_GUARD"

CONSULTATION_OUT="$(guard_cached_run "$TAG" bash "$CONSULTATION_GUARD")"
if ! grep -q '^selected_next_card=MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-INPUT-SNAPSHOT-001$' <<<"$CONSULTATION_OUT"; then
  printf '%s\n' "$CONSULTATION_OUT" >&2
  guard_fail "$TAG" "previous consultation does not select canonical loop facts input snapshot"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-INPUT-SNAPSHOT-AOT-BOUNDARY-DESIGN-STOP-001"
if fixture.get("kind") != "MirBuilderProgramJsonCanonicalLoopFactsInputSnapshotAotBoundaryDesignStopV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

blocker = fixture.get("observed_blocker") or {}
if blocker.get("kind") != "AotBoundarySelectionRequired":
    raise SystemExit("bad blocker kind")
if "module_generic_prepass_failed" not in blocker.get("string_summary_failure", ""):
    raise SystemExit("missing string boundary failure")
if "map_handle publication contract" not in blocker.get("mapbox_snapshot_boundary", ""):
    raise SystemExit("missing mapbox boundary")
if blocker.get("recipe_matcher_input_task_still_valid") is not True:
    raise SystemExit("input task must remain valid")

states = {row.get("id"): row for row in fixture.get("candidate_decisions") or []}
if states.get("A_MAPBOX_SNAPSHOT_PUBLICATION_BRIDGE", {}).get("state") != "RecommendedDefault":
    raise SystemExit("A must be recommended")
if states.get("B_COMPLEX_STRING_SUMMARY_AOT_ROUTE", {}).get("state") != "ConsultationAlternative":
    raise SystemExit("B must remain an alternative")
if states.get("C_VM_ONLY_TRAVERSAL_GATE", {}).get("state") != "RejectedForNow":
    raise SystemExit("C must be rejected for now")

decision = fixture.get("decision") or {}
if decision.get("kind") != "ConsultationRequired":
    raise SystemExit("decision must require consultation")
if decision.get("selected_next_card") != "CONSULTATION_REQUIRED":
    raise SystemExit("selected next must be consultation")

claims = fixture.get("claims") or {}
if claims.get("design_stop") != 1:
    raise SystemExit("design stop claim missing")
for key, value in claims.items():
    if key == "design_stop":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    "A_MAPBOX_SNAPSHOT_PUBLICATION_BRIDGE",
    "module_generic_prepass_failed",
    "CONSULTATION_REQUIRED",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [token, "CONSULTATION_REQUIRED", "A_MAPBOX_SNAPSHOT_PUBLICATION_BRIDGE"]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-canonical-loop-facts-input-snapshot-aot-boundary-design-stop-guard-v0
token=MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-INPUT-SNAPSHOT-AOT-BOUNDARY-DESIGN-STOP-001
design_stop=1
blocker=AotBoundarySelectionRequired
recommended_default=A_MAPBOX_SNAPSHOT_PUBLICATION_BRIDGE
selected_next_card=CONSULTATION_REQUIRED
canonical_loop_facts_input_snapshot_implemented=0
recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_route_switch=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
