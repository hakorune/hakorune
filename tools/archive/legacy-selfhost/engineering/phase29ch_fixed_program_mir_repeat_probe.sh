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

prog_out="$tmp_dir/program.out"
prog_json="$tmp_dir/program.json"
mir_a_out="$tmp_dir/mir.a.out"
mir_b_out="$tmp_dir/mir.b.out"
mir_a_json="$tmp_dir/mir.a.json"
mir_b_json="$tmp_dir/mir.b.json"

stage1_contract_exec_mode "$BIN" emit-program "$ENTRY" "$src_text" >"$prog_out"
extract_json "$prog_out" "$prog_json"

HAKO_SELFHOST_NO_DELEGATE=1 \
HAKO_MIR_BUILDER_DELEGATE=0 \
  stage1_contract_exec_mode "$BIN" emit-mir "$ENTRY" "$src_text" "$prog_json" >"$mir_a_out"

HAKO_SELFHOST_NO_DELEGATE=1 \
HAKO_MIR_BUILDER_DELEGATE=0 \
  stage1_contract_exec_mode "$BIN" emit-mir "$ENTRY" "$src_text" "$prog_json" >"$mir_b_out"

extract_json "$mir_a_out" "$mir_a_json"
extract_json "$mir_b_out" "$mir_b_json"

echo "[fixed-program] bin=$BIN"
echo "[fixed-program] entry=$ENTRY"
echo "[fixed-program] program-json source: exact saved payload"
python3 "$ROOT/tools/selfhost/lib/mir_canonical_compare.py" summarize-first-diff "$mir_a_json" "$mir_b_json"
