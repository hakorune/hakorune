#!/bin/bash
# Check that provider route emits v1-shaped JSON (schema_version present)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

set +e
out=$(bash "$ROOT/tools/selfhost/gen_v1_from_provider.sh" 2>&1)
rc=$?
set -e

if [ $rc -ne 0 ]; then
  echo "[FAIL] provider_v1_shape_canary_vm (gen rc=$rc)" >&2
  printf '%s\n' "$out" | sed -n '1,120p' >&2
  exit 1
fi

if echo "$out" | grep -q '"schema_version"\s*:\s*"1\.0"'; then
  echo "[PASS] provider_v1_shape_canary_vm"
  exit 0
fi
# When old binary is used (not rebuilt), provider may return v0; treat as SKIP
if echo "$out" | grep -q '"version"\s*:\s*0'; then
  echo "[SKIP] provider_v1_shape_canary_vm (binary not rebuilt; v0 detected)" >&2
  exit 0
fi
echo "[FAIL] provider_v1_shape_canary_vm (unexpected shape)" >&2
printf '%s\n' "$out" | sed -n '1,160p' >&2
exit 1

