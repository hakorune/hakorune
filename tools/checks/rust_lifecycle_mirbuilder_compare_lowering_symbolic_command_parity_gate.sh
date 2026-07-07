#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-lowering-symbolic-command-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-lowering-symbolic-command-parity-v0.json"
IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/compare_lowering_symbolic_command_snapshot.hako"
PILOT_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_lowering_symbolic_command_pilot_gate.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$IMPL" "$PILOT_GATE" "$HAKO_BIN"

PILOT_OUT="$(guard_cached_run "$TAG" bash "$PILOT_GATE")"
if ! grep -q '^compare_lowering_symbolic_command_pilot=1$' <<<"$PILOT_OUT"; then
  printf '%s\n' "$PILOT_OUT" >&2
  guard_fail "$TAG" "symbolic command pilot prerequisite is not green"
fi

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
need(fixture.get("kind") == "MirBuilderCompareLoweringSymbolicCommandParityV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PARITY-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PILOT-001", "bad prerequisite")
need(fixture.get("owner") == "CompareLoweringSymbolicCommandSnapshotBox", "bad owner")
need([row.get("row_id") for row in fixture.get("rows") or []] == ["intent_var_le_literal", "intent_var_lt_symbol"], "row set drift")

claims = fixture.get("claims") or {}
for key in [
    "compare_lowering_symbolic_command_parity",
    "intent_to_command_field_parity",
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
    "build_command_from_intent(intent): MapBox",
    '"lhs_symbol_id" => me._i(intent, "lhs_symbol_id")',
    '"mir_compare_op_code" => me._i(intent, "mir_compare_op_code")',
    '"rhs_bound_kind_code" => me._i(intent, "rhs_bound_kind_code")',
    '"rhs_bound_i64" => me._i(intent, "rhs_bound_i64")',
    '"rhs_bound_symbol_id" => me._i(intent, "rhs_bound_symbol_id")',
    '"lowering_executed" => 0',
    '"mir_cmp_emission" => 0',
    '"branch_emission" => 0',
    '"value_id_allocation" => 0',
]:
    need(needle in impl, f"implementation parity token missing: {needle}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-compare-symbolic-parity.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/compare_symbolic_parity.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/compare_symbolic_parity.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

lines = [
    "using selfhost.shared.common.string_helpers as StringHelpers",
    "using selfhost.shared.common.box_helpers as BoxHelpers",
    "using lang.compiler.mirbuilder.compare_lowering_symbolic_command_snapshot as CompareLoweringSymbolicCommandSnapshotBox",
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
    intent = f"intent_{idx}"
    command = f"command_{idx}"
    lhs_eq = f"lhs_eq_{idx}"
    op_eq = f"op_eq_{idx}"
    kind_eq = f"kind_eq_{idx}"
    i64_eq = f"i64_eq_{idx}"
    symbol_eq = f"symbol_eq_{idx}"
    nonmut_eq = f"nonmut_eq_{idx}"
    lines.append(f"    local {intent} = %{{{intent_entries}}}")
    lines.append(f"    local {command} = CompareLoweringSymbolicCommandSnapshotBox.build_command_from_intent({intent})")
    lines.append(f"    local {lhs_eq} = 0")
    lines.append(f"    if StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"lhs_symbol_id\")) == StringHelpers.to_i64(BoxHelpers.map_get({command}, \"lhs_symbol_id\")) {{ {lhs_eq} = 1 }}")
    lines.append(f"    local {op_eq} = 0")
    lines.append(f"    if StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"mir_compare_op_code\")) == StringHelpers.to_i64(BoxHelpers.map_get({command}, \"mir_compare_op_code\")) {{ {op_eq} = 1 }}")
    lines.append(f"    local {kind_eq} = 0")
    lines.append(f"    if StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"rhs_bound_kind_code\")) == StringHelpers.to_i64(BoxHelpers.map_get({command}, \"rhs_bound_kind_code\")) {{ {kind_eq} = 1 }}")
    lines.append(f"    local {i64_eq} = 0")
    lines.append(f"    if StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"rhs_bound_i64\")) == StringHelpers.to_i64(BoxHelpers.map_get({command}, \"rhs_bound_i64\")) {{ {i64_eq} = 1 }}")
    lines.append(f"    local {symbol_eq} = 0")
    lines.append(f"    if StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"rhs_bound_symbol_id\")) == StringHelpers.to_i64(BoxHelpers.map_get({command}, \"rhs_bound_symbol_id\")) {{ {symbol_eq} = 1 }}")
    lines.append(f"    local {nonmut_eq} = 0")
    lines.append(f"    if StringHelpers.to_i64(BoxHelpers.map_get({command}, \"mir_cmp_emission\")) == 0 {{")
    lines.append(f"      if StringHelpers.to_i64(BoxHelpers.map_get({command}, \"branch_emission\")) == 0 {{")
    lines.append(f"        if StringHelpers.to_i64(BoxHelpers.map_get({command}, \"value_id_allocation\")) == 0 {{")
    lines.append(f"          if StringHelpers.to_i64(BoxHelpers.map_get({command}, \"basic_block_mutation\")) == 0 {{ {nonmut_eq} = 1 }}")
    lines.append("        }")
    lines.append("      }")
    lines.append("    }")
    lines.append(
        f"    print(\"parity:{row['row_id']}:\""
        f" + \";lhs_symbol_equal=\" + StringHelpers.int_to_str({lhs_eq})"
        f" + \";compare_op_equal=\" + StringHelpers.int_to_str({op_eq})"
        f" + \";rhs_bound_kind_equal=\" + StringHelpers.int_to_str({kind_eq})"
        f" + \";rhs_bound_i64_equal=\" + StringHelpers.int_to_str({i64_eq})"
        f" + \";rhs_bound_symbol_equal=\" + StringHelpers.int_to_str({symbol_eq})"
        f" + \";non_mutating_claims_preserved=\" + StringHelpers.int_to_str({nonmut_eq}))"
    )
    expected_parity = row["expected_parity"]
    expected_lines.append(
        f"parity:{row['row_id']}:"
        f";lhs_symbol_equal={expected_parity['lhs_symbol_equal']}"
        f";compare_op_equal={expected_parity['compare_op_equal']}"
        f";rhs_bound_kind_equal={expected_parity['rhs_bound_kind_equal']}"
        f";rhs_bound_i64_equal={expected_parity['rhs_bound_i64_equal']}"
        f";rhs_bound_symbol_equal={expected_parity['rhs_bound_symbol_equal']}"
        f";non_mutating_claims_preserved={expected_parity['non_mutating_claims_preserved']}"
    )

lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit Compare lowering symbolic parity executable"
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
    print("[compare-lowering/symbolic-command-parity] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-lowering-symbolic-command-parity-gate-v0
token=MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PARITY-001
owner=CompareLoweringSymbolicCommandSnapshotBox
parity_rows=2
compare_lowering_symbolic_command_parity=1
intent_to_command_field_parity=1
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
selected_next_card=MIRBUILDER-COMPARE-RHS-MATERIALIZATION-OWNER-SELECTION-001
summary=ok
REPORT
