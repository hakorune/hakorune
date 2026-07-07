#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-lowering-symbolic-command-pilot-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-lowering-symbolic-command-pilot-v0.json"
IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/compare_lowering_symbolic_command_snapshot.hako"
INTENT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/bool_recipe_compare_lowering_intent_snapshot.hako"
BOOL_RECIPE_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/bool_recipe_box.hako"
SELECTION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_lowering_mutation_owner_selection_guard.sh"
INTENT_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_lowering_observe_only_pilot_gate.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" sha256sum
guard_require_files "$TAG" "$FIXTURE" "$IMPL" "$INTENT_IMPL" "$BOOL_RECIPE_IMPL" "$SELECTION_GATE" "$INTENT_GATE" "$HAKO_BIN"

SELECTION_OUT="$(guard_cached_run "$TAG" bash "$SELECTION_GATE")"
if ! grep -q '^symbolic_compare_lowering_command_pilot_selected=1$' <<<"$SELECTION_OUT"; then
  printf '%s\n' "$SELECTION_OUT" >&2
  guard_fail "$TAG" "symbolic command selection prerequisite is not green"
fi

INTENT_OUT="$(guard_cached_run "$TAG" bash "$INTENT_GATE")"
if ! grep -q '^observe_only_lowering_intent=1$' <<<"$INTENT_OUT"; then
  printf '%s\n' "$INTENT_OUT" >&2
  guard_fail "$TAG" "BoolRecipe lowering intent prerequisite is not green"
fi

export HAKO_COMPARE_LOWERING_SYMBOLIC_COMMAND_IMPL_HASH="$(
  sha256sum "$IMPL" "$INTENT_IMPL" "$BOOL_RECIPE_IMPL" | sha256sum | awk '{ print $1 }'
)"

python3 - "$FIXTURE" "$IMPL" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
impl = Path(sys.argv[2]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderCompareLoweringSymbolicCommandPilotV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PILOT-001", "bad token")
need(fixture.get("owner") == "CompareLoweringSymbolicCommandSnapshotBox", "bad owner")
need(fixture.get("output_contract") == "CompareLoweringSymbolicCommandSnapshotV1", "bad output contract")
need([row.get("row_id") for row in fixture.get("rows") or []] == ["intent_var_le_literal", "intent_var_lt_symbol"], "row set drift")

claims = fixture.get("claims") or {}
for key in [
    "compare_lowering_symbolic_command_pilot",
    "symbolic_command_snapshot",
    "intent_map_input_boundary",
    "analysis_only",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "bool_recipe_lowering_executed",
    "operand_value_id_resolution",
    "rhs_runtime_materialization",
    "mir_cmp_emission",
    "branch_emission",
    "basic_block_mutation",
    "value_id_allocation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "programjson_var_rhs_full_dispatcher_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "build_command(program_json): MapBox",
    "BoolRecipeCompareLoweringIntentSnapshotBox.build_intent",
    "build_command_from_intent(intent): MapBox",
    "\"symbolic_command_ready\" => 1",
    "\"dst_policy_code\" => 1",
    "\"branch_target_policy_code\" => 1",
    "\"operand_value_id_resolution\" => 0",
    "\"rhs_runtime_materialization\" => 0",
    "\"mir_cmp_emission\" => 0",
    "\"branch_emission\" => 0",
    "\"value_id_allocation\" => 0",
    "command_summary(command)",
]:
    need(needle in impl, f"implementation missing token: {needle}")
for forbidden in [
    "MirInstruction",
    "emit_mir",
    "emit_compare",
    "emit_branch",
    "route_registry",
    "RecipeMatcherBox",
    "next_value_id",
]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-compare-symbolic-command.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/compare_symbolic_command.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/compare_symbolic_command.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import os
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
    "    local cache_hash = " + json.dumps(os.environ.get("HAKO_COMPARE_LOWERING_SYMBOLIC_COMMAND_IMPL_HASH", "")),
    "    if cache_hash == \"__never__\" { print(cache_hash) }",
]
expected_lines = []
for idx, row in enumerate(fixture["rows"]):
    var = f"command_{idx}"
    if row.get("input_kind") != "intent_map":
        raise SystemExit(f"unknown input_kind for {row['row_id']}")
    intent_entries = ", ".join(
        json.dumps(key) + " => " + str(value)
        for key, value in row["intent"].items()
    )
    lines.append(f"    local intent_{idx} = %{{{intent_entries}}}")
    lines.append(f"    local {var} = CompareLoweringSymbolicCommandSnapshotBox.build_command_from_intent(intent_{idx})")
    lines.append(
        f"    print(\"command:{row['row_id']}:\" + CompareLoweringSymbolicCommandSnapshotBox.command_summary({var}))"
    )
    expected_lines.append(f"command:{row['row_id']}:{row['expected_command_summary']}")

lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit Compare lowering symbolic command executable"
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
    print("[compare-lowering/symbolic-command] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-lowering-symbolic-command-pilot-gate-v0
token=MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PILOT-001
owner=CompareLoweringSymbolicCommandSnapshotBox
command_rows=2
compare_lowering_symbolic_command_pilot=1
symbolic_command_snapshot=1
intent_map_input_boundary=1
analysis_only=1
bool_recipe_lowering_executed=0
operand_value_id_resolution=0
rhs_runtime_materialization=0
mir_cmp_emission=0
branch_emission=0
basic_block_mutation=0
value_id_allocation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
programjson_var_rhs_full_dispatcher_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PARITY-001
summary=ok
REPORT
