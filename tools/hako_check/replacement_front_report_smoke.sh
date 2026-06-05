#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE_DIR="$ROOT/tools/hako_check/tests/replacement_front_report"
OUT="$(mktemp "${TMPDIR:-/tmp}/hako_replacement_front_report.XXXXXX")"
trap 'rm -f "$OUT"' EXIT

bash "$ROOT/tools/hako_check.sh" replacement-front-report \
  --report "$FIXTURE_DIR/report.kv" \
  --baseline-skip-report "$FIXTURE_DIR/skip_report.kv" \
  >"$OUT"

grep -q '^output_contract=hako-check-replacement-front-report-v0$' "$OUT"
grep -q '^benchmark_front_class=replacement_front_c_shim$' "$OUT"
grep -q '^measured_hot_path_owner=generated_c_replacement_front$' "$OUT"
grep -q '^api_boundary_gap_suspect=0$' "$OUT"
grep -q '^remote_free_workload=0$' "$OUT"
grep -q '^likely_next_owner=free_path_page_lookup$' "$OUT"
grep -q '^replacement_front_page_bins_lookup_route=page_from_ptr_bridge$' "$OUT"
grep -q '^replacement_front_page_from_ptr_route=side_table_direct$' "$OUT"
grep -q '^free_path_page_lookup_route=page_map_bridge$' "$OUT"
grep -q '^free_path_page_lookup_range_scan_count=0$' "$OUT"
grep -q '^page_map_bridge_kind=flat_side_table$' "$OUT"
grep -q '^page_map_bridge_benchmark_front_pilot=1$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_v0=1$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_non_activating=1$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_report_only=1$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_route=replacement_front_benchmark_to_product_ldpreload_descriptor$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_source_truth=hako_alloc.size_class_box$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_shape_ok=1$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_safety_ok=1$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_coverage_ok=1$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_preflight_ok=1$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_no_type_abi_hot_lookup=1$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_no_provider_dispatch=1$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_no_global_lock_hot_path=1$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_no_range_scan_hot_path=1$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_no_host_passthrough=1$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_evidence_ready=1$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_activation_ready=0$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_block_reason=activation_row_required$' "$OUT"
grep -q '^replacement_front_product_shaped_bridge_missing=product_gate_open,activation_row$' "$OUT"
grep -q '^skip_hot_counter_gap_class=low$' "$OUT"
grep -q '^summary=ok$' "$OUT"

echo "[TEST/OK] replacement_front_report"
