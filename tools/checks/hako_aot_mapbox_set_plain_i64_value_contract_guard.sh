#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-aot-mapbox-set-plain-i64-value-contract-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-aot-mapbox-set-i64.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
if [[ "${HAKO_KEEP_TMP:-0}" == "1" ]]; then
  echo "debug_tmp=$TMP_DIR" >&2
else
  trap cleanup EXIT
fi

APP="$TMP_DIR/mapbox_set_plain_i64_contract.hako"
EXPECTED="$TMP_DIR/expected.txt"
RUN_LOG="$TMP_DIR/run.log"
EXE="$TMP_DIR/mapbox_set_plain_i64_contract.exe"
MIR_JSON="$TMP_DIR/mapbox_set_plain_i64_contract.mir.json"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$APP" "$EXPECTED" <<'PY'
import sys
from pathlib import Path

app = Path(sys.argv[1])
expected = Path(sys.argv[2])

app.write_text(
    r'''using selfhost.shared.common.string_helpers as StringHelpers

static box SameModuleMaker {
  make(a, b): MapBox {
    return %{
      "err" => 1,
      "err_line" => "hello",
      "next_idx" => 97,
      "arg_a" => a,
      "arg_b" => b
    }
  }
}

static box Main {
  main() {
    local literal = %{
      "err" => 1,
      "err_line" => "hello",
      "next_idx" => 97,
      "arg_a" => "AAA",
      "arg_b" => "BBB"
    }
    local helper = SameModuleMaker.make("AAA", "BBB")

    print("literal:"
      + "err=" + StringHelpers.int_to_str(literal.get("err"))
      + ";err_line=" + literal.get("err_line")
      + ";next_idx=" + StringHelpers.int_to_str(literal.get("next_idx"))
      + ";arg_a=" + literal.get("arg_a")
      + ";arg_b=" + literal.get("arg_b"))

    print("helper:"
      + "err=" + StringHelpers.int_to_str(helper.get("err"))
      + ";err_line=" + helper.get("err_line")
      + ";next_idx=" + StringHelpers.int_to_str(helper.get("next_idx"))
      + ";arg_a=" + helper.get("arg_a")
      + ";arg_b=" + helper.get("arg_b"))

    return 0
  }
}
''',
    encoding="utf-8",
)

expected.write_text(
    "\n".join(
        [
            "literal:err=1;err_line=hello;next_idx=97;arg_a=AAA;arg_b=BBB",
            "helper:err=1;err_line=hello;next_idx=97;arg_a=AAA;arg_b=BBB",
        ]
    )
    + "\n",
    encoding="utf-8",
)
PY

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for MapBox.set plain-i64 contract"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
set_routes = []
for fn in data.get("functions", []):
    for route in (fn.get("metadata") or {}).get("generic_method_routes", []):
        if route.get("method") == "set" and route.get("route_kind", "").startswith("map_store_"):
            set_routes.append(route)

if len(set_routes) < 2:
    raise SystemExit("expected MapBox.set route metadata for literal and helper rows")
if any(route.get("receiver_box") == "ArrayBox" for route in set_routes):
    raise SystemExit("unexpected array set route in MapBox.set plain-i64 guard")
PY

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 160 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit AOT executable for MapBox.set plain-i64 contract"
fi

if ! "$EXE" >"$RUN_LOG" 2>&1; then
  tail -n 160 "$RUN_LOG" || true
  guard_fail "$TAG" "failed to run AOT executable for MapBox.set plain-i64 contract"
fi

python3 - "$EXPECTED" "$RUN_LOG" <<'PY'
import sys
from pathlib import Path

expected = Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()
actual = [
    line.strip()
    for line in Path(sys.argv[2]).read_text(encoding="utf-8").splitlines()
    if line.strip()
    and not line.startswith("Result:")
    and not line.startswith("[freeze:contract]")
]
if actual != expected:
    print("expected:", expected, file=sys.stderr)
    print("actual:", actual, file=sys.stderr)
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=hako-aot-mapbox-set-plain-i64-value-contract-guard-v0
mapbox_set_plain_i64_value=boxed_before_any_store
same_function_literal_row=green
same_module_mapbox_return_row=green
generic_void_object_return_widening=0
scanner_nullable_bridge=0
mixed_runtime_i64_or_handle_for_scanner_out_map=0
source_selfhost_claim=0
summary=ok
REPORT
