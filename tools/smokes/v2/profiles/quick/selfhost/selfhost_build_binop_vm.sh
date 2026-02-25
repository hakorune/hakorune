#!/bin/bash
# selfhost_build_binop_vm.sh — Hybrid selfhost build: 1+2*3 -> 7 (opt‑in)

set -euo pipefail
if [ "${SMOKES_ENABLE_SELFHOST:-0}" != "1" ]; then
  echo "[SKIP] selfhost_build_binop_vm (SMOKES_ENABLE_SELFHOST=1 to enable)"
  exit 0
fi
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

tmp_hako="/tmp/selfhost_binop_$$.hako"
cat > "$tmp_hako" <<'HAKO'
static box Main { method main(args) { local a=1; local b=2; local c=3; return a + b * c } }
HAKO

set +e
"$ROOT/tools/selfhost/selfhost_build.sh" --in "$tmp_hako" --run >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_hako" 2>/dev/null || true

if [ "$rc" = "7" ]; then
  echo "[PASS] selfhost_build_binop_vm"
else
  echo "[FAIL] selfhost_build_binop_vm (rc=$rc)" >&2
  exit 1
fi

