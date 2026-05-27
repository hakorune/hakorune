#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mir-builder-field-property-receiver-facts-cleanup"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_144="docs/development/current/main/phases/phase-296x/296x-144-MIR-BUILDER-FIELD-PROPERTY-RECEIVER-FACTS-CLEANUP.md"
CARD_145="docs/development/current/main/phases/phase-296x/296x-145-MIR-BUILDER-POST-BOXSHAPE-CORRECTNESS-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mir_builder_field_property_receiver_facts_cleanup_guard.sh"
FACTS="src/mir/builder/field_facts.rs"
FIELDS="src/mir/builder/fields.rs"
PROPS="src/mir/builder/property_reads.rs"
BUILDER="src/mir/builder.rs"
APP_SINGLE_EVAL="apps/mir-single-eval-surface-sweep/main.hako"

echo "[$TAG] checking field/property receiver facts cleanup"

guard_require_files "$TAG" "$CARD_144" "$CARD_145" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SELF_SCRIPT" "$FACTS" "$FIELDS" "$PROPS" "$BUILDER" "$APP_SINGLE_EVAL"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_144" "row144 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_145" "row145 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=mir-builder-field-property-receiver-facts-cleanup-v0' "$CARD_144" "row144 must record output contract"
guard_expect_fixed_in_file "$TAG" 'fact_owner=src/mir/builder/field_facts.rs' "$CARD_144" "row144 must record fact owner"
guard_expect_fixed_in_file "$TAG" 'generic_cse_opened=0' "$CARD_144" "row144 must keep generic CSE closed"
guard_expect_fixed_in_file "$TAG" 'mod field_facts' "$BUILDER" "builder must include field_facts module"
guard_expect_fixed_in_file "$TAG" 'fn declared_field_type_for_value' "$FACTS" "field facts must own declared type lookup"
guard_expect_fixed_in_file "$TAG" 'fn publish_field_result_origin' "$FACTS" "field facts must publish result origin"
guard_expect_fixed_in_file "$TAG" 'fn resolve_property_getter_name' "$FACTS" "field facts must own property getter lookup"
guard_expect_fixed_in_file "$TAG" 'self.declared_field_type_for_value' "$FIELDS" "fields lowering must use fact helper"
guard_expect_fixed_in_file "$TAG" 'self.publish_field_result_origin' "$FIELDS" "fields lowering must publish through fact helper"
guard_expect_fixed_in_file "$TAG" 'self.resolve_property_getter_name' "$PROPS" "property reads must use fact helper"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-144-MIR-BUILDER-FIELD-PROPERTY-RECEIVER-FACTS-CLEANUP"' "$CURRENT_STATE" "current state latest card must advance to row144"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIR-BUILDER-POST-BOXSHAPE-CORRECTNESS-CLOSEOUT-296X-001"' "$CURRENT_STATE" "current state must select row145"
guard_expect_fixed_in_file "$TAG" '| 144 | `MIR-BUILDER-FIELD-PROPERTY-RECEIVER-FACTS-CLEANUP-296X-001` | Landed |' "$TASKBOARD" "taskboard row144 must be landed"
guard_expect_fixed_in_file "$TAG" '| 145 | `MIR-BUILDER-POST-BOXSHAPE-CORRECTNESS-CLOSEOUT-296X-001` | Current |' "$TASKBOARD" "taskboard row145 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

summary="$(target/release/hakorune "$APP_SINGLE_EVAL")"
printf '%s\n' "$summary" | grep -Fx 'summary=ok' >/dev/null || {
  echo "[$TAG] ERROR: single-eval surface sweep must end summary=ok" >&2
  printf '%s\n' "$summary" >&2
  exit 1
}

echo "[$TAG] ok"
