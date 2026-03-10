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

run_probe() {
  local name="$1"
  shift
  local out="$tmp_dir/${name}.out"
  local err="$tmp_dir/${name}.err"
  local rc=0

  set +e
  (
    export "$@"
    stage1_contract_exec_mode "$BIN" emit-mir "$ENTRY" "$src_text"
  ) >"$out" 2>"$err"
  rc=$?
  set -e

  echo "[bridge-bypass] case=$name rc=$rc"
  if grep -Eq '^[[:space:]]*\{.*"functions"[[:space:]]*:.*\}[[:space:]]*$' "$out"; then
    echo "[bridge-bypass] case=$name payload=mir-json"
  elif [[ -s "$out" ]]; then
    echo "[bridge-bypass] case=$name payload=non-json"
    sed -n '1,20p' "$out"
  else
    echo "[bridge-bypass] case=$name payload=empty"
  fi
  if [[ -s "$err" ]]; then
    echo "[bridge-bypass] case=$name stderr:"
    sed -n '1,20p' "$err"
  fi
}

echo "[bridge-bypass] bin=$BIN"
echo "[bridge-bypass] entry=$ENTRY"
echo "[bridge-bypass] note=if bridge-only knobs are ignored while MIR still emits, current authority is not controlled by Rust bridge entry selection"

run_probe child-guard \
  NYASH_STAGE1_CLI_CHILD=1 \
  HAKO_SELFHOST_NO_DELEGATE=1 \
  HAKO_MIR_BUILDER_DELEGATE=0 \
  HAKO_MIR_BUILDER_INTERNAL=0

run_probe missing-entry-override \
  STAGE1_CLI_ENTRY=/tmp/__missing_stage1_entry__.hako \
  HAKO_SELFHOST_NO_DELEGATE=1 \
  HAKO_MIR_BUILDER_DELEGATE=0 \
  HAKO_MIR_BUILDER_INTERNAL=0
