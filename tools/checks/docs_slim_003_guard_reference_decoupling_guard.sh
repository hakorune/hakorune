#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-003-guard-reference-decoupling"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/checks/lib/phase_card_paths.sh"

CARD_FILENAME="293x-410-DOCS-SLIM-003-GUARD-REFERENCE-DECOUPLING.md"
CARD="$(guard_require_phase293x_card "$TAG" "$CARD_FILENAME")"
POLICY="docs/development/current/main/design/current-docs-update-policy-ssot.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
MANIFEST="docs/development/current/main/phases/phase-293x/archive/cards/phase-293x-card-archive-manifest.md"
CHECK_INDEX="docs/tools/check-scripts-index.md"
HELPER="tools/checks/lib/phase_card_paths.sh"
SELF_SCRIPT="tools/checks/docs_slim_003_guard_reference_decoupling_guard.sh"
DOCS_SLIM_002_GUARD="tools/checks/docs_slim_002_archive_manifest_guard.sh"

stale_pin_guards=(
  "tools/checks/manifest_runner_pilot_guard.sh"
  "tools/checks/k2_wide_hako_alloc_lifecycle_stats_observer_surface_guard.sh"
  "tools/checks/k2_wide_hako_alloc_decommit_recommit_reuse_exe_hardening_guard.sh"
  "tools/checks/k2_wide_hako_alloc_purge_candidate_policy_inventory_guard.sh"
  "tools/checks/k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh"
  "tools/checks/k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh"
  "tools/checks/k2_wide_hako_alloc_options_inventory_guard.sh"
  "tools/checks/k2_wide_hako_alloc_thread_heap_owner_inventory_guard.sh"
  "tools/checks/k2_wide_loop_range_parser_capsule_guard.sh"
  "tools/checks/k2_wide_static_const_table_eval_guard.sh"
)

echo "[$TAG] running DOCS-SLIM-003 guard reference decoupling guard"

guard_require_command "$TAG" rg
guard_require_files \
  "$TAG" \
  "$CARD" \
  "$POLICY" \
  "$ARCHIVE_POLICY" \
  "$MANIFEST" \
  "$CHECK_INDEX" \
  "$HELPER" \
  "$SELF_SCRIPT" \
  "$DOCS_SLIM_002_GUARD" \
  "${stale_pin_guards[@]}"
guard_require_exec_files "$TAG" "$HELPER" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "DOCS-SLIM-003" "$CARD" "DOCS-SLIM-003 card must exist"
guard_expect_in_file "$TAG" "Do not move numbered cards in this row" "$CARD" "card must keep no-move stop-line"
guard_expect_in_file "$TAG" "Past row guards must not pin" "$POLICY" "update policy must forbid stale current pointer pins"
guard_expect_in_file "$TAG" "Third Slimming Phase" "$ARCHIVE_POLICY" "archive policy must record DOCS-SLIM-003"
guard_expect_in_file "$TAG" "historical snapshot" "$MANIFEST" "archive manifest must describe DOCS-SLIM-002 counts as historical"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$CHECK_INDEX" "check script index must list DOCS-SLIM-003 guard"
guard_expect_in_file "$TAG" "phase293x_card_path" "$HELPER" "phase card helper must expose resolver"

latest_prefix='latest_card = "'
latest_phase='293x-'
blocker_prefix='current_blocker_token = '
quote='"'

if rg -n "${latest_prefix}${latest_phase}|${blocker_prefix}${quote}" tools/checks --glob '!docs_slim_003_guard_reference_decoupling_guard.sh' >/tmp/"$TAG".current_pin 2>&1; then
  echo "[$TAG] ERROR: check guards must not pin CURRENT_STATE latest_card/current_blocker_token" >&2
  cat /tmp/"$TAG".current_pin >&2
  rm -f /tmp/"$TAG".current_pin
  exit 1
fi
rm -f /tmp/"$TAG".current_pin

for script in "${stale_pin_guards[@]}"; do
  if rg -n 'CURRENT_STATE|latest_card|current_blocker_token' "$script" >/tmp/"$TAG".stale_pin 2>&1; then
    echo "[$TAG] ERROR: stale current pointer dependency remains in $script" >&2
    cat /tmp/"$TAG".stale_pin >&2
    rm -f /tmp/"$TAG".stale_pin
    exit 1
  fi
done
rm -f /tmp/"$TAG".stale_pin

dynamic_latest_pin="${latest_prefix}${latest_phase}"
if rg -n "count_root_cards|count_range|direct_guard_refs|unique_card_refs|${dynamic_latest_pin}" "$DOCS_SLIM_002_GUARD" >/tmp/"$TAG".docs_slim_002_dynamic 2>&1; then
  echo "[$TAG] ERROR: DOCS-SLIM-002 guard must not pin evolving root counts, reference counts, or latest-card fields" >&2
  cat /tmp/"$TAG".docs_slim_002_dynamic >&2
  rm -f /tmp/"$TAG".docs_slim_002_dynamic
  exit 1
fi
rm -f /tmp/"$TAG".docs_slim_002_dynamic

resolved="$(phase293x_card_path "$CARD_FILENAME")"
if [[ "$resolved" != "$CARD" ]]; then
  guard_fail "$TAG" "phase-card resolver returned unexpected path: $resolved"
fi

echo "[$TAG] ok"
