#!/bin/bash
# Repeat const42 v1 generation 3 times and ensure normalized hash matches
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

set +e
out=$(bash "$ROOT/tools/selfhost/bootstrap_s1_s2_s3_repeat.sh" 'bash tools/selfhost/gen_v1_from_builder.sh' 2>&1)
rc=$?
set -e
if [ "$rc" -eq 0 ]; then
  echo "[PASS] s1s2s3_repeat_const_canary_vm"
  exit 0
fi
echo "[FAIL] s1s2s3_repeat_const_canary_vm (rc=$rc)" >&2
printf '%s\n' "$out" | sed -n '1,200p' >&2
exit 1

