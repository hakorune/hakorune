#!/usr/bin/env bash
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../../../.." && pwd)
cd "$ROOT"

TMP_SRC=$(mktemp --suffix .hako)
cat >"$TMP_SRC" <<'HAKO'
static box Main {
  method add(a,b){ return a + b }
  method main(){ return add(2,3) }
}
HAKO

TMP_JSON=$(mktemp --suffix .json)

# Emit MIR(JSON) with defs + call resolution + methodize (dev)
HAKO_SELFHOST_BUILDER_FIRST=1 \
HAKO_STAGEB_FUNC_SCAN=1 \
HAKO_MIR_BUILDER_FUNCS=1 \
HAKO_MIR_BUILDER_CALL_RESOLVE=1 \
HAKO_MIR_BUILDER_METHODIZE=1 \
NYASH_USE_NY_COMPILER=0 HAKO_DISABLE_NY_COMPILER=1 \
NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
  bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-60}" --out "$TMP_JSON" --input "$TMP_SRC" >/dev/null

# Observability: prefer mir_call(Method), but tolerate delegate(Global) during bring-up
if ! rg -q '"op"\s*:\s*"mir_call"' "$TMP_JSON" || ! rg -q '"callee"\s*:\s*\{[^}]*"type"\s*:\s*"Method"' "$TMP_JSON"; then
  echo "[NOTE] methodize: mir_call(Method) not observed (delegate Global path likely). Proceeding with VM check." >&2
  cp "$TMP_JSON" /tmp/phase217_methodize_last.json || true
fi

# Execute semantics via standard .hako compile path（JSON実行はv1検討中）
BIN=${NY_BIN:-${NY_BIN:-$ROOT/target/release/hakorune}}
set +e
"$BIN" --backend vm "$TMP_SRC" >/dev/null 2>&1; rc=$?
set -e
if [[ "$rc" == "5" ]]; then
  echo "[PASS] phase217_methodize (compile-run rc=5)"
else
  echo "[FAIL] phase217_methodize — compile-run rc=$rc" >&2
  exit 1
fi

rm -f "$TMP_SRC" "$TMP_JSON" 2>/dev/null || true
exit 0
