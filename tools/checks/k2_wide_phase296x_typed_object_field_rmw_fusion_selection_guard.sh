#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-221-TYPED-OBJECT-FIELD-RMW-FUSION-SELECTION.md"
TOOL="$ROOT_DIR/tools/allocator/mir_typed_object_field_rmw_fusion_selection.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row221_rmw_selection.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR_JSON="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local line="$2"
  if ! grep -Fqx "$line" "$file"; then
    echo "[row221-rmw-selection] missing line in $file: $line" >&2
    exit 1
  fi
}

for line in \
  "output_contract=typed-object-field-rmw-fusion-selection-v0" \
  "input_contract=mir-typed-field-direct-op-selected-method-feasibility-v0" \
  "selected_method=HakoAllocPageModel.acquire_usize/1" \
  "selected_owner=typed_object_exact_slot_rmw_fusion" \
  "rmw_candidate_count=5" \
  "planned_net_helper_call_delta=5" \
  "planned_net_helper_call_delta_positive=1" \
  "runtime_storage_owner_preserved=1" \
  "helper_free_direct_op_rejected=1" \
  "selected_next=typed_object_field_rmw_fusion_keeper" \
  "by_name_special_case=0" \
  "winner_claim=0" \
  "summary=ok"; do
  require_line "$DOC" "$line"
done

if [ ! -x "$ROOT_DIR/target/release/hakorune" ]; then
  cargo build --release --bin hakorune >/tmp/hakorune_row221_hakorune_build.log
fi

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR_JSON" \
    "$APP" >/tmp/hakorune_row221_mir_emit.log

"$TOOL" --mir-json "$MIR_JSON" > "$REPORT"

for line in \
  "output_contract=typed-object-field-rmw-fusion-selection-v0" \
  "input_contract=mir-typed-field-direct-op-selected-method-feasibility-v0" \
  "selected_method=HakoAllocPageModel.acquire_usize/1" \
  "selected_owner=typed_object_exact_slot_rmw_fusion" \
  "rmw_candidate_count=5" \
  "rmw_candidate_usize_count=5" \
  "planned_erased_get_set_helper_calls=10" \
  "planned_added_fused_helper_calls=5" \
  "planned_net_helper_call_delta=5" \
  "planned_net_helper_call_delta_positive=1" \
  "runtime_storage_owner_preserved=1" \
  "helper_free_direct_op_rejected=1" \
  "generic_residence_open=0" \
  "source_rewrite=0" \
  "candidate_0_field=HakoAllocPageModel.reject_count" \
  "candidate_3_field=HakoAllocPageModel.alloc_count" \
  "candidate_4_field=HakoAllocPageModel.requested_bytes" \
  "selected_next=typed_object_field_rmw_fusion_keeper" \
  "by_name_special_case=0" \
  "winner_claim=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "summary=ok"; do
  require_line "$REPORT" "$line"
done

cat "$REPORT"
