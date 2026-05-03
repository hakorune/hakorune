#!/usr/bin/env bash
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../../../.." && pwd)
cd "$ROOT"

TMP_SRC=$(mktemp --suffix .hako)
cat >"$TMP_SRC" <<'HAKO'
static box Main { method main(){ return 42 } }
HAKO

TMP_JSON=$(mktemp --suffix .json)
OUT_EXE=$(mktemp --suffix .exe)

HAKO_SELFHOST_BUILDER_FIRST=1 \
NYASH_USE_NY_COMPILER=0 HAKO_DISABLE_NY_COMPILER=1 \
NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
  bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-mainline --timeout-secs "${HAKO_BUILD_TIMEOUT:-60}" --out "$TMP_JSON" --input "$TMP_SRC" >/dev/null

NYASH_LLVM_BACKEND=crate NYASH_LLVM_SKIP_BUILD=1 \
NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
  bash "$ROOT/tools/ny_mir_builder.sh" --in "$TMP_JSON" --emit exe -o "$OUT_EXE" --quiet >/dev/null

set +e
"$OUT_EXE"; rc=$?
set -e
[[ "$rc" == "42" ]] && echo "[PASS] phase216_return rc=42" || { echo "[FAIL] rc=$rc" >&2; exit 1; }

rm -f "$TMP_SRC" "$TMP_JSON" "$OUT_EXE" 2>/dev/null || true
exit 0
