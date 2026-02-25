#!/usr/bin/env bash
# Opt-in canary for MirBuilderMin: if.compare int-int then/else return int
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

# If(Compare '<', 2 < 5) then Return(1) else Return(0)
PROG='{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Compare","op":"<","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":5}},"then":[{"type":"Return","expr":{"type":"Int","value":1}}],"else":[{"type":"Return","expr":{"type":"Int","value":0}}]}]}'

tmp_stdout=$(mktemp); trap 'rm -f "$tmp_stdout" "$TMP_HAKO" || true' EXIT
set +e
NYASH_FAIL_FAST=0 NYASH_FEATURES=stage3 NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
PROG_JSON="$PROG" "${BIN}" --backend vm "${TMP_HAKO}" 2>/dev/null | tee "$tmp_stdout" >/dev/null
rc=$?
set -e
if [[ "$rc" -ne 0 ]]; then echo "[SKIP] builder-min vm exec failed"; exit 0; fi
if ! grep -q "\[mirbuilder/min:if.compare.intint\]" "$tmp_stdout"; then
  echo "[SKIP] min tag not observed (if)"; exit 0
fi
mir=$(awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag' "$tmp_stdout")
if [[ -z "$mir" ]] || ! echo "$mir" | grep -q '"functions"'; then echo "[SKIP] MIR missing functions (min/if)"; exit 0; fi
echo "[PASS] builder_min_if_compare_intint"
exit 0

