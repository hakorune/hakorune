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
stdout_file="$tmp_dir/impossible.stdout"
stderr_file="$tmp_dir/impossible.stderr"

set +e
HAKO_SELFHOST_NO_DELEGATE=1 \
HAKO_MIR_BUILDER_DELEGATE=0 \
HAKO_MIR_BUILDER_INTERNAL=0 \
  stage1_contract_exec_mode "$BIN" emit-mir "$ENTRY" "$src_text" >"$stdout_file" 2>"$stderr_file"
rc=$?
set -e

echo "[impossible-gate] bin=$BIN"
echo "[impossible-gate] entry=$ENTRY"
echo "[impossible-gate] env=HAKO_SELFHOST_NO_DELEGATE=1 HAKO_MIR_BUILDER_DELEGATE=0 HAKO_MIR_BUILDER_INTERNAL=0"
echo "[impossible-gate] rc=$rc"

if grep -Eq '^[[:space:]]*\{.*"functions"[[:space:]]*:.*\}[[:space:]]*$' "$stdout_file"; then
  echo "[impossible-gate] payload=mir-json"
  echo "[impossible-gate] unexpected success: impossible gate still emitted MIR" >&2
  exit 1
fi

if [[ -s "$stdout_file" ]]; then
  echo "[impossible-gate] payload=non-json"
  sed -n '1,40p' "$stdout_file"
else
  echo "[impossible-gate] payload=empty"
fi

if [[ -s "$stderr_file" ]]; then
  echo "[impossible-gate] stderr:"
  sed -n '1,80p' "$stderr_file"
fi

if [[ "$rc" -eq 0 ]]; then
  echo "[impossible-gate] unexpected rc=0 without MIR payload" >&2
  exit 1
fi

echo "[impossible-gate] PASS"
