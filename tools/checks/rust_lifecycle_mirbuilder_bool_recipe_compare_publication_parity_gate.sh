#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-bool-recipe-compare-publication-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bool-recipe-compare-publication-parity-v0.json"
EXPANDED_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-parity-v0.json"
PUBLICATION_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_bool_recipe_compare_publication.hako"
FACTS_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_canonical_loop_facts_input_snapshot.hako"
BOOL_RECIPE_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/recipe/bool_recipe_box.hako"
CONSUME_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_canonical_loop_facts_numeric_compare_canon_consume_gate.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" sha256sum
guard_require_files "$TAG" "$FIXTURE" "$EXPANDED_FIXTURE" "$PUBLICATION_IMPL" "$FACTS_IMPL" "$BOOL_RECIPE_IMPL" "$CONSUME_GATE" "$HAKO_BIN"

CONSUME_OUT="$(guard_cached_run "$TAG" bash "$CONSUME_GATE")"
if ! grep -q '^canonical_loop_facts_numeric_compare_canon_consume=1$' <<<"$CONSUME_OUT"; then
  printf '%s\n' "$CONSUME_OUT" >&2
  guard_fail "$TAG" "CanonicalLoopFacts numeric compare consume prerequisite is not green"
fi

export HAKO_BOOL_RECIPE_COMPARE_PUBLICATION_IMPL_HASH="$(
  sha256sum "$PUBLICATION_IMPL" "$FACTS_IMPL" "$BOOL_RECIPE_IMPL" | sha256sum | awk '{ print $1 }'
)"

python3 - "$FIXTURE" "$PUBLICATION_IMPL" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
impl = Path(sys.argv[2]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderBoolRecipeComparePublicationParityV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-PARITY-001", "bad token")
need(fixture.get("owner") == "ProgramJsonBoolRecipeComparePublicationBox", "bad owner")
need(fixture.get("output_contract") == "BoolRecipeComparePublicationV1", "bad output contract")
rows = fixture.get("rows") or []
need([row.get("row_id") for row in rows] == ["var_le_literal"], "row set drift")

claims = fixture.get("claims") or {}
for key in [
    "bool_recipe_compare_publication_parity",
    "read_only_bool_recipe_compare_publication",
    "analysis_only",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "recipe_item_attachment",
    "recipe_matcher_input_authority",
    "bool_recipe_lowering",
    "mir_cmp_emission",
    "branch_emission",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "build_publication(program_json): MapBox",
    "ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot",
    "BoolRecipeBox.from_numeric_compare_code_map",
    "publication_summary(publication)",
    "\"recipe_matcher_executed\" => 0",
    "\"route_selection\" => 0",
    "\"runtime_route_switch\" => 0",
    "\"source_selfhost_claim\" => 0",
]:
    need(needle in impl, f"implementation missing token: {needle}")
for forbidden in [
    "RecipeMatcherBox",
    "emit_mir",
    "route_registry",
    "ASTNode",
]:
    need(forbidden not in impl, f"forbidden implementation token: {forbidden}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-bool-recipe-publication.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/bool_recipe_compare_publication.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/bool_recipe_compare_publication.exe"
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
    "using lang.compiler.mirbuilder.program_json_bool_recipe_compare_publication as ProgramJsonBoolRecipeComparePublicationBox",
    "",
    "static box Main {",
    "  main() {",
    "    local cache_hash = " + json.dumps(os.environ.get("HAKO_BOOL_RECIPE_COMPARE_PUBLICATION_IMPL_HASH", "")),
    "    if cache_hash == \"__never__\" { print(cache_hash) }",
]
expected_lines = []
for idx, row in enumerate(fixture["rows"]):
    program = json.loads(json.dumps(expanded_by_id[row["source_program_row"]]))
    program["body"][1]["cond"] = row["loop_condition_patch"]
    program_json = json.dumps(program, separators=(",", ":"))
    var = f"publication_{idx}"
    lines.append(
        f"    local {var} = ProgramJsonBoolRecipeComparePublicationBox.build_publication({json.dumps(program_json)})"
    )
    lines.append(
        f"    print(\"publication:{row['row_id']}:\" + ProgramJsonBoolRecipeComparePublicationBox.publication_summary({var}))"
    )
    expected_lines.append(f"publication:{row['row_id']}:{row['expected_publication_summary']}")

lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$PUBLICATION_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit BoolRecipe publication executable"
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
    print("[bool-recipe-compare/publication] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-bool-recipe-compare-publication-parity-gate-v0
token=MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-PARITY-001
owner=ProgramJsonBoolRecipeComparePublicationBox
publication_rows=1
bool_recipe_compare_publication_parity=1
read_only_bool_recipe_compare_publication=1
canonical_loop_facts_numeric_compare_consume_required=1
analysis_only=1
recipe_item_attachment=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-001
summary=ok
REPORT
