#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-materialization-intent"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/3329-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-MATERIALIZATION-INTENT-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-hard-authority-pilot-compare-rhs-materialization-intent-v0.json"
SELECTION_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-post-hard-authority-pilot-next-seam-selection-v0.json"
RHS_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-materialization-intent-parity-v0.json"
RHS_IMPL="lang/src/compiler/mirbuilder/compare_rhs_materialization_intent_snapshot.hako"
SELECTION_GUARD="tools/checks/rust_lifecycle_mirbuilder_post_hard_authority_pilot_next_seam_selection_guard.sh"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
HAKO_BIN="tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$FIXTURE" "$SELECTION_FIXTURE" \
  "$RHS_FIXTURE" "$RHS_IMPL" "$SELECTION_GUARD" "$STATE" "$TASK_ORDER" \
  "$INDEX" "$HAKO_BIN"

SELECTION_OUT="$(guard_cached_run "$TAG" bash "$SELECTION_GUARD")"
if ! grep -q '^post_hard_authority_pilot_next_seam_selected=1$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "post-hard-authority seam selection prerequisite is not green"
fi

python3 - "$CARD" "$FIXTURE" "$SELECTION_FIXTURE" "$RHS_FIXTURE" "$RHS_IMPL" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
selection_path = Path(sys.argv[3])
rhs_fixture_path = Path(sys.argv[4])
rhs_impl_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
index_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
selection = json.loads(selection_path.read_text(encoding="utf-8"))
rhs = json.loads(rhs_fixture_path.read_text(encoding="utf-8"))
rhs_impl = rhs_impl_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")


def need(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-MATERIALIZATION-INTENT-001"
output_contract = "rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-materialization-intent-v0"
blocker = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
candidate = "CompareRhsMaterializationIntentBoundary"
follow_on_card = "MIRBUILDER-POST-RHS-MATERIALIZATION-INTENT-NEXT-SEAM-SELECTION-001"
follow_on_card_path = "docs/development/current/main/phases/phase-296x/3330-MIRBUILDER-POST-RHS-MATERIALIZATION-INTENT-NEXT-SEAM-SELECTION-001.md"
second_follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-RESOLUTION-PLAN-001"
second_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3331-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-RESOLUTION-PLAN-001.md"
third_follow_on_card = "MIRBUILDER-POST-RHS-VALUEID-PLAN-NEXT-SEAM-SELECTION-001"
third_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3332-MIRBUILDER-POST-RHS-VALUEID-PLAN-NEXT-SEAM-SELECTION-001.md"
fourth_follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-REQUEST-ABI-001"
fourth_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3333-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-REQUEST-ABI-001.md"
fifth_follow_on_card = "MIRBUILDER-POST-RHS-VALUEID-REQUEST-ABI-NEXT-SEAM-SELECTION-001"
fifth_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3334-MIRBUILDER-POST-RHS-VALUEID-REQUEST-ABI-NEXT-SEAM-SELECTION-001.md"
sixth_follow_on_card = "MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001"
sixth_follow_on_card_path = "docs/development/current/main/phases/phase-296x/3335-MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001.md"

need(f"# 3329 - {token}" in card, "card token drift")
need(output_contract in card, "card output contract drift")
need(candidate in card, "card candidate drift")

need(fixture.get("kind") == "MirBuilderHardAuthorityPilotCompareRhsMaterializationIntentV1", "fixture kind drift")
need(fixture.get("token") == token, "fixture token drift")
need(fixture.get("output_contract") == output_contract, "fixture output contract drift")
need((fixture.get("current_state") or {}).get("latest_card") == token, "fixture latest card drift")
need((fixture.get("current_state") or {}).get("current_blocker_token") == blocker, "fixture blocker drift")

need((selection.get("selected_next_seam") or {}).get("candidate_id") == candidate, "selection candidate drift")
need((selection.get("claims") or {}).get("post_hard_authority_pilot_next_seam_selected") == 1, "selection claim drift")
need((selection.get("claims") or {}).get("next_seam_implemented") == 0, "selection must not implement seam")

pilot = fixture.get("pilot") or {}
need(pilot.get("candidate_id") == candidate, "pilot candidate drift")
need(pilot.get("owner_id") == "CompareRhsMaterializationIntentSnapshotBox", "pilot owner drift")
need(pilot.get("input_surface") == "CompareLoweringSymbolicCommandSnapshotV1", "input surface drift")
need(pilot.get("output_surface") == "CompareRhsMaterializationIntentSnapshotV1", "output surface drift")
need(pilot.get("downstream_consumer") == "CompareRhsValueIdResolutionPlanSnapshotBox", "consumer drift")

need(rhs.get("owner") == "CompareRhsMaterializationIntentSnapshotBox", "rhs owner drift")
need(rhs.get("output_contract") == "CompareRhsMaterializationIntentSnapshotV1", "rhs output drift")
need([row.get("row_id") for row in rhs.get("rows") or []] == ["command_literal_i64", "command_symbol_ref"], "rhs row drift")
need((rhs.get("claims") or {}).get("compare_rhs_materialization_intent_parity") == 1, "rhs parity claim drift")
for key in [
    "rhs_value_id_resolution",
    "rhs_runtime_materialization",
    "constant_mir_emission",
    "runtime_helper_emission",
    "mir_cmp_emission",
    "branch_emission",
    "basic_block_mutation",
    "value_id_allocation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need((rhs.get("claims") or {}).get(key) == 0, f"rhs forbidden claim drift: {key}")

need(len(rhs_impl.splitlines()) < 800, "rhs source exceeds 800-line source limit")
for needle in [
    "build_intent_from_command(command): MapBox",
    '"rhs_materialization_intent_ready" => 1',
    '"literal_i64_required" => literal_required',
    '"symbol_lookup_required" => symbol_required',
    '"rhs_value_id_resolution" => 0',
    '"rhs_runtime_materialization" => 0',
    '"constant_mir_emission" => 0',
    '"value_id_allocation" => 0',
]:
    need(needle in rhs_impl, f"rhs implementation missing token: {needle}")

claims = fixture.get("claims") or {}
for key in [
    "hard_authority_pilot_implemented",
    "compare_rhs_materialization_intent_owner",
    "hako_semantic_intent_surface",
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
    "rhs_value_id_resolution",
    "rhs_runtime_materialization",
    "constant_mir_emission",
    "runtime_helper_emission",
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

need(state.get("latest_card") in [token, follow_on_card, second_follow_on_card, third_follow_on_card, fourth_follow_on_card, fifth_follow_on_card, sixth_follow_on_card], "CURRENT_STATE latest card drift")
need(state.get("latest_card_path") in [str(card_path), follow_on_card_path, second_follow_on_card_path, third_follow_on_card_path, fourth_follow_on_card_path, fifth_follow_on_card_path, sixth_follow_on_card_path], "CURRENT_STATE latest path drift")
need(state.get("current_blocker_token") == blocker, "CURRENT_STATE blocker drift")

for needle in [
    token,
    output_contract,
    candidate,
    "compare_rhs_materialization_intent_owner = 1",
    "hard_authority_pilot_implemented = 1",
    "source_selfhost_claim = 0",
    blocker,
]:
    need(needle in task_order, f"task-order missing {needle}")

need("tools/checks/rust_lifecycle_mirbuilder_hard_authority_pilot_compare_rhs_materialization_intent_guard.sh" in index, "check index missing guard")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-hard-authority-rhs-intent.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/hard_authority_rhs_intent.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/hard_authority_rhs_intent.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$RHS_FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

lines = [
    "using lang.compiler.mirbuilder.compare_rhs_materialization_intent_snapshot as CompareRhsMaterializationIntentSnapshotBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for idx, row in enumerate(fixture["rows"]):
    entries = ", ".join(json.dumps(k) + " => " + str(v) for k, v in row["command"].items())
    lines.append(f"    local command_{idx} = %{{{entries}}}")
    lines.append(f"    local intent_{idx} = CompareRhsMaterializationIntentSnapshotBox.build_intent_from_command(command_{idx})")
    lines.append(f"    print(\"intent:{row['row_id']}:\" + CompareRhsMaterializationIntentSnapshotBox.intent_summary(intent_{idx}))")
    if row["command"]["rhs_bound_kind_code"] == 1:
        summary = (
            "snapshot_kind=CompareRhsMaterializationIntentSnapshotV1;ok=1"
            + ";rhs_materialization_intent_ready=1;bound_kind=LiteralI64;bound_kind_code=1"
            + f";bound_i64={row['command']['rhs_bound_i64']};bound_symbol_id=0"
            + ";rhs_materialization_kind=LiteralI64Intent;rhs_materialization_kind_code=1"
            + ";literal_i64_required=1;symbol_lookup_required=0"
        )
    else:
        summary = (
            "snapshot_kind=CompareRhsMaterializationIntentSnapshotV1;ok=1"
            + ";rhs_materialization_intent_ready=1;bound_kind=SymbolRef;bound_kind_code=2"
            + f";bound_i64=0;bound_symbol_id={row['command']['rhs_bound_symbol_id']}"
            + ";rhs_materialization_kind=SymbolLookupIntent;rhs_materialization_kind_code=2"
            + ";literal_i64_required=0;symbol_lookup_required=1"
        )
    summary += (
        ";analysis_only=1;rhs_value_id_resolution=0;rhs_runtime_materialization=0"
        + ";constant_mir_emission=0;runtime_helper_emission=0;mir_cmp_emission=0"
        + ";branch_emission=0;basic_block_mutation=0;value_id_allocation=0"
        + ";route_selection=0;runtime_route_switch=0;source_selfhost_claim=0"
    )
    expected_lines.append(f"intent:{row['row_id']}:{summary}")

lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$RHS_IMPL" >/dev/null
if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit RHS materialization intent executable"
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
    print("[hard-authority/rhs-materialization-intent] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-materialization-intent-v0
token=MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-MATERIALIZATION-INTENT-001
candidate=CompareRhsMaterializationIntentBoundary
owner=CompareRhsMaterializationIntentSnapshotBox
hard_authority_pilot_implemented=1
compare_rhs_materialization_intent_owner=1
hako_semantic_intent_surface=1
rust_oracle_parity=1
aot_exe_guard=1
downstream_boundary_present=1
hako_adopted_decision=0
source_selfhost_claim=0
rhs_value_id_resolution=0
rhs_runtime_materialization=0
constant_mir_emission=0
runtime_helper_emission=0
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
