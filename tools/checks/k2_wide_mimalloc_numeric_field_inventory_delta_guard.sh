#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-numeric-field-inventory-delta"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

NUMERIC="lang/src/hako_alloc/memory/NUMERIC_FIELDS.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-197-M185-HAKO-ALLOC-FIELD-INVENTORY-DELTA.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_numeric_field_inventory_delta_guard.sh"

echo "[$TAG] checking M185 numeric field inventory delta"

guard_require_files \
  "$TAG" \
  "$NUMERIC" \
  "$PLAN" \
  "$CARD" \
  "$TASKBOARD" \
  "$PHASE_README" \
  "$INDEX"

expected="$(sed -n 's/^Current stored numeric field count: \([0-9][0-9]*\).$/\1/p' "$NUMERIC" | head -n 1)"
actual="$(rg -n '^[[:space:]]+[A-Za-z_][A-Za-z0-9_]*:[[:space:]]+(i64|usize)' lang/src/hako_alloc/memory -g '*.hako' | rg -v 'usize_field_probe_box.hako|allocator_metadata_records.hako' | wc -l | tr -d '[:space:]')"

if [[ "$expected" != "$actual" ]]; then
  echo "[$TAG] ERROR: NUMERIC_FIELDS.md count is $expected but source count is $actual" >&2
  exit 1
fi

guard_expect_in_file "$TAG" 'Current stored numeric field count: 220\.' "$NUMERIC" "M185/C205d must update the current numeric field count"
guard_expect_in_file "$TAG" 'allocator metadata `record` declarations are also excluded' "$NUMERIC" "record declaration fields must be excluded from live stored-field count"
guard_expect_in_file "$TAG" 'aligned_small_meta_store_box.hako' "$NUMERIC" "C205c metadata store counter must be inventoried"
guard_expect_in_file "$TAG" 'huge_page_meta_store_box.hako' "$NUMERIC" "C205d metadata store counters must be inventoried"
guard_expect_in_file "$TAG" 'M185 Grouped Current Inventory' "$NUMERIC" "M185 must add a grouped current inventory"
guard_expect_in_file "$TAG" 'secure_free_list_policy_box.hako` \| `HakoAllocSecureFreeListPolicy` \| none' "$NUMERIC" "M184 policy must be recorded as storing no numeric fields"
guard_expect_in_file "$TAG" 'page_map_release_invariant_box.hako' "$NUMERIC" "M173 observer fields must be inventoried"
guard_expect_in_file "$TAG" 'page_map_realloc_alloc_copy_release_box.hako' "$NUMERIC" "M175 fields must be inventoried"
guard_expect_in_file "$TAG" 'page_map_realloc_failure_contract_box.hako' "$NUMERIC" "M176 fields must be inventoried"
guard_expect_in_file "$TAG" 'page_map_aligned_small_path_box.hako' "$NUMERIC" "M178 fields must be inventoried"
guard_expect_in_file "$TAG" 'huge_threshold_router_box.hako' "$NUMERIC" "M179 fields must be inventoried"
guard_expect_in_file "$TAG" 'huge_page_model_box.hako' "$NUMERIC" "M180 fields must be inventoried"
guard_expect_in_file "$TAG" 'huge_release_seam_box.hako' "$NUMERIC" "M181 fields must be inventoried"
guard_expect_in_file "$TAG" 'secure_free_list_diagnostics_box.hako' "$NUMERIC" "M183 fields must be inventoried"
guard_expect_in_file "$TAG" 'Stored negative sentinel:' "$NUMERIC" "M185 must refresh sentinel notes"
guard_expect_in_file "$TAG" 'HakoAllocSecureFreeListPolicy.invalid_next\(\)' "$NUMERIC" "M184 non-stored invalid sentinel must be documented"
guard_expect_in_file "$TAG" 'M185 hako_alloc field inventory delta` \| Complete' "$PLAN" "plan must mark M185 complete"
guard_expect_in_file "$TAG" '293x-197 M185 hako_alloc Field Inventory Delta' "$CARD" "missing M185 card"
guard_expect_in_file "$TAG" '293x-197' "$TASKBOARD" "taskboard must list M185"
guard_expect_in_file "$TAG" 'M185 hako_alloc field inventory delta landed' "$PHASE_README" "phase README must retain the M185 summary"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M185 guard"

echo "[$TAG] ok: $actual stored production numeric fields inventoried"
