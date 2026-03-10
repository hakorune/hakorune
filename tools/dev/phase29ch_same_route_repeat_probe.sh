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

emit_once() {
  local mode="$1"
  local out="$2"
  case "$mode" in
    default)
      stage1_contract_exec_mode "$BIN" emit-mir "$ENTRY" "$src_text" >"$out"
      ;;
    internal-only)
      HAKO_SELFHOST_NO_DELEGATE=1 \
      HAKO_MIR_BUILDER_DELEGATE=0 \
        stage1_contract_exec_mode "$BIN" emit-mir "$ENTRY" "$src_text" >"$out"
      ;;
    delegate-only)
      HAKO_SELFHOST_NO_DELEGATE=0 \
      HAKO_MIR_BUILDER_DELEGATE=1 \
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

compare_pair() {
  local lhs="$1"
  local rhs="$2"
  local lhs_json="$tmp_dir/$lhs.json"
  local rhs_json="$tmp_dir/$rhs.json"
  echo "[repeat] $lhs vs $rhs"
  python3 "$ROOT/tools/selfhost/lib/mir_canonical_compare.py" summarize-first-diff "$lhs_json" "$rhs_json"
}

for mode in default internal-only delegate-only; do
  emit_once "$mode" "$tmp_dir/${mode}.a.out"
  emit_once "$mode" "$tmp_dir/${mode}.b.out"
  extract_json "$tmp_dir/${mode}.a.out" "$tmp_dir/${mode}.a.json"
  extract_json "$tmp_dir/${mode}.b.out" "$tmp_dir/${mode}.b.json"
done

echo "[repeat] bin=$BIN"
echo "[repeat] entry=$ENTRY"
compare_pair "default.a" "default.b"
compare_pair "internal-only.a" "internal-only.b"
compare_pair "delegate-only.a" "delegate-only.b"
