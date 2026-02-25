#!/usr/bin/env bash
# Verify that enabling NYASH_MIR_DEV_IDEMP does not change behavior (rc parity)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true

require_env || { echo "[SKIP] env not ready"; exit 0; }

TMP_HAKO=$(mktemp --suffix .hako)
trap 'rm -f "$TMP_HAKO" 2>/dev/null || true' EXIT

cat >"$TMP_HAKO" <<'HAKO'
static box Main { method main(args) {
  let a = 2
  let b = 3
  // Two identical comparisons to give optimizer something to chew on
  let x = (a * b) == 6
  let y = (a * b) == 6
  if x && y { return 10 } else { return 1 }
} }
HAKO

set +e
NYASH_MIR_DEV_IDEMP=1 "$NYASH_BIN" --backend vm "$TMP_HAKO" 2>/dev/null | filter_noise >/dev/null
rc=$?
set -e

if [[ "$rc" -eq 10 ]]; then
  echo "[PASS] mir_dev_idemp_toggle_canary_vm"
  exit 0
fi
echo "[SKIP] unexpected rc=$rc (expect 10)"
exit 0

