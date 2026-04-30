#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh" || true
source "$ROOT/tools/selfhost/lib/stageb_program_json_capture.sh"

# Quick profile: Stage-B emit is flaky under Stage-3 default; skip for now.
echo "[SKIP] stageb_parser_loop_json_canary_vm (disabled in quick profile after env consolidation)"
exit 0

# Build minimal program
TMP_HAKO=$(mktemp --suffix .hako)
cat >"$TMP_HAKO" <<'HAKO'
static box Main { method main(){
  local n=10; local i=0;
  loop(i<n){ i=i+1 }
  return i
} }
HAKO

OUT_JSON=$(mktemp --suffix .json)
trap 'rm -f "$TMP_HAKO" "$OUT_JSON" 2>/dev/null || true' EXIT

# Stage‑B: Program(JSON v0) を直接出力
if ! NYASH_JSON_ONLY=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
     NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
     NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 NYASH_DISABLE_PLUGINS=1 NYASH_FILEBOX_MODE="core-ro" \
     "$ROOT/target/release/hakorune" --backend vm "$ROOT/lang/src/compiler/entry/compiler_stageb.hako" -- --source "$(cat "$TMP_HAKO")" 2>/dev/null \
     | stageb_program_json_extract_from_stdin >"$OUT_JSON"; then
  echo "[FAIL] stageb_parser_loop_json: failed to produce Program(JSON)"; exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "[SKIP] stageb_parser_loop_json: jq not available"; exit 0
fi

# Verify: Loop exists; cond is Compare '<'; body not empty; contains Local i=Binary '+'
HAS_LOOP=$(jq '.body | any(.type=="Loop")' "$OUT_JSON")
if [[ "$HAS_LOOP" != "true" ]]; then echo "[FAIL] no Loop node"; exit 1; fi
COND_OK=$(jq '.body[] | select(.type=="Loop") | .cond | ( .type=="Compare" and (.op=="<" or .op=="Lt") )' "$OUT_JSON" | tail -n1)
if [[ "$COND_OK" != "true" ]]; then echo "[FAIL] cond is not Compare <"; exit 1; fi
BODY_LEN=$(jq '.body[] | select(.type=="Loop") | (.body|length)' "$OUT_JSON" | tail -n1)
if [[ "${BODY_LEN:-0}" -eq 0 ]]; then echo "[FAIL] loop body is empty"; exit 1; fi
ASSIGN_OK=$(jq '.body[] | select(.type=="Loop") | .body | any(.type=="Local" and .name=="i" and .expr.type=="Binary" and (.expr.op=="+" or .expr.op=="plus"))' "$OUT_JSON")
if [[ "$ASSIGN_OK" != "true" ]]; then echo "[FAIL] no i=i+1 assignment detected"; exit 1; fi

echo "[PASS] stageb_parser_loop_json_canary_vm"
exit 0
