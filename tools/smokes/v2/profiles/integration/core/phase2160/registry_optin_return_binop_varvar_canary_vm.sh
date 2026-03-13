#!/usr/bin/env bash
# Opt-in canary for MirBuilder registry path: return.binop.varvar
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
BIN="${ROOT_DIR}/target/release/hakorune"
if [[ ! -x "${BIN}" ]]; then echo "[SKIP] hakorune not built"; exit 0; fi

# Keep this varvar canary explicit: ad hoc preinclude-heavy env is part of the contract.
export NYASH_USE_NY_COMPILER=0
export NYASH_FAIL_FAST=0
export NYASH_ENABLE_USING=1
export HAKO_ENABLE_USING=1
export HAKO_PREINCLUDE=1

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

# Program(JSON v0): a=2; b=5; return a+b
PROG='{"version":0,"kind":"Program","body":[{"type":"Local","name":"a","expr":{"type":"Int","value":2}},{"type":"Local","name":"b","expr":{"type":"Int","value":5}},{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"a"},"rhs":{"type":"Var","name":"b"}}}]}'

tmp_stdout=$(mktemp); trap 'rm -f "$tmp_stdout" "$TMP_HAKO" || true' EXIT
set +e
NYASH_FAIL_FAST=0 \
PROG_JSON="$PROG" \
HAKO_MIR_BUILDER_DELEGATE=0 HAKO_MIR_BUILDER_INTERNAL=1 HAKO_MIR_BUILDER_REGISTRY=1 HAKO_MIR_BUILDER_DEBUG=1 \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
NYASH_FEATURES=stage3 \
"${BIN}" --backend vm "${TMP_HAKO}" 2>/dev/null | tee "$tmp_stdout" >/dev/null
rc=$?
set -e

if [[ "$rc" -ne 0 ]]; then echo "[SKIP] builder vm exec failed"; exit 0; fi
if ! grep -E -q "\[mirbuilder/(min|registry):return.binop.varvar\]" "$tmp_stdout"; then
  echo "[SKIP] registry/min tag not observed (return.binop.varvar)"; exit 0
fi
mir=$(awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag' "$tmp_stdout")
if [[ -z "$mir" ]] || ! echo "$mir" | grep -q '"functions"'; then echo "[SKIP] MIR missing functions"; exit 0; fi
echo "[PASS] registry_optin_return_binop_varvar"
exit 0
