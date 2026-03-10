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

emit_mode() {
  local mode="$1"
  local out="$2"
  case "$mode" in
    default)
      stage1_contract_exec_mode "$BIN" emit-mir "$ENTRY" "$src_text" >"$out"
      ;;
    internal-only)
      HAKO_SELFHOST_NO_DELEGATE=1 \
        stage1_contract_exec_mode "$BIN" emit-mir "$ENTRY" "$src_text" >"$out"
      ;;
    delegate-only)
      HAKO_MIR_BUILDER_INTERNAL=0 \
        stage1_contract_exec_mode "$BIN" emit-mir "$ENTRY" "$src_text" >"$out"
      ;;
    *)
      echo "[FAIL] unsupported mode: $mode" >&2
      exit 2
      ;;
  esac
}

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

default_out="$tmp_dir/default.out"
internal_out="$tmp_dir/internal-only.out"
delegate_out="$tmp_dir/delegate-only.out"
default_json="$tmp_dir/default.json"
internal_json="$tmp_dir/internal-only.json"
delegate_json="$tmp_dir/delegate-only.json"

emit_mode default "$default_out"
emit_mode internal-only "$internal_out"
emit_mode delegate-only "$delegate_out"

extract_json "$default_out" "$default_json"
extract_json "$internal_out" "$internal_json"
extract_json "$delegate_out" "$delegate_json"

echo "[matrix] bin=$BIN"
echo "[matrix] entry=$ENTRY"
echo "[matrix] default vs internal-only"
python3 "$ROOT/tools/selfhost/lib/mir_canonical_compare.py" summarize-first-diff "$default_json" "$internal_json"
echo "[matrix] default vs delegate-only"
python3 "$ROOT/tools/selfhost/lib/mir_canonical_compare.py" summarize-first-diff "$default_json" "$delegate_json"
echo "[matrix] internal-only vs delegate-only"
python3 "$ROOT/tools/selfhost/lib/mir_canonical_compare.py" summarize-first-diff "$internal_json" "$delegate_json"
