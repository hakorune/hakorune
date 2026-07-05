#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-aot-dynamic-string-eq-and-int-to-str-correctness-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/hako-aot-dynamic-string-eq-and-int-to-str-correctness-v0.json"
SCANNER_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako"
STRING_HELPERS="$ROOT_DIR/lang/src/shared/common/string_helpers.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$SCANNER_IMPL" "$STRING_HELPERS" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-aot-string-eq-int-to-str.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
if [[ "${HAKO_KEEP_TMP:-0}" == "1" ]]; then
  echo "debug_tmp=$TMP_DIR" >&2
else
  trap cleanup EXIT
fi

APP="$TMP_DIR/aot_dynamic_string_eq_int_to_str.hako"
EXPECTED="$TMP_DIR/expected.txt"
RUN_LOG="$TMP_DIR/run.log"
EXE="$TMP_DIR/aot_dynamic_string_eq_int_to_str.exe"
MIR_JSON="$TMP_DIR/aot_dynamic_string_eq_int_to_str.mir.json"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

if fixture.get("kind") != "HakoAotDynamicStringEqAndIntToStrCorrectnessV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "HAKO-AOT-DYNAMIC-STRING-EQ-AND-INT-TO-STR-CORRECTNESS-001":
    raise SystemExit("bad fixture token")

rows = fixture.get("rows") or []
if len(rows) != 1:
    raise SystemExit("expected exactly one focused AOT correctness row")
row = rows[0]
program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
exp = row["expected"]
expected_line = (
    "aot_correctness:"
    f"type={exp['node_type']};"
    f"is_return={exp['dynamic_string_eq']};"
    f"rc_eq={exp['return_count_by_dynamic_eq']};"
    f"rc_char_int={exp['int_to_str_of_dynamic_count']};"
    f"rc_char_token={exp['count_token_of_dynamic_count']}"
)

claims = fixture.get("claims") or {}
for key in [
    "hako_syntax_change",
    "new_hako_library_api",
    "programjson_traversal_capability",
    "source_selfhost_claim",
    "mir_mutation",
    "id_allocation",
    "backend_lowering_claim",
    "new_backend_route",
    "new_abi",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

source = f'''using selfhost.shared.common.string_helpers as StringHelpers
using lang.compiler.mirbuilder.program_json_v0_scanner_box as ProgramJsonV0ScannerBox

static box AotCorrectnessProbeBox {{
  debug(program_json) {{
    local program_body = me._array_field_start(program_json, "body", 0)
    local loop_start = me._first_object_in_array(program_json, program_body)
    local loop_body = me._array_field_start(program_json, "body", loop_start)
    local stmt_start = me._first_object_in_array(program_json, loop_body)

    local t = me._node_type_at(program_json, stmt_start)
    local is_return = me._node_is_return_dynamic(program_json, stmt_start)
    local rc_eq = me._return_count_stmt_dynamic(program_json, stmt_start)
    local rc_char = me._return_count_stmt_char(program_json, stmt_start)

    return "aot_correctness:type=" + t
      + ";is_return=" + me._count_token(is_return)
      + ";rc_eq=" + me._count_token(rc_eq)
      + ";rc_char_int=" + StringHelpers.int_to_str(rc_char)
      + ";rc_char_token=" + me._count_token(rc_char)
  }}

  _return_count_stmt_dynamic(s, stmt_start) {{
    if me._node_is_return_dynamic(s, stmt_start) == 1 {{ return 1 }}
    return 0
  }}

  _return_count_stmt_char(s, stmt_start) {{
    local p = ProgramJsonV0ScannerBox.seek_obj_field_value_start(s, "type", stmt_start)
    if p < 0 {{ return 0 }}
    p = StringHelpers.skip_ws(s, p)
    if ProgramJsonV0ScannerBox._read_char(s, p + 1) == "R" {{ return 1 }}
    return 0
  }}

  _node_is_return_dynamic(s, obj_start) {{
    local node_type = me._node_type_at(s, obj_start)
    if node_type == "Return" {{ return 1 }}
    return 0
  }}

  _node_type_at(s, obj_start) {{
    return me._string_field_at(s, "type", obj_start)
  }}

  _string_field_at(s, key, obj_start) {{
    local p = ProgramJsonV0ScannerBox.seek_obj_field_value_start(s, key, obj_start)
    if p < 0 {{ return "" }}
    p = StringHelpers.skip_ws(s, p)
    if ProgramJsonV0ScannerBox._read_char(s, p) != "\\"" {{ return "" }}
    local end_quote = StringHelpers.index_of(s, p + 1, "\\"")
    if end_quote < 0 {{ return "" }}
    return s.substring(p + 1, end_quote)
  }}

  _array_field_start(s, key, obj_start) {{
    if obj_start < 0 {{ return -1 }}
    local p = ProgramJsonV0ScannerBox.seek_obj_field_value_start(s, key, obj_start)
    if p < 0 {{ return -1 }}
    p = StringHelpers.skip_ws(s, p)
    if ProgramJsonV0ScannerBox._read_char(s, p) != "[" {{ return -1 }}
    return p
  }}

  _first_object_in_array(s, array_start) {{
    if array_start < 0 {{ return -1 }}
    local p = StringHelpers.skip_ws(s, array_start + 1)
    if ProgramJsonV0ScannerBox._read_char(s, p) != "{{" {{ return -1 }}
    return p
  }}

  _count_token(n) {{
    if n <= 0 {{ return "0" }}
    if n == 1 {{ return "1" }}
    if n == 2 {{ return "2" }}
    return "many"
  }}
}}

static box Main {{
  main() {{
    print(AotCorrectnessProbeBox.debug({json.dumps(program_json)}))
    return 0
  }}
}}
'''

app.write_text(source, encoding="utf-8")
expected.write_text(expected_line + "\n", encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$SCANNER_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for AOT correctness probe"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
string_eq_return = 0
stringbox_add = 0

for fn in data.get("functions", []):
    name = fn.get("name", "")
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") == "compare" and inst.get("operation") == "==":
                rhs = inst.get("rhs")
                consts = {
                    ci.get("dst"): ci.get("value", {}).get("value")
                    for b in fn.get("blocks", [])
                    for ci in b.get("instructions", [])
                    if ci.get("op") == "const"
                }
                if consts.get(rhs) == "Return" and inst.get("cmp_kind") == "string":
                    string_eq_return += 1
            if inst.get("op") == "binop" and inst.get("operation") == "+":
                dst_type = inst.get("dst_type")
                if name == "StringHelpers.int_to_str/1":
                    if isinstance(dst_type, dict) and dst_type.get("box_type") == "StringBox":
                        stringbox_add += 1

if string_eq_return < 1:
    raise SystemExit("missing cmp_kind=string for dynamic node_type == \"Return\"")
if stringbox_add < 1:
    raise SystemExit("missing StringBox dst_type for StringHelpers.int_to_str concat")
PY

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit AOT correctness executable"
fi

if ! "$EXE" >"$RUN_LOG" 2>&1; then
  tail -n 160 "$RUN_LOG" || true
  guard_fail "$TAG" "failed to run AOT correctness executable"
fi

python3 - "$EXPECTED" "$RUN_LOG" <<'PY'
import sys
from pathlib import Path

expected = Path(sys.argv[1]).read_text(encoding="utf-8").strip()
lines = [
    line.strip()
    for line in Path(sys.argv[2]).read_text(encoding="utf-8").splitlines()
    if line.strip() and not line.startswith("Result:")
]
if lines != [expected]:
    print(f"expected: {expected}", file=sys.stderr)
    print(f"actual: {lines}", file=sys.stderr)
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=hako-aot-dynamic-string-eq-and-int-to-str-correctness-gate-v0
fixture=hako-aot-dynamic-string-eq-and-int-to-str-correctness-v0.json
execution_backend=aot
dynamic_string_equality=green
dynamic_int_to_str=green
mir_json_string_cmp_kind=green
mir_json_string_concat_dst_type=green
hako_syntax_change=0
new_hako_library_api=0
programjson_traversal_capability=0
source_selfhost_claim=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
