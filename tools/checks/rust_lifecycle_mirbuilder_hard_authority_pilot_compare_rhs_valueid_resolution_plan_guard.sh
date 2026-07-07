#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-valueid-resolution-plan-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3331-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-RESOLUTION-PLAN-001.md"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-hard-authority-pilot-compare-rhs-valueid-resolution-plan-v0.json"
SELECTION_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-post-rhs-materialization-intent-next-seam-selection-v0.json"
PARITY_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-materialization-readonly-resolution-parity-v0.json"
PILOT_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-materialization-readonly-resolution-pilot-v0.json"
IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/compare_rhs_valueid_resolution_plan_snapshot.hako"
SELECTION_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_post_rhs_materialization_intent_next_seam_selection_guard.sh"
PARITY_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_materialization_readonly_resolution_parity_gate.sh"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$FIXTURE" "$SELECTION_FIXTURE" "$PARITY_FIXTURE" "$PILOT_FIXTURE" "$IMPL" "$SELECTION_GUARD" "$PARITY_GUARD" "$STATE" "$TASK_ORDER" "$INDEX" "$HAKO_BIN"

SELECTION_OUT="$(guard_cached_run "$TAG" bash "$SELECTION_GUARD")"
if ! grep -q '^post_rhs_materialization_intent_next_seam_selected=1$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "RHS plan seam selection prerequisite is not green"
fi

PARITY_OUT="$(guard_cached_run "$TAG" bash "$PARITY_GUARD")"
if ! grep -q '^rhs_valueid_resolution_plan_snapshot=1$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "RHS ValueId resolution plan parity prerequisite is not green"
fi

python3 - "$CARD" "$FIXTURE" "$SELECTION_FIXTURE" "$PARITY_FIXTURE" "$IMPL" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
import json
import sys
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
selection_path = Path(sys.argv[3])
parity_path = Path(sys.argv[4])
impl_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
index_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
selection = json.loads(selection_path.read_text(encoding="utf-8"))
parity = json.loads(parity_path.read_text(encoding="utf-8"))
impl = impl_path.read_text(encoding="utf-8")
state = state_path.read_text(encoding="utf-8")
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def need(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-RESOLUTION-PLAN-001"
output_contract = "rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-valueid-resolution-plan-v0"
blocker = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
candidate = "CompareRhsValueIdResolutionPlanBoundary"
card_rel_path = "docs/development/current/main/phases/phase-296x/3331-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-RESOLUTION-PLAN-001.md"
follow_on_cards = {
    token: card_rel_path,
    "MIRBUILDER-POST-RHS-VALUEID-PLAN-NEXT-SEAM-SELECTION-001": "docs/development/current/main/phases/phase-296x/3332-MIRBUILDER-POST-RHS-VALUEID-PLAN-NEXT-SEAM-SELECTION-001.md",
    "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-REQUEST-ABI-001": "docs/development/current/main/phases/phase-296x/3333-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-REQUEST-ABI-001.md",
}

need(f"# 3331 - {token}" in card, "card token drift")
need(output_contract in card, "card output contract drift")
need(candidate in card, "card candidate drift")

need(fixture.get("kind") == "MirBuilderHardAuthorityPilotCompareRhsValueIdResolutionPlanV1", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need(fixture.get("output_contract") == output_contract, "fixture output contract drift")
need((fixture.get("current_state") or {}).get("latest_card") == token, "fixture latest card drift")
need((fixture.get("current_state") or {}).get("current_blocker_token") == blocker, "fixture blocker drift")

need((selection.get("selected_next_seam") or {}).get("candidate_id") == candidate, "selection candidate drift")
need((selection.get("claims") or {}).get("post_rhs_materialization_intent_next_seam_selected") == 1, "selection claim drift")
need((selection.get("claims") or {}).get("next_seam_implemented") == 0, "selection must not implement seam")

need(parity.get("owner") == "CompareRhsValueIdResolutionPlanSnapshotBox", "parity owner drift")
need(parity.get("output_contract") == "CompareRhsValueIdResolutionPlanSnapshotV1", "parity output drift")
need([row.get("row_id") for row in parity.get("rows") or []] == ["intent_literal_i64", "intent_symbol_ref"], "parity row drift")
need((parity.get("claims") or {}).get("compare_rhs_materialization_readonly_resolution_parity") == 1, "parity claim drift")
need((parity.get("claims") or {}).get("rhs_value_id_resolution") == 0, "parity actual resolution drift")
need((parity.get("claims") or {}).get("source_selfhost_claim") == 0, "parity Source Selfhost drift")

pilot = fixture.get("pilot") or {}
need(pilot.get("candidate_id") == candidate, "pilot candidate drift")
need(pilot.get("owner_id") == "CompareRhsValueIdResolutionPlanSnapshotBox", "pilot owner drift")
need(pilot.get("input_surface") == "CompareRhsMaterializationIntentSnapshotV1", "input surface drift")
need(pilot.get("output_surface") == "CompareRhsValueIdResolutionPlanSnapshotV1", "output surface drift")
need(pilot.get("downstream_consumer") == "CompareRhsValueIdResolutionRequestSnapshotBox", "downstream consumer drift")
need(pilot.get("claim_ceiling") == "scoped_hard_authority_pilot", "claim ceiling drift")

need(len(impl.splitlines()) < 800, "source exceeds 800-line source limit")
for needle in [
    "build_plan_from_intent(intent): MapBox",
    '"rhs_valueid_resolution_plan_ready" => 1',
    '"literal_constant_required" => literal_required',
    '"symbol_lookup_required" => symbol_required',
    '"rhs_value_id_resolution" => 0',
    '"literal_constant_value_id_allocation" => 0',
    '"constant_mir_emission" => 0',
    '"runtime_helper_emission" => 0',
    '"local_ssa_finalize_compare_execution" => 0',
    '"mir_cmp_emission" => 0',
    '"branch_emission" => 0',
    '"basic_block_mutation" => 0',
    '"value_id_allocation" => 0',
]:
    need(needle in impl, f"implementation token missing: {needle}")

claims = fixture.get("claims") or {}
for key in [
    "hard_authority_pilot_implemented",
    "compare_rhs_valueid_resolution_plan_owner",
    "hako_semantic_plan_surface",
    "rust_oracle_parity",
    "aot_exe_guard",
    "downstream_boundary_present",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
    "hako_adopted_decision",
    "source_selfhost_claim",
    "native_seed_materialization",
    "rhs_value_id_resolution",
    "literal_constant_value_id_allocation",
    "constant_mir_emission",
    "runtime_helper_emission",
    "symbol_lookup_execution",
    "local_ssa_finalize_compare_execution",
    "mir_cmp_emission",
    "branch_emission",
    "basic_block_mutation",
    "value_id_allocation",
    "mir_mutation",
    "id_allocation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(any(f'latest_card = "{card}"' in state for card in follow_on_cards), "CURRENT_STATE latest card drift")
need(any(f'latest_card_path = "{path}"' in state for path in follow_on_cards.values()), "CURRENT_STATE latest path drift")
need(f'current_blocker_token = "{blocker}"' in state, "CURRENT_STATE blocker drift")

for needle in [
    token,
    output_contract,
    candidate,
    "compare_rhs_valueid_resolution_plan_owner = 1",
    "hard_authority_pilot_implemented = 1",
    "rhs_value_id_resolution = 0",
    "source_selfhost_claim = 0",
    blocker,
]:
    need(needle in task_order, f"task-order missing {needle}")

need("tools/checks/rust_lifecycle_mirbuilder_hard_authority_pilot_compare_rhs_valueid_resolution_plan_guard.sh" in index, "check index missing guard")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-hard-authority-rhs-valueid-plan.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/hard_authority_rhs_valueid_plan.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/hard_authority_rhs_valueid_plan.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$PILOT_FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

lines = [
    "using lang.compiler.mirbuilder.compare_rhs_valueid_resolution_plan_snapshot as CompareRhsValueIdResolutionPlanSnapshotBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for idx, row in enumerate(fixture["rows"]):
    intent_entries = ", ".join(
        json.dumps(key) + " => " + str(value)
        for key, value in row["intent"].items()
    )
    lines.append(f"    local intent_{idx} = %{{{intent_entries}}}")
    lines.append(f"    local plan_{idx} = CompareRhsValueIdResolutionPlanSnapshotBox.build_plan_from_intent(intent_{idx})")
    lines.append(f"    print(\"rhs_valueid_plan:{row['row_id']}:\" + CompareRhsValueIdResolutionPlanSnapshotBox.plan_summary(plan_{idx}))")
    expected_lines.append(f"rhs_valueid_plan:{row['row_id']}:" + row["expected_plan_summary"])

lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit RHS ValueId resolution plan pilot executable"
fi

chmod +x "$EXE"
"$EXE" >"$ACTUAL.raw"

python3 - "$EXPECTED" "$ACTUAL.raw" "$ACTUAL" <<'PY'
import sys
from pathlib import Path

expected = Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()
raw = Path(sys.argv[2]).read_text(encoding="utf-8").splitlines()
actual_path = Path(sys.argv[3])
actual = [line.strip() for line in raw if line.strip() and not line.startswith("Result:")]
actual_path.write_text("\n".join(actual) + "\n", encoding="utf-8")
if actual != expected:
    print("[hard-authority/rhs-valueid-plan] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-valueid-resolution-plan-v0
token=MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-RESOLUTION-PLAN-001
candidate=CompareRhsValueIdResolutionPlanBoundary
owner=CompareRhsValueIdResolutionPlanSnapshotBox
hard_authority_pilot_implemented=1
compare_rhs_valueid_resolution_plan_owner=1
hako_semantic_plan_surface=1
rust_oracle_parity=1
aot_exe_guard=1
downstream_boundary_present=1
hako_adopted_decision=0
source_selfhost_claim=0
rhs_value_id_resolution=0
literal_constant_value_id_allocation=0
constant_mir_emission=0
runtime_helper_emission=0
symbol_lookup_execution=0
local_ssa_finalize_compare_execution=0
mir_cmp_emission=0
branch_emission=0
basic_block_mutation=0
value_id_allocation=0
mir_mutation=0
id_allocation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
new_backend_route=0
new_abi=0
selected_next_card=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
summary=ok
REPORT
