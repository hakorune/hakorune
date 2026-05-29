#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-302-TYPED-OBJECT-RESIDENT-SCALAR-SELECTED-METHOD-PLAN.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-301-TYPED-OBJECT-RESIDENT-SCALAR-GUARD-SURFACE.md"
TOOL="$ROOT_DIR/tools/allocator/typed_object_resident_scalar_selected_method_plan.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row302_resident_plan.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR_JSON="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row302-typed-object-resident-plan] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-resident-scalar-selected-method-plan-v0"
require_line "$DOC" "input_contract=typed-object-resident-scalar-guard-surface-v0"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "candidate_representation=ResidentScalar"
require_line "$DOC" "eligible_field_get_count=13"
require_line "$DOC" "eligible_field_set_count=8"
require_line "$DOC" "planned_erased_helper_ops=21"
require_line "$DOC" "planned_materialization_ops_added=0"
require_line "$DOC" "planned_net_helper_delta=21"
require_line "$DOC" "dynamic_planned_net_helper_delta=11010048"
require_line "$DOC" "planned_net_helper_delta_positive=1"
require_line "$DOC" "resident_field_key_count=11"
require_line "$DOC" "unknown_receiver_count=0"
require_line "$DOC" "unknown_field_plan_count=0"
require_line "$DOC" "unsupported_storage_count=0"
require_line "$DOC" "weak_field_count=0"
require_line "$DOC" "resident_field_0=HakoAllocPageModel.reject_count.usize"
require_line "$DOC" "selected_next=typed_object_resident_scalar_implementation_owner_selection"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

if [ ! -x "$ROOT_DIR/target/release/hakorune" ]; then
  cargo build --release --bin hakorune >/tmp/hakorune_row302_hakorune_build.log
fi

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR_JSON" \
    "$APP" >/tmp/hakorune_row302_mir_emit.log

"$TOOL" --mir-json "$MIR_JSON" --out "$REPORT" >/tmp/hakorune_row302_resident_plan.log

require_line "$REPORT" "output_contract=typed-object-resident-scalar-selected-method-plan-v0"
require_line "$REPORT" "input_contract=typed-object-resident-scalar-guard-surface-v0"
require_line "$REPORT" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "eligible_field_get_count=13"
require_line "$REPORT" "eligible_field_set_count=8"
require_line "$REPORT" "planned_erased_helper_ops=21"
require_line "$REPORT" "planned_materialization_ops_added=0"
require_line "$REPORT" "planned_net_helper_delta=21"
require_line "$REPORT" "dynamic_planned_net_helper_delta=11010048"
require_line "$REPORT" "planned_net_helper_delta_positive=1"
require_line "$REPORT" "resident_field_key_count=11"
require_line "$REPORT" "unknown_receiver_count=0"
require_line "$REPORT" "unknown_field_plan_count=0"
require_line "$REPORT" "unsupported_storage_count=0"
require_line "$REPORT" "weak_field_count=0"
require_line "$REPORT" "resident_field_0=HakoAllocPageModel.reject_count.usize"
require_line "$REPORT" "selected_next=typed_object_resident_scalar_implementation_owner_selection"
require_line "$REPORT" "summary=ok"

echo "[row302-typed-object-resident-plan] ok"
