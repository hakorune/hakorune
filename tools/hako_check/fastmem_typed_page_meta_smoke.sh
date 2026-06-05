#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE_DIR="$ROOT/tools/hako_check/tests/fastmem_capability_inventory"
GOOD_INV="$(mktemp "${TMPDIR:-/tmp}/hako_typed_page_meta_inv_good.XXXXXX")"
GOOD_CHECK="$(mktemp "${TMPDIR:-/tmp}/hako_typed_page_meta_check_good.XXXXXX")"
BAD_INV="$(mktemp "${TMPDIR:-/tmp}/hako_typed_page_meta_inv_bad.XXXXXX")"
BAD_CHECK="$(mktemp "${TMPDIR:-/tmp}/hako_typed_page_meta_check_bad.XXXXXX")"
trap 'rm -f "$GOOD_INV" "$GOOD_CHECK" "$BAD_INV" "$BAD_CHECK"' EXIT

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --report "$FIXTURE_DIR/report.kv" \
  >"$GOOD_INV"
grep -q '^typed_page_meta_handle=1$' "$GOOD_INV"
grep -q '^typed_page_meta_layout_verified=1$' "$GOOD_INV"
grep -q '^typed_page_meta_layout_id=PageMetaLayoutV0$' "$GOOD_INV"
grep -q '^typed_page_meta_field_count=7$' "$GOOD_INV"
grep -q '^typed_page_meta_required_field_missing_count=0$' "$GOOD_INV"
grep -q '^typed_page_meta_field_remote_head=1$' "$GOOD_INV"
grep -q '^fastmem_unverified_offset_load_count=0$' "$GOOD_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --report "$FIXTURE_DIR/report.kv" \
  --format kv \
  >"$GOOD_CHECK"
grep -q '^summary=ok$' "$GOOD_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --report "$FIXTURE_DIR/bad_typed_page_meta_report.kv" \
  >"$BAD_INV"
grep -q '^typed_page_meta_handle=1$' "$BAD_INV"
grep -q '^typed_page_meta_field_remote_head=0$' "$BAD_INV"
grep -q '^typed_page_meta_required_field_missing_count=1$' "$BAD_INV"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --report "$FIXTURE_DIR/bad_typed_page_meta_report.kv" \
  --format kv \
  >"$BAD_CHECK"; then
  echo "[TEST/FAIL] fastmem-check accepted incomplete TypedPageMetaHandle fixture" >&2
  cat "$BAD_CHECK" >&2 || true
  exit 1
fi
grep -q '^failure_count=1$' "$BAD_CHECK"
grep -q '^failure_0_reason=typed_page_meta_required_field_missing_count$' "$BAD_CHECK"
grep -q '^summary=failed$' "$BAD_CHECK"

echo "[TEST/OK] fastmem_typed_page_meta"
