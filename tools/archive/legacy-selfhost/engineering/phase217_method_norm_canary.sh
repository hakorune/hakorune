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

# Emit MIR(JSON) via direct route (v1 schema + unified call contract)
HAKO_STAGEB_FUNC_SCAN=1 HAKO_MIR_BUILDER_FUNCS=1 HAKO_MIR_BUILDER_CALL_RESOLVE=1 \
  bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route direct --timeout-secs "${HAKO_BUILD_TIMEOUT:-60}" --out "$TMP_JSON" --input "$TMP_SRC" >/dev/null

# Check that names include arity suffix '/2'
if ! rg -q '"name"\s*:\s*"Main\.add/2"' "$TMP_JSON"; then
  echo "[FAIL] missing arity-suffixed function name Main.add/2" >&2
  exit 1
fi
if ! rg -q '"value"\s*:\s*"Main\.add/2"' "$TMP_JSON" \
   && ! (rg -q '"box_name"\s*:\s*"Main"' "$TMP_JSON" && rg -q '"name"\s*:\s*"add"' "$TMP_JSON" && rg -q '"type"\s*:\s*"Method"' "$TMP_JSON"); then
  echo "[FAIL] missing methodized call target for Main.add" >&2
  exit 1
fi

# Build and run EXE (crate)
NYASH_LLVM_BACKEND=crate NYASH_LLVM_SKIP_BUILD=1 \
NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
  bash "$ROOT/tools/ny_mir_builder.sh" --in "$TMP_JSON" --emit exe -o /tmp/phase217_norm.exe --quiet >/dev/null

set +e
/tmp/phase217_norm.exe; rc=$?
set -e
[[ "$rc" == "5" ]] && echo "[PASS] phase217_method_norm (rc=5, arity suffix present)" || { echo "[FAIL] rc=$rc" >&2; exit 1; }

rm -f "$TMP_SRC" "$TMP_JSON" /tmp/phase217_norm.exe 2>/dev/null || true
exit 0
