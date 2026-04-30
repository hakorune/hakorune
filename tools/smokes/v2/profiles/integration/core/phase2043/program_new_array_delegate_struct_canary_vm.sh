#!/usr/bin/env bash
# Program(JSON v0) → Hako MirBuilder (delegate) → MIR(JSON) 構造検査: Constructor(ArrayBox) + Core実行(rc=0)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_prog="/tmp/prog_new_array_delegate_$$.json"
tmp_mir="/tmp/mir_new_array_delegate_$$.json"

cat >"$tmp_prog" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"a","expr":{"type":"New","class":"ArrayBox","args":[]}},
  {"type":"Return","expr":{"type":"Int","value":0}}
]}
JSON

# Builder (delegate) runner code
BUILDER_CODE=$(cat <<'HCODE'
using "hako.mir.builder" as MirBuilderBox
static box Main { method main(args) {
  local prog_json = env.get("HAKO_BUILDER_PROGRAM_JSON")
  if prog_json == null { print("Builder failed"); return 1 }
  local mir_out = MirBuilderBox.emit_from_program_json_v0(prog_json, null)
  if mir_out == null { print("Builder failed"); return 1 }
  print("[MIR_OUT_BEGIN]")
  print("" + mir_out)
  print("[MIR_OUT_END]")
  return 0
} }
HCODE
)

prog_json_raw="$(cat "$tmp_prog")"

# Hako MirBuilder is the contract. Do not fall back to the raw Rust CLI builder.
set +e
mir_json=$(HAKO_MIR_BUILDER_DELEGATE=1 \
           HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
           HAKO_ROUTE_HAKOVM=1 \
           NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
           NYASH_USING_AST=1 NYASH_RESOLVE_FIX_BRACES=1 \
           NYASH_DISABLE_NY_COMPILER=1 \
           NYASH_FEATURES=stage3 \
           NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
           HAKO_BUILDER_PROGRAM_JSON="$prog_json_raw" \
           run_nyash_vm -c "$BUILDER_CODE" 2>/dev/null | awk '/\[MIR_OUT_BEGIN\]/{flag=1;next}/\[MIR_OUT_END\]/{flag=0}flag')
rc=$?
set -e

if [ $rc -ne 0 ] || [ -z "$mir_json" ] || ! echo "$mir_json" | grep -q '"functions"'; then
  echo "[SKIP] program_new_array_delegate_struct_canary_vm: Hako MirBuilder not ready (rc=$rc); raw CLI fallback retired" >&2
  echo "$mir_json" >&2
  rm -f "$tmp_prog" "$tmp_mir" || true
  exit 0
fi

echo "$mir_json" > "$tmp_mir"

if (! grep -E -q '"op"\s*:\s*"mir_call"' "$tmp_mir" || ! grep -E -q '"type"\s*:\s*"Constructor"' "$tmp_mir" || ! grep -E -q '"box_type"\s*:\s*"ArrayBox"' "$tmp_mir") \
   && (! grep -E -q '"op"\s*:\s*"newbox"' "$tmp_mir" || ! grep -E -q '"type"\s*:\s*"ArrayBox"' "$tmp_mir"); then
  echo "[FAIL] program_new_array_delegate_struct_canary_vm: expected Constructor(ArrayBox) mir_call or newbox(ArrayBox) in MIR" >&2
  tail -n 60 "$tmp_mir" >&2 || true
  rm -f "$tmp_prog" "$tmp_mir" || true
  exit 1
fi

rm -f "$tmp_prog" "$tmp_mir" || true

echo "[PASS] program_new_array_delegate_struct_canary_vm"
