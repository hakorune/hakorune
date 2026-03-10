#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
BIN="${BIN:-target/release/hakorune}"
ENTRY="${1:-apps/tests/hello_simple_llvm.hako}"

if [[ ! -x "$BIN" ]]; then
  echo "[FAIL] missing bin: $BIN" >&2
  exit 2
fi
if [[ ! -f "$ENTRY" ]]; then
  echo "[FAIL] missing entry: $ENTRY" >&2
  exit 2
fi

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

helper_src="$tmp_dir/phase29ch_source_route_probe.hako"
helper_bin="$tmp_dir/phase29ch_source_route_probe"
helper_out="$tmp_dir/probe.out"
helper_json="$tmp_dir/probe.json"

extract_json() {
  local src="$1"
  local out="$2"
  python3 - "$src" "$out" <<'PY'
import json
import pathlib
import sys

text = pathlib.Path(sys.argv[1]).read_text(errors="ignore")
start = text.find("{")
if start < 0:
    raise SystemExit("missing JSON object")
data = json.loads(text[start:])
pathlib.Path(sys.argv[2]).write_text(json.dumps(data, ensure_ascii=False))
PY
}

source_text="$(cat "$ENTRY")"
python3 - "$helper_src" "$source_text" <<'PY'
import json
import pathlib
import sys

helper_path = pathlib.Path(sys.argv[1])
source_text = sys.argv[2]
source_literal = json.dumps(source_text, ensure_ascii=False)
helper_path.write_text(
    "using lang.mir.builder.MirBuilderBox as MirBuilderBox\n\n"
    "static box Main {\n"
    "  main() {\n"
    f"    local source_text = {source_literal}\n"
    "    local mir = MirBuilderBox.emit_from_source_v0(source_text, null)\n"
    "    print(\"\" + mir)\n"
    "    return 0\n"
    "  }\n"
    "}\n",
    encoding="utf-8",
)
PY

NYASH_BIN="$BIN" bash "$ROOT/tools/selfhost_exe_stageb.sh" "$helper_src" -o "$helper_bin" >/dev/null

NYASH_NYRT_SILENT_RESULT="${NYASH_NYRT_SILENT_RESULT:-1}" \
NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
NYASH_FILEBOX_MODE="${NYASH_FILEBOX_MODE:-core-ro}" \
"$helper_bin" >"$helper_out"

extract_json "$helper_out" "$helper_json"

echo "[source-route-direct] bin=$BIN"
echo "[source-route-direct] entry=$ENTRY"
echo "[source-route-direct] helper-build=compiled-artifact direct probe"
echo "[source-route-direct] note=diagnostics-only (helper artifact, not reduced-case authority evidence)"
python3 - "$helper_json" <<'PY'
import json
import pathlib
import sys

payload = json.loads(pathlib.Path(sys.argv[1]).read_text())
funcs = payload.get("functions")
user_boxes = payload.get("user_box_decls")
func_count = len(funcs) if isinstance(funcs, list) else -1
box_count = len(user_boxes) if isinstance(user_boxes, list) else -1
print(f"[source-route-direct] functions={func_count} user_box_decls={box_count}")
PY
