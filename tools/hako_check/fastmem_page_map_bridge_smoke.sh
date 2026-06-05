#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE_DIR="$ROOT/tools/hako_check/tests/fastmem_capability_inventory"
REPORT_FIXTURE="$ROOT/tools/hako_check/tests/replacement_front_report/report.kv"
GOOD_REPORT="$(mktemp "${TMPDIR:-/tmp}/hako_pagemap_report_good.XXXXXX")"
GOOD_INV="$(mktemp "${TMPDIR:-/tmp}/hako_pagemap_inv_good.XXXXXX")"
GOOD_CHECK="$(mktemp "${TMPDIR:-/tmp}/hako_pagemap_check_good.XXXXXX")"
BAD_INV="$(mktemp "${TMPDIR:-/tmp}/hako_pagemap_inv_bad.XXXXXX")"
BAD_CHECK="$(mktemp "${TMPDIR:-/tmp}/hako_pagemap_check_bad.XXXXXX")"
trap 'rm -f "$GOOD_REPORT" "$GOOD_INV" "$GOOD_CHECK" "$BAD_INV" "$BAD_CHECK"' EXIT

bash "$ROOT/tools/hako_check.sh" replacement-front-report \
  --report "$REPORT_FIXTURE" \
  >"$GOOD_REPORT"
grep -q '^replacement_front_page_bins_lookup_route=page_from_ptr_bridge$' "$GOOD_REPORT"
grep -q '^replacement_front_page_from_ptr_route=side_table_direct$' "$GOOD_REPORT"
grep -q '^free_path_page_lookup_route=page_map_bridge$' "$GOOD_REPORT"
grep -q '^free_path_page_lookup_range_scan_count=0$' "$GOOD_REPORT"
grep -q '^page_map_bridge_kind=flat_side_table$' "$GOOD_REPORT"
grep -q '^page_map_bridge_type_abi_hot_lookup_count=0$' "$GOOD_REPORT"
grep -q '^page_map_bridge_provider_abi_hot_dispatch_count=0$' "$GOOD_REPORT"
grep -q '^page_map_bridge_benchmark_front_pilot=1$' "$GOOD_REPORT"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --report "$FIXTURE_DIR/report.kv" \
  >"$GOOD_INV"
grep -q '^free_path_page_lookup_route=page_map_bridge$' "$GOOD_INV"
grep -q '^free_path_page_lookup_range_scan_count=0$' "$GOOD_INV"
grep -q '^page_map_bridge_kind=flat_side_table$' "$GOOD_INV"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --report "$FIXTURE_DIR/report.kv" \
  --format kv \
  >"$GOOD_CHECK"
grep -q '^summary=ok$' "$GOOD_CHECK"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --report "$FIXTURE_DIR/range_scan_report.kv" \
  >"$BAD_INV"
grep -q '^free_path_page_lookup_route=range_scan$' "$BAD_INV"
grep -q '^free_path_page_lookup_range_scan_count=1000$' "$BAD_INV"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --report "$FIXTURE_DIR/range_scan_report.kv" \
  --format kv \
  >"$BAD_CHECK"; then
  echo "[TEST/FAIL] fastmem-check accepted range_scan PageMapBridge fixture" >&2
  cat "$BAD_CHECK" >&2 || true
  exit 1
fi
grep -q '^failure_count=2$' "$BAD_CHECK"
grep -q '^failure_0_reason=free_path_page_lookup_range_scan_count$' "$BAD_CHECK"
grep -q '^failure_1_reason=free_path_page_lookup_route$' "$BAD_CHECK"
grep -q '^summary=failed$' "$BAD_CHECK"

echo "[TEST/OK] fastmem_page_map_bridge"
