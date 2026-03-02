#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && git rev-parse --show-toplevel 2>/dev/null || true)"
if [ -z "$ROOT" ]; then
  ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh" || true

INPUT="$ROOT/apps/tests/emit_boxcall_length_canary_vm.hako"
if [ ! -f "$INPUT" ]; then
  echo "[SKIP] stageb_min_emit: input not found: $INPUT" >&2
  exit 0
fi

TMP_JSON=$(mktemp --suffix .json)
trap 'rm -f "$TMP_JSON" 2>/dev/null || true' EXIT

# Stage‑B emit (RAW 保存は dev 用)
if ! NYASH_EMIT_MIR_KEEP_RAW=1 NYASH_EMIT_USE_COMPILER=1 NYASH_EMIT_MIR_TRACE="${NYASH_EMIT_MIR_TRACE:-0}" \
     NYASH_FEATURES="${NYASH_FEATURES:-stage3}" \
     bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-10}" --out "$TMP_JSON" --input "$INPUT" >/dev/null 2>&1; then
  echo "[FAIL] stageb_min_emit: failed to emit MIR JSON" >&2
  exit 1
fi

if [ ! -s "$TMP_JSON" ]; then
  echo "[FAIL] stageb_min_emit: JSON output missing or empty" >&2
  exit 1
fi

echo "[PASS] stageb_min_emit"
exit 0
