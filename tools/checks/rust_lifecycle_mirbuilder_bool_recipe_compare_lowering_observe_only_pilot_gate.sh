#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-bool-recipe-compare-lowering-observe-only-pilot-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bool-recipe-compare-lowering-observe-only-pilot-v0.json"
EXPANDED_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-parity-v0.json"
IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/bool_recipe_compare_lowering_intent_snapshot.hako"
PUBLICATION_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_bool_recipe_compare_publication.hako"
BOOL_RECIPE_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/bool_recipe_box.hako"
CONSULTATION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_lowering_boundary_consultation_guard.sh"
PUBLICATION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_publication_parity_gate.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" sha256sum
guard_require_files "$TAG" "$FIXTURE" "$EXPANDED_FIXTURE" "$IMPL" "$PUBLICATION_IMPL" "$BOOL_RECIPE_IMPL" "$CONSULTATION_GATE" "$PUBLICATION_GATE" "$HAKO_BIN"

CONSULT_OUT="$(guard_cached_run "$TAG" bash "$CONSULTATION_GATE")"
if ! grep -q '^observe_only_lowering_intent_next=1$' <<<"$CONSULT_OUT"; then
  printf '%s\n' "$CONSULT_OUT" >&2
  guard_fail "$TAG" "BoolRecipe lowering boundary consultation prerequisite is not green"
fi

PUB_OUT="$(guard_cached_run "$TAG" bash "$PUBLICATION_GATE")"
if ! grep -q '^bool_recipe_compare_publication_parity=1$' <<<"$PUB_OUT"; then
  printf '%s\n' "$PUB_OUT" >&2
  guard_fail "$TAG" "BoolRecipe publication prerequisite is not green"
fi

export HAKO_BOOL_RECIPE_COMPARE_LOWERING_INTENT_IMPL_HASH="$(
  sha256sum "$IMPL" "$PUBLICATION_IMPL" "$BOOL_RECIPE_IMPL" | sha256sum | awk '{ print $1 }'
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
need(fixture.get("kind") == "MirBuilderBoolRecipeCompareLoweringObserveOnlyPilotV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-OBSERVE-ONLY-PILOT-001", "bad token")
need(fixture.get("owner") == "BoolRecipeCompareLoweringIntentSnapshotBox", "bad owner")
need(fixture.get("output_contract") == "BoolRecipeCompareLoweringIntentSnapshotV1", "bad output contract")
need([row.get("row_id") for row in fixture.get("rows") or []] == ["var_le_literal"], "row set drift")

claims = fixture.get("claims") or {}
for key in [
    "bool_recipe_compare_lowering_intent_snapshot",
    "observe_only_lowering_intent",
    "analysis_only",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "bool_recipe_lowering_executed",
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
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "build_intent(program_json): MapBox",
    "ProgramJsonBoolRecipeComparePublicationBox.build_publication",
    "build_intent_from_recipe(recipe): MapBox",
    "\"lowering_intent_ready\" => 1",
    "\"lowering_executed\" => 0",
    "\"mir_cmp_emission\" => 0",
    "\"branch_emission\" => 0",
    "\"value_id_allocation\" => 0",
    "intent_summary(intent)",
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

TMP_DIR="$(mktemp -d /tmp/hakorune-bool-recipe-lowering-intent.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/bool_recipe_compare_lowering_intent.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/bool_recipe_compare_lowering_intent.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$EXPANDED_FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import os
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
expanded = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
app = Path(sys.argv[3])
expected = Path(sys.argv[4])

expanded_by_id = {row["row_id"]: row["program_json"] for row in expanded.get("rows") or []}

lines = [
    "using lang.compiler.mirbuilder.bool_recipe_compare_lowering_intent_snapshot as BoolRecipeCompareLoweringIntentSnapshotBox",
    "",
    "static box Main {",
    "  main() {",
    "    local cache_hash = " + json.dumps(os.environ.get("HAKO_BOOL_RECIPE_COMPARE_LOWERING_INTENT_IMPL_HASH", "")),
    "    if cache_hash == \"__never__\" { print(cache_hash) }",
]
expected_lines = []
for idx, row in enumerate(fixture["rows"]):
    program = json.loads(json.dumps(expanded_by_id[row["source_program_row"]]))
    program["body"][1]["cond"] = row["loop_condition_patch"]
    program_json = json.dumps(program, separators=(",", ":"))
    var = f"intent_{idx}"
    lines.append(
        f"    local {var} = BoolRecipeCompareLoweringIntentSnapshotBox.build_intent({json.dumps(program_json)})"
    )
    lines.append(
        f"    print(\"intent:{row['row_id']}:\" + BoolRecipeCompareLoweringIntentSnapshotBox.intent_summary({var}))"
    )
    expected_lines.append(f"intent:{row['row_id']}:{row['expected_intent_summary']}")

lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit BoolRecipe lowering intent executable"
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
    print("[bool-recipe-compare/lowering-intent] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-bool-recipe-compare-lowering-observe-only-pilot-gate-v0
token=MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-OBSERVE-ONLY-PILOT-001
owner=BoolRecipeCompareLoweringIntentSnapshotBox
intent_rows=1
bool_recipe_compare_lowering_intent_snapshot=1
observe_only_lowering_intent=1
analysis_only=1
bool_recipe_lowering_executed=0
mir_cmp_emission=0
branch_emission=0
basic_block_mutation=0
value_id_allocation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-OPERATOR-EXPANSION-SELECTION-001
summary=ok
REPORT
