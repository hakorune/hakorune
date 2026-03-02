#!/usr/bin/env bash
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$ROOT"

FIXTURE="$ROOT/apps/tests/phase216_mainline_binop_base_min.hako"
if [[ ! -f "$FIXTURE" ]]; then
  echo "[FAIL] missing fixture: $FIXTURE" >&2
  exit 1
fi

TMP_JSON=$(mktemp --suffix .json)
OUT_EXE=$(mktemp --suffix .exe)

HAKO_SELFHOST_BUILDER_FIRST=1 \
NYASH_USE_NY_COMPILER=0 HAKO_DISABLE_NY_COMPILER=1 \
NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
  bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-mainline --timeout-secs "${HAKO_BUILD_TIMEOUT:-60}" --out "$TMP_JSON" --input "$FIXTURE" >/dev/null

NYASH_LLVM_BACKEND=crate NYASH_LLVM_SKIP_BUILD=1 \
NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
  bash "$ROOT/tools/ny_mir_builder.sh" --in "$TMP_JSON" --emit exe -o "$OUT_EXE" --quiet >/dev/null

set +e
"$OUT_EXE"; rc=$?
set -e
[[ "$rc" == "3" ]] && echo "[PASS] phase216_binop_base rc=3" || { echo "[FAIL] phase216_binop_base rc=$rc (expect 3)" >&2; exit 1; }

rm -f "$TMP_JSON" "$OUT_EXE" 2>/dev/null || true
exit 0
