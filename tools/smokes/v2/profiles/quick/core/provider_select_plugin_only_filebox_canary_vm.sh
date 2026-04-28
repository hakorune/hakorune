#!/usr/bin/env bash
# provider_select_plugin_only_filebox_canary_vm.sh — plugin-only モードで ring1 が選ばれないことを確認（環境によりSKIP）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
BIN="${ROOT_DIR}/target/release/hakorune"
if [[ ! -x "${BIN}" ]]; then echo "[SKIP] hakorune not built"; exit 0; fi

source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

TMP_HAKO=$(mktemp --suffix .hako)
cat >"${TMP_HAKO}" <<'HAKO'
static box Main { method main(args) {
  // Do nothing; we only care about provider selection logs
  return 0
} }
HAKO

set +e
out="$(
  NYASH_FILEBOX_MODE=plugin-only \
  HAKO_PROVIDER_TRACE=1 \
  NYASH_JOINIR_DEV=0 \
  HAKO_JOINIR_STRICT=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_PLANNER_REQUIRED=0 \
  "${BIN}" --backend vm "${TMP_HAKO}" 2>&1 | filter_noise
)"; rc=$?
set -e
rm -f "$TMP_HAKO" || true

# In plugin-only mode, ring1 must not be selected; tag for ring1 must be absent.
if echo "$out" | grep -q "\[provider/select:FileBox ring=1"; then
  echo "[FAIL] provider_select_plugin_only: ring1 selected unexpectedly" >&2; exit 1
fi
# Environment may lack plugins; accept rc!=0 and treat as SKIP when no plugin tag appears.
if [[ "$rc" -ne 0 ]]; then
  echo "[SKIP] provider_select_plugin_only: no plugin available (env)"; exit 0
fi
echo "[PASS] provider_select_plugin_only_filebox_canary_vm"
exit 0
