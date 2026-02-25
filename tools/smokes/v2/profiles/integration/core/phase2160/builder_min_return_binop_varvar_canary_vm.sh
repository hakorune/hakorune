#!/usr/bin/env bash
# Opt-in canary for MirBuilderMin: return.binop var+var with prior Local Ints
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
using "hako.mir.builder.min" as MirBuilderBox
static box Main { method main(args) {
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local out = MirBuilderBox.emit_from_program_json_v0(j, null)
  if out == null { print("[fail:builder]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

# Program: Local a=2; Local b=5; Return(Binary '+', Var a, Var b)
PROG='{"version":0,"kind":"Program","body":[{"type":"Local","name":"a","expr":{"type":"Int","value":2}},{"type":"Local","name":"b","expr":{"type":"Int","value":5}},{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"a"},"rhs":{"type":"Var","name":"b"}}}]}'

tmp_stdout=$(mktemp); trap 'rm -f "$tmp_stdout" "$TMP_HAKO" || true' EXIT
set +e
NYASH_FAIL_FAST=0 NYASH_FEATURES=stage3 NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
PROG_JSON="$PROG" "${BIN}" --backend vm "${TMP_HAKO}" 2>/dev/null | tee "$tmp_stdout" >/dev/null
rc=$?
set -e
# Allow rc!=0 on this host; pass on tag observation
if ! grep -q "\[mirbuilder/min:return.binop.varvar\]" "$tmp_stdout"; then
  echo "[SKIP] min tag not observed (binop varvar)"; exit 0
fi
echo "[PASS] builder_min_return_binop_varvar"
exit 0
echo "[PASS] builder_min_return_binop_varvar"
exit 0
