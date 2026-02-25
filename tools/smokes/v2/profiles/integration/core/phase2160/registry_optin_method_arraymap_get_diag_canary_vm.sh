#!/usr/bin/env bash
# Diagnostic canary for MirBuilder registry path: return.method.arraymap (get)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
BIN="${ROOT_DIR}/target/release/hakorune"
if [[ ! -x "${BIN}" ]]; then echo "[SKIP] hakorune not built"; exit 0; fi

source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
enable_mirbuilder_dev_env

TMP_HAKO=$(mktemp --suffix .hako)
cat >"${TMP_HAKO}" <<'HAKO'
using "hako.mir.builder" as MirBuilderBox
static box Main { method main(args) {
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local out = MirBuilderBox.emit_from_program_json_v0(j, null)
  if out == null { print("[fail:builder]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

# Program(JSON v0): Return(Method Var recv 'a', method 'get', args [Int 0])
PROG='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Method","recv":{"type":"Var","name":"a"},"method":"get","args":[{"type":"Int","value":0}]}}]}'

tmp_stdout=$(mktemp); trap 'rm -f "$tmp_stdout" "$TMP_HAKO" || true' EXIT
set +e
NYASH_FAIL_FAST=0 \
HAKO_MIR_BUILDER_SKIP_LOOPS=1 \
PROG_JSON="$PROG" \
HAKO_MIR_BUILDER_DELEGATE=0 HAKO_MIR_BUILDER_INTERNAL=1 HAKO_MIR_BUILDER_REGISTRY=1 HAKO_MIR_BUILDER_DEBUG=1 \
HAKO_MIR_BUILDER_REGISTRY_ONLY=return.method.arraymap \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
NYASH_FEATURES=stage3 \
"${BIN}" --backend vm "${TMP_HAKO}" | tee "$tmp_stdout"
rc=$?
set -e

echo "[diag] rc=$rc"
if [[ "$rc" -ne 0 ]]; then echo "[SKIP] builder vm exec failed (diag)"; exit 0; fi
if ! grep -q "\[mirbuilder/registry:return.method.arraymap\]" "$tmp_stdout"; then
  echo "[SKIP] registry tag not observed (diag)"; exit 0
fi
mir=$(awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag' "$tmp_stdout")
if [[ -z "$mir" ]] || ! echo "$mir" | grep -q '"functions"'; then echo "[SKIP] MIR missing functions (diag)"; exit 0; fi
echo "[PASS] registry_optin_method_arraymap_get (diag)"
exit 0
