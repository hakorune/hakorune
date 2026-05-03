#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
source "$ROOT/tools/selfhost/lib/stage1_contract.sh"

BIN_STAGE1="${BIN_STAGE1:-target/selfhost/hakorune.stage1_cli}"
BIN_STAGE2="${BIN_STAGE2:-target/selfhost/hakorune.stage1_cli.stage2}"
ENTRY="${1:-lang/src/compiler/entry/compiler_stageb.hako}"

if [[ ! -x "$BIN_STAGE1" ]]; then
  echo "[FAIL] missing stage1 bin: $BIN_STAGE1" >&2
  exit 2
fi
if [[ ! -x "$BIN_STAGE2" ]]; then
  echo "[FAIL] missing stage2 bin: $BIN_STAGE2" >&2
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
s1_out="$tmp_dir/stage1.out"
s2_out="$tmp_dir/stage2.out"
s1_json="$tmp_dir/stage1.mir.json"
s2_json="$tmp_dir/stage2.mir.json"

stage1_contract_exec_mode "$BIN_STAGE1" emit-mir "$ENTRY" "$src_text" >"$s1_out"
stage1_contract_exec_mode "$BIN_STAGE2" emit-mir "$ENTRY" "$src_text" >"$s2_out"

python3 - "$s1_out" "$s1_json" "$s2_out" "$s2_json" <<'PY'
import json
import pathlib
import sys

def extract_json(text: str):
    start = text.find("{")
    if start < 0:
        raise SystemExit("missing JSON object")
    return json.loads(text[start:])

for src, out in ((sys.argv[1], sys.argv[2]), (sys.argv[3], sys.argv[4])):
    data = extract_json(pathlib.Path(src).read_text())
    pathlib.Path(out).write_text(json.dumps(data, ensure_ascii=False))
PY

echo "[probe] entry=$ENTRY"
echo "[probe] stage1=$BIN_STAGE1"
echo "[probe] stage2=$BIN_STAGE2"
echo "[probe] files stage1=$s1_json stage2=$s2_json"

python3 "$ROOT/tools/selfhost/lib/mir_canonical_compare.py" summarize-first-diff "$s1_json" "$s2_json"
