#!/usr/bin/env bash
# JsonFragBox.read_string_after contract canary.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
BIN="${ROOT_DIR}/target/release/hakorune"
if [[ ! -x "${BIN}" ]]; then echo "[SKIP] hakorune not built"; exit 0; fi

FIXTURE="${ROOT_DIR}/apps/tests/phase29bq_json_frag_read_string_contract_min.hako"
tmp_stdout=$(mktemp)
trap 'rm -f "$tmp_stdout" || true' EXIT

set +e
NYASH_FAIL_FAST=0 \
JSON_ONE='{"name":"i"}' \
JSON_EMPTY='{"name":""}' \
JSON_BAD='{"name":i}' \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
NYASH_FEATURES=stage3 \
"${BIN}" --backend vm "$FIXTURE" 2>/dev/null | tee "$tmp_stdout" >/dev/null
rc=$?
set -e

if [[ "$rc" -ne 0 ]]; then
  echo "[FAIL] phase29bq_json_frag_read_string_contract_vm rc=$rc" >&2
  cat "$tmp_stdout" >&2
  exit 1
fi
if ! grep -qx "json_frag_read_string_contract=ok" "$tmp_stdout"; then
  echo "[FAIL] phase29bq_json_frag_read_string_contract_vm unexpected output" >&2
  cat "$tmp_stdout" >&2
  exit 1
fi
echo "[PASS] phase29bq_json_frag_read_string_contract_vm"
