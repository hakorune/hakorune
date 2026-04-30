#!/bin/bash
# selfhost_build_exe_return.sh — Build EXE via tools/selfhost_build.sh (opt‑in)

set -euo pipefail
if [ "${SMOKES_ENABLE_SELFHOST:-0}" != "1" ]; then
  echo "[SKIP] selfhost_build_exe_return (SMOKES_ENABLE_SELFHOST=1 to enable)"
  exit 0
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

# Check ny-llvmc
NYLL="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}"
if [ ! -f "$NYLL" ]; then
  echo "[SKIP] selfhost_build_exe_return (ny-llvmc not found)"
  exit 0
fi

tmp_hako="/tmp/selfhost_exe_return7_$$.hako"
tmp_exe="/tmp/selfhost_exe_return7_$$.out"
cat > "$tmp_hako" <<'HAKO'
static box Main { method main(args) { return 7 } }
HAKO

"$ROOT/tools/selfhost/selfhost_build.sh" --in "$tmp_hako" --exe "$tmp_exe" >/dev/null 2>&1 || { echo "[FAIL] selfhost_build_exe_return (build failed)" >&2; exit 1; }

set +e
"$tmp_exe" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_hako" "$tmp_exe" 2>/dev/null || true

if [ "$rc" = "7" ]; then
  echo "[PASS] selfhost_build_exe_return"
else
  echo "[FAIL] selfhost_build_exe_return (rc=$rc)" >&2
  exit 1
fi
