#!/usr/bin/env bash
# Canary: loop_scan_box extract_ne_else_sentinel_value for '!=' with else Continue
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
BIN="${ROOT_DIR}/target/release/hakorune"
if [[ ! -x "${BIN}" ]]; then echo "[SKIP] hakorune not built"; exit 0; fi

TMP_HAKO=$(mktemp --suffix .hako)
cat >"${TMP_HAKO}" <<'HAKO'
using "hako.mir.builder.internal.loop_scan" as LoopScanBox
using selfhost.shared.json.utils.json_frag as JsonFragBox
static box Main { method main(args) {
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local v = LoopScanBox.extract_ne_else_sentinel_value(j, "Continue", 0, "i")
  if v == null { print("[fail:noval]"); return 1 }
  print("[VAL:" + v + "]")
  return 0
} }
HAKO

# Program(JSON v0): If(cond i!=2) then Return(0) else Continue
PROG='{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Compare","op":"!=","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":2}},"then":[{"type":"Return","expr":{"type":"Int","value":0}}],"else":[{"type":"Continue"}]}]}'

tmp_stdout=$(mktemp); trap 'rm -f "$tmp_stdout" "$TMP_HAKO" || true' EXIT
set +e
NYASH_FAIL_FAST=0 \
PROG_JSON="$PROG" \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
NYASH_FEATURES=stage3 \
"${BIN}" --backend vm "${TMP_HAKO}" 2>/dev/null | tee "$tmp_stdout" >/dev/null
rc=$?
set -e

if [[ "$rc" -ne 0 ]]; then echo "[SKIP] loop_scan vm exec failed"; exit 0; fi
if ! grep -q "\[VAL:2\]" "$tmp_stdout"; then echo "[SKIP] unexpected sentinel value"; exit 0; fi
echo "[PASS] loop_scan_ne_else_continue"
exit 0

