#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-mirbuilder-hard-authority-pilot-boolrecipe-compare-semantic-command"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/3327-MIRBUILDER-HARD-AUTHORITY-PILOT-BOOLRECIPE-COMPARE-SEMANTIC-COMMAND-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-hard-authority-pilot-boolrecipe-compare-semantic-command-v0.json"
POLICY_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-authority-facade-hard-authority-pilot-policy-v0.json"
INTENT_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bool-recipe-compare-lowering-observe-only-pilot-v0.json"
COMMAND_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-lowering-symbolic-command-parity-v0.json"
CLOSEOUT_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-boolrecipe-to-mir-compare-branch-closeout-v0.json"
PUBLICATION_IMPL="lang/src/compiler/mirbuilder/program_json_bool_recipe_compare_publication.hako"
INTENT_IMPL="lang/src/compiler/mirbuilder/bool_recipe_compare_lowering_intent_snapshot.hako"
COMMAND_IMPL="lang/src/compiler/mirbuilder/compare_lowering_symbolic_command_snapshot.hako"
POLICY_GUARD="tools/checks/rust_lifecycle_mirbuilder_authority_facade_hard_authority_pilot_policy_guard.sh"
INTENT_GATE="tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_lowering_observe_only_pilot_gate.sh"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
HAKO_BIN="tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" \
  "$CARD" "$FIXTURE" "$POLICY_FIXTURE" "$INTENT_FIXTURE" "$COMMAND_FIXTURE" \
  "$CLOSEOUT_FIXTURE" "$PUBLICATION_IMPL" "$INTENT_IMPL" "$COMMAND_IMPL" \
  "$POLICY_GUARD" "$INTENT_GATE" "$STATE" "$TASK_ORDER" "$INDEX" "$HAKO_BIN"

POLICY_OUT="$(guard_cached_run "$TAG" bash "$POLICY_GUARD")"
if ! grep -q '^hard_authority_pilot_selected=1$' <<<"$POLICY_OUT"; then
  printf '%s\n' "$POLICY_OUT" >&2
  guard_fail "$TAG" "hard-authority pilot policy prerequisite is not green"
fi

INTENT_OUT="$(guard_cached_run "$TAG" bash "$INTENT_GATE")"
if ! grep -q '^observe_only_lowering_intent=1$' <<<"$INTENT_OUT"; then
  printf '%s\n' "$INTENT_OUT" >&2
  guard_fail "$TAG" "BoolRecipe Compare lowering intent AOT/EXE gate is not green"
fi

python3 - "$CARD" "$FIXTURE" "$POLICY_FIXTURE" "$INTENT_FIXTURE" "$COMMAND_FIXTURE" "$CLOSEOUT_FIXTURE" "$PUBLICATION_IMPL" "$INTENT_IMPL" "$COMMAND_IMPL" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
policy_path = Path(sys.argv[3])
intent_path = Path(sys.argv[4])
command_path = Path(sys.argv[5])
closeout_path = Path(sys.argv[6])
publication_impl_path = Path(sys.argv[7])
intent_impl_path = Path(sys.argv[8])
command_impl_path = Path(sys.argv[9])
state_path = Path(sys.argv[10])
task_order_path = Path(sys.argv[11])
index_path = Path(sys.argv[12])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
policy = json.loads(policy_path.read_text(encoding="utf-8"))
intent = json.loads(intent_path.read_text(encoding="utf-8"))
command = json.loads(command_path.read_text(encoding="utf-8"))
closeout = json.loads(closeout_path.read_text(encoding="utf-8"))
impls = {
    "publication": publication_impl_path.read_text(encoding="utf-8"),
    "intent": intent_impl_path.read_text(encoding="utf-8"),
    "command": command_impl_path.read_text(encoding="utf-8"),
}
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")


def need(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-HARD-AUTHORITY-PILOT-BOOLRECIPE-COMPARE-SEMANTIC-COMMAND-001"
output_contract = "rust-lifecycle-mirbuilder-hard-authority-pilot-boolrecipe-compare-semantic-command-v0"
blocker = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
candidate = "BoolRecipeCompareSemanticCommandBoundary"
follow_on_card = "MIRBUILDER-POST-HARD-AUTHORITY-PILOT-NEXT-SEAM-SELECTION-001"
follow_on_card_path = "docs/development/current/main/phases/phase-296x/3328-MIRBUILDER-POST-HARD-AUTHORITY-PILOT-NEXT-SEAM-SELECTION-001.md"

need(f"# 3327 - {token}" in card, "card token drift")
need(output_contract in card, "card output contract drift")
need(candidate in card, "card candidate drift")

need(fixture.get("kind") == "MirBuilderHardAuthorityPilotBoolRecipeCompareSemanticCommandV1", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need(fixture.get("output_contract") == output_contract, "fixture output contract drift")
need((fixture.get("current_state") or {}).get("latest_card") == token, "fixture latest card drift")
need((fixture.get("current_state") or {}).get("current_blocker_token") == blocker, "fixture blocker drift")

need((policy.get("decision") or {}).get("selected_candidate") == candidate, "policy candidate drift")
need((policy.get("claims") or {}).get("hard_authority_pilot_selected") == 1, "policy selection not green")
need((policy.get("claims") or {}).get("hard_authority_pilot_implemented") == 0, "policy card must not implement")

pilot = fixture.get("pilot") or {}
need(pilot.get("candidate_id") == candidate, "pilot candidate drift")
need(pilot.get("owner_id") == "BoolRecipeCompareLoweringIntentSnapshotBox", "pilot owner drift")
need(pilot.get("supporting_owner_id") == "CompareLoweringSymbolicCommandSnapshotBox", "supporting owner drift")
need(pilot.get("input_surface") == "BoolRecipeComparePublicationV1", "input surface drift")
need(pilot.get("output_surface") == "BoolRecipeCompareLoweringIntentSnapshotV1", "output surface drift")
need(pilot.get("semantic_command_surface") == "CompareLoweringSymbolicCommandSnapshotV1", "command surface drift")
need(pilot.get("claim_ceiling") == "scoped_hard_authority_pilot", "claim ceiling drift")

need(intent.get("owner") == "BoolRecipeCompareLoweringIntentSnapshotBox", "intent owner drift")
need(intent.get("output_contract") == "BoolRecipeCompareLoweringIntentSnapshotV1", "intent output drift")
need((intent.get("claims") or {}).get("observe_only_lowering_intent") == 1, "intent claim drift")
need((command.get("owner") == "CompareLoweringSymbolicCommandSnapshotBox"), "command owner drift")
need(command.get("output_contract") == "CompareLoweringSymbolicCommandSnapshotV1", "command output drift")
need([row.get("row_id") for row in command.get("rows") or []] == ["intent_var_le_literal", "intent_var_lt_symbol"], "command row drift")
need((command.get("claims") or {}).get("compare_lowering_symbolic_command_parity") == 1, "command parity claim drift")
need((closeout.get("claims") or {}).get("compare_branch_lowering_bridge_chain_green") == 1, "downstream closeout drift")

for name, text in impls.items():
    need(len(text.splitlines()) < 800, f"{name} source exceeds 800-line source limit")
    for forbidden in ["route_registry", "RecipeMatcherBox"]:
        need(forbidden not in text, f"{name} leaks forbidden token {forbidden}")

for needle in [
    "ProgramJsonBoolRecipeComparePublicationBox.build_publication",
    "build_intent_from_recipe(recipe): MapBox",
    '"lowering_executed" => 0',
    '"mir_cmp_emission" => 0',
    '"branch_emission" => 0',
    '"value_id_allocation" => 0',
    "build_command_from_intent(intent): MapBox",
    '"symbolic_command_ready" => 1',
    '"dst_policy_code" => 1',
    '"branch_target_policy_code" => 1',
]:
    need(any(needle in text for text in impls.values()), f"implementation token missing: {needle}")

claims = fixture.get("claims") or {}
for key in [
    "hard_authority_pilot_implemented",
    "boolrecipe_compare_semantic_command_owner",
    "hako_semantic_command_surface",
    "rust_oracle_parity",
    "aot_exe_guard",
    "downstream_boundary_present",
    "scoped_hard_authority_pilot",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
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

need(state.get("latest_card") in [token, follow_on_card], "CURRENT_STATE latest card drift")
need(state.get("latest_card_path") in [str(card_path), follow_on_card_path], "CURRENT_STATE latest path drift")
need(state.get("current_blocker_token") == blocker, "CURRENT_STATE blocker drift")

for needle in [
    token,
    output_contract,
    "hard_authority_pilot_implemented = 1",
    "boolrecipe_compare_semantic_command_owner = 1",
    "source_selfhost_claim = 0",
    "runtime_route_switch = 0",
    blocker,
]:
    need(needle in task_order, f"task-order missing {needle}")

need("tools/checks/rust_lifecycle_mirbuilder_hard_authority_pilot_boolrecipe_compare_semantic_command_guard.sh" in index, "check index missing guard")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-hard-authority-boolrecipe.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/hard_authority_boolrecipe_command.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/hard_authority_boolrecipe_command.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$COMMAND_FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

lines = [
    "using lang.compiler.mirbuilder.compare_lowering_symbolic_command_snapshot as CompareLoweringSymbolicCommandSnapshotBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for idx, row in enumerate(fixture["rows"]):
    intent_entries = ", ".join(json.dumps(k) + " => " + str(v) for k, v in row["intent"].items())
    lines.append(f"    local intent_{idx} = %{{{intent_entries}}}")
    lines.append(f"    local command_{idx} = CompareLoweringSymbolicCommandSnapshotBox.build_command_from_intent(intent_{idx})")
    lines.append(f"    print(\"command:{row['row_id']}:\" + CompareLoweringSymbolicCommandSnapshotBox.command_summary(command_{idx}))")
    summary = (
        "snapshot_kind=CompareLoweringSymbolicCommandSnapshotV1"
        + ";ok=1;symbolic_command_ready=1"
        + f";lhs_symbol_id={row['intent']['lhs_symbol_id']}"
        + (";mir_compare_op=Le;mir_compare_op_code=2" if row["intent"]["mir_compare_op_code"] == 2 else ";mir_compare_op=Lt;mir_compare_op_code=1")
        + (";bound_kind=LiteralI64;bound_kind_code=1" if row["intent"]["rhs_bound_kind_code"] == 1 else ";bound_kind=SymbolRef;bound_kind_code=2")
        + f";bound_i64={row['intent']['rhs_bound_i64']}"
        + f";bound_symbol_id={row['intent']['rhs_bound_symbol_id']}"
        + ";dst_policy=FreshCompareValueIdRequired;dst_policy_code=1"
        + ";branch_target_policy=ExternalBasicBlockTargetsRequired;branch_target_policy_code=1"
        + ";analysis_only=1;lowering_executed=0;operand_value_id_resolution=0"
        + ";rhs_runtime_materialization=0;mir_cmp_emission=0;branch_emission=0"
        + ";basic_block_mutation=0;value_id_allocation=0;route_selection=0"
        + ";runtime_route_switch=0;source_selfhost_claim=0"
    )
    expected_lines.append(f"command:{row['row_id']}:{summary}")

lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$COMMAND_IMPL" >/dev/null
if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit hard-authority BoolRecipe command executable"
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
    print("[hard-authority/boolrecipe-command] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-hard-authority-pilot-boolrecipe-compare-semantic-command-v0
token=MIRBUILDER-HARD-AUTHORITY-PILOT-BOOLRECIPE-COMPARE-SEMANTIC-COMMAND-001
candidate=BoolRecipeCompareSemanticCommandBoundary
owner=BoolRecipeCompareLoweringIntentSnapshotBox
supporting_owner=CompareLoweringSymbolicCommandSnapshotBox
hard_authority_pilot_implemented=1
boolrecipe_compare_semantic_command_owner=1
hako_semantic_command_surface=1
rust_oracle_parity=1
aot_exe_guard=1
downstream_boundary_present=1
hako_adopted_decision=0
source_selfhost_claim=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
mir_mutation=0
id_allocation=0
new_backend_route=0
new_abi=0
selected_next_card=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
summary=ok
REPORT
