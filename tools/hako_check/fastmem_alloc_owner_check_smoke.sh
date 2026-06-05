#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE_DIR="$ROOT/tools/hako_check/tests/fastmem_capability_inventory"
GOOD_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_alloc_owner_check_good.XXXXXX")"
BAD_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_alloc_owner_check_bad.XXXXXX")"
trap 'rm -f "$GOOD_OUT" "$BAD_OUT"' EXIT

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --report "$FIXTURE_DIR/report.kv" \
  --format kv \
  >"$GOOD_OUT"

grep -q '^output_contract=hako-check-fastmem-check-v0$' "$GOOD_OUT"
grep -q '^failure_count=0$' "$GOOD_OUT"
grep -q '^summary=ok$' "$GOOD_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_owner_state_inventory.kv" \
  --format kv \
  >"$BAD_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad owner-state inventory" >&2
  cat "$BAD_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=9$' "$BAD_OUT"
grep -q '^failure_0_reason=worker_id_equals_os_thread_id_claim$' "$BAD_OUT"
grep -q '^failure_1_reason=worker_id_equals_runtime_worker_id_claim$' "$BAD_OUT"
grep -q '^failure_2_reason=page_owner_count_mismatch$' "$BAD_OUT"
grep -q '^failure_3_reason=page_owner_stale_generation_count$' "$BAD_OUT"
grep -q '^failure_4_reason=hako_source_thread_support_claim$' "$BAD_OUT"
grep -q '^failure_5_reason=alloc_owner_id_kind$' "$BAD_OUT"
grep -q '^failure_6_reason=worker_id_kind$' "$BAD_OUT"
grep -q '^failure_7_reason=allocator_tls_arena_init_count$' "$BAD_OUT"
grep -q '^failure_8_reason=page_owner_check_route$' "$BAD_OUT"
grep -q '^summary=failed$' "$BAD_OUT"

echo "[TEST/OK] fastmem_alloc_owner_check"
