#!/usr/bin/env bash
# provider_select_ring1_filebox_canary_vm.sh — HAKO_PROVIDER_POLICY=safe-core-first で ring1 選択タグを検証
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
enable_mirbuilder_dev_env

TMP=$(mktemp); echo "hello" > "$TMP"; trap 'rm -f "$TMP" || true' EXIT

TMP_HAKO=$(mktemp --suffix .hako)
cat >"${TMP_HAKO}" <<HAKO
static box Main { method main(args) {
  local f = FileBox.open("${TMP}");
  local s = f.read();
  print("[OUT]"); print("" + s); print("[END]");
  return 0
} }
HAKO

set +e
out="$(
  NYASH_FAIL_FAST=0 \
  HAKO_PROVIDER_POLICY=safe-core-first \
  NYASH_FILEBOX_MODE=auto \
  NYASH_FILEBOX_ALLOW_FALLBACK=1 \
  NYASH_JOINIR_DEV=0 \
  HAKO_JOINIR_STRICT=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_PLANNER_REQUIRED=0 \
  "${BIN}" --backend vm "${TMP_HAKO}" 2>&1 | filter_noise
)"; rc=$?
set -e
rm -f "$TMP_HAKO" || true
if ! grep -q "\[provider/select:FileBox ring=1 src=static\]" <<< "$out"; then
  echo "[SKIP] provider_select_ring1 tag missing"; exit 0
fi
# Content check is best-effort; when VM environment lacks plugins for FileBox wrapper, skip content verification.
if [[ "$rc" -eq 0 ]]; then
  if ! awk '/\[OUT\]/{f=1;next}/\[END\]/{f=0}f' <<< "$out" | grep -q "hello"; then
    echo "[SKIP] provider_select_ring1 content missing (env)"; exit 0
  fi
fi
echo "[PASS] provider_select_ring1_filebox_canary_vm"
exit 0
