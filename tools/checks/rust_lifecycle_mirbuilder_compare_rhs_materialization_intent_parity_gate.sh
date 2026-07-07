#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-rhs-materialization-intent-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-materialization-intent-parity-v0.json"
IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/compare_rhs_materialization_intent_snapshot.hako"
PILOT_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_materialization_intent_pilot_gate.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$IMPL" "$PILOT_GATE" "$HAKO_BIN"

PILOT_OUT="$(guard_cached_run "$TAG" bash "$PILOT_GATE")"
if ! grep -q '^compare_rhs_materialization_intent_pilot=1$' <<<"$PILOT_OUT"; then
  printf '%s\n' "$PILOT_OUT" >&2
  guard_fail "$TAG" "RHS materialization intent pilot prerequisite is not green"
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
need(fixture.get("kind") == "MirBuilderCompareRhsMaterializationIntentParityV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PARITY-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PILOT-001", "bad prerequisite")
need(fixture.get("owner") == "CompareRhsMaterializationIntentSnapshotBox", "bad owner")
need([row.get("row_id") for row in fixture.get("rows") or []] == ["command_literal_i64", "command_symbol_ref"], "row set drift")

claims = fixture.get("claims") or {}
for key in [
    "compare_rhs_materialization_intent_parity",
    "command_to_intent_field_parity",
    "rhs_materialization_intent_snapshot",
    "analysis_only",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
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
    "programjson_var_rhs_full_dispatcher_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

for needle in [
    "build_intent_from_command(command): MapBox",
    '"rhs_bound_kind_code" => me._i(command, "rhs_bound_kind_code")',
    '"rhs_bound_i64" => me._i(command, "rhs_bound_i64")',
    '"rhs_bound_symbol_id" => me._i(command, "rhs_bound_symbol_id")',
    '"rhs_materialization_intent_ready" => 1',
    '"rhs_value_id_resolution" => 0',
    '"rhs_runtime_materialization" => 0',
    '"constant_mir_emission" => 0',
    '"runtime_helper_emission" => 0',
    '"mir_cmp_emission" => 0',
    '"branch_emission" => 0',
    '"value_id_allocation" => 0',
]:
    need(needle in impl, f"implementation parity token missing: {needle}")
PY

TMP_DIR="$(mktemp -d /tmp/hakorune-rhs-materialization-intent-parity.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/rhs_materialization_intent_parity.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/rhs_materialization_intent_parity.exe"
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
    "using lang.compiler.mirbuilder.compare_rhs_materialization_intent_snapshot as CompareRhsMaterializationIntentSnapshotBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for idx, row in enumerate(fixture["rows"]):
    command_entries = ", ".join(
        json.dumps(key) + " => " + str(value)
        for key, value in row["command"].items()
    )
    command = f"command_{idx}"
    intent = f"intent_{idx}"
    kind_eq = f"kind_eq_{idx}"
    i64_eq = f"i64_eq_{idx}"
    symbol_eq = f"symbol_eq_{idx}"
    literal_value = f"literal_value_{idx}"
    symbol_lookup_value = f"symbol_lookup_value_{idx}"
    nonmat_eq = f"nonmat_eq_{idx}"
    lines.append(f"    local {command} = %{{{command_entries}}}")
    lines.append(f"    local {intent} = CompareRhsMaterializationIntentSnapshotBox.build_intent_from_command({command})")
    lines.append(f"    local {kind_eq} = 0")
    lines.append(f"    if StringHelpers.to_i64(BoxHelpers.map_get({command}, \"rhs_bound_kind_code\")) == StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"rhs_bound_kind_code\")) {{ {kind_eq} = 1 }}")
    lines.append(f"    local {i64_eq} = 0")
    lines.append(f"    if StringHelpers.to_i64(BoxHelpers.map_get({command}, \"rhs_bound_i64\")) == StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"rhs_bound_i64\")) {{ {i64_eq} = 1 }}")
    lines.append(f"    local {symbol_eq} = 0")
    lines.append(f"    if StringHelpers.to_i64(BoxHelpers.map_get({command}, \"rhs_bound_symbol_id\")) == StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"rhs_bound_symbol_id\")) {{ {symbol_eq} = 1 }}")
    lines.append(f"    local {literal_value} = StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"literal_i64_required\"))")
    lines.append(f"    local {symbol_lookup_value} = StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"symbol_lookup_required\"))")
    lines.append(f"    local {nonmat_eq} = 0")
    lines.append(f"    if StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"rhs_value_id_resolution\")) == 0 {{")
    lines.append(f"      if StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"rhs_runtime_materialization\")) == 0 {{")
    lines.append(f"        if StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"constant_mir_emission\")) == 0 {{")
    lines.append(f"          if StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"runtime_helper_emission\")) == 0 {{")
    lines.append(f"            if StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"mir_cmp_emission\")) == 0 {{")
    lines.append(f"              if StringHelpers.to_i64(BoxHelpers.map_get({intent}, \"value_id_allocation\")) == 0 {{ {nonmat_eq} = 1 }}")
    lines.append("            }")
    lines.append("          }")
    lines.append("        }")
    lines.append("      }")
    lines.append("    }")
    lines.append(
        f"    print(\"parity:{row['row_id']}:\""
        f" + \";rhs_bound_kind_equal=\" + StringHelpers.int_to_str({kind_eq})"
        f" + \";rhs_bound_i64_equal=\" + StringHelpers.int_to_str({i64_eq})"
        f" + \";rhs_bound_symbol_equal=\" + StringHelpers.int_to_str({symbol_eq})"
        f" + \";literal_i64_intent=\" + StringHelpers.int_to_str({literal_value})"
        f" + \";symbol_lookup_intent=\" + StringHelpers.int_to_str({symbol_lookup_value})"
        f" + \";non_materializing_claims_preserved=\" + StringHelpers.int_to_str({nonmat_eq}))"
    )
    expected_parity = row["expected_parity"]
    expected_lines.append(
        f"parity:{row['row_id']}:"
        f";rhs_bound_kind_equal={expected_parity['rhs_bound_kind_equal']}"
        f";rhs_bound_i64_equal={expected_parity['rhs_bound_i64_equal']}"
        f";rhs_bound_symbol_equal={expected_parity['rhs_bound_symbol_equal']}"
        f";literal_i64_intent={expected_parity['literal_i64_intent']}"
        f";symbol_lookup_intent={expected_parity['symbol_lookup_intent']}"
        f";non_materializing_claims_preserved={expected_parity['non_materializing_claims_preserved']}"
    )

lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit RHS materialization intent parity executable"
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
    print("[compare-rhs/materialization-intent-parity] mismatch")
    for idx in range(max(len(expected), len(actual))):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp!r} actual={got!r}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-rhs-materialization-intent-parity-gate-v0
token=MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PARITY-001
owner=CompareRhsMaterializationIntentSnapshotBox
parity_rows=2
compare_rhs_materialization_intent_parity=1
command_to_intent_field_parity=1
rhs_materialization_intent_snapshot=1
analysis_only=1
rhs_value_id_resolution=0
rhs_runtime_materialization=0
constant_mir_emission=0
runtime_helper_emission=0
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
selected_next_card=MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-OWNER-SELECTION-001
summary=ok
REPORT
