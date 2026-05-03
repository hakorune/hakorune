#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
source "$ROOT/tools/selfhost/lib/stage1_contract.sh"

BIN="${BIN:-target/selfhost/hakorune.stage1_cli}"
ENTRY="${1:-lang/src/compiler/entry/compiler_stageb.hako}"

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

src_text="$(cat "$ENTRY")"
prog_out="$tmp_dir/program.out"
prog_json="$tmp_dir/program.json"
mir_source_out="$tmp_dir/mir.source.out"
mir_supplied_out="$tmp_dir/mir.supplied.out"
mir_source_json="$tmp_dir/mir.source.json"
mir_supplied_json="$tmp_dir/mir.supplied.json"

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

stage1_contract_exec_mode "$BIN" emit-program "$ENTRY" "$src_text" >"$prog_out"
extract_json "$prog_out" "$prog_json"

stage1_contract_exec_mode "$BIN" emit-mir "$ENTRY" "$src_text" >"$mir_source_out"
stage1_contract_exec_mode "$BIN" emit-mir "$ENTRY" "$src_text" "$prog_json" >"$mir_supplied_out"

extract_json "$mir_source_out" "$mir_source_json"
extract_json "$mir_supplied_out" "$mir_supplied_json"

echo "[transient-boundary] bin=$BIN"
echo "[transient-boundary] entry=$ENTRY"
echo "[transient-boundary] compare=source-only vs supplied-program-json"
python3 "$ROOT/tools/selfhost/lib/mir_canonical_compare.py" summarize-first-diff \
  "$mir_source_json" "$mir_supplied_json"
