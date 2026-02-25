#!/usr/bin/env bash
# hakorune_emit_mir_return42_canary_vm.sh — Hako-first pipeline (Stage‑B → MirBuilder) emits MIR and runs rc=42
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

TMP_HAKO="/tmp/hako_emit_mir_42_$$.hako"
TMP_JSON="/tmp/hako_emit_mir_42_$$.json"
trap 'rm -f "$TMP_HAKO" "$TMP_JSON" || true' EXIT

cat >"$TMP_HAKO" <<'HAKO'
static box Main { method main(args) { return 42 } }
HAKO

set +e
out=$("$ROOT/tools/hakorune_emit_mir.sh" "$TMP_HAKO" "$TMP_JSON" 2>&1)
rc=$?
set -e
if [ $rc -ne 0 ] || [ ! -s "$TMP_JSON" ]; then
  echo "[FAIL] hakorune_emit_mir_return42_canary_vm (emit failed rc=$rc)" >&2
  printf '%s\n' "$out" | sed -n '1,120p' >&2
  exit 1
fi

set +e
"$NYASH_BIN" --mir-json-file "$TMP_JSON" >/dev/null 2>&1
rc=$?
set -e
if [ $rc -ne 42 ]; then
  echo "[FAIL] hakorune_emit_mir_return42_canary_vm (expected rc=42, got rc=$rc)" >&2
  head -n1 "$TMP_JSON" >&2 || true
  exit 1
fi

echo "[PASS] hakorune_emit_mir_return42_canary_vm"
exit 0

