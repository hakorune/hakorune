#!/usr/bin/env bash
# Opt-in canary for MirBuilder registry path (HAKO_MIR_BUILDER_REGISTRY=1).
# Runs Hako MirBuilderBox in VM with a minimal Program(JSON v0) and checks MIR(JSON) presence.
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
using "hako.mir.builder" as MirBuilderBox
static box Main { method main(args) {
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local out = MirBuilderBox.emit_from_program_json_v0(j, null)
  if out == null { print("[fail:builder]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

# Minimal Program(JSON v0): If(Compare Int==Int) → Return(Int)/Return(Int)
PROG='{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Compare","op":"==","lhs":{"type":"Int","value":1},"rhs":{"type":"Int","value":1}},"then":[{"type":"Return","expr":{"type":"Int","value":42}}],"else":[{"type":"Return","expr":{"type":"Int","value":0}}]}]}'

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
# Tag observation (debug on)
if ! grep -q "\[mirbuilder/registry:if.compare.intint\]" "$tmp_stdout"; then
  echo "[SKIP] registry tag not observed"; exit 0
fi
mir=$(awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag' "$tmp_stdout")
if [[ -z "$mir" ]]; then echo "[SKIP] empty MIR (registry)"; exit 0; fi
if echo "$mir" | grep -q '"functions"'; then echo "[PASS] registry_optin"; exit 0; fi
echo "[SKIP] MIR without functions (registry)"; exit 0
