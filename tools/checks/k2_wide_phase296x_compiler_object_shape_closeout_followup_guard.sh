#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-833-COMPILER-OBJECT-SHAPE-CLOSEOUT-FOLLOWUP-001.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
CLOSEOUT_GUARD="tools/checks/k2_wide_phase296x_compiler_object_shape_closeout_guard.sh"
OBJECT_PLAN_SRC="src/object_storage_plan.rs"
LEGACY_PROOF_CHAIN="src/array_receiver_representation_source.rs"
LIVE_ARRAY_PROOF="src/mir/array_receiver_proof.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_compiler_object_shape_closeout_followup_guard.sh"

[[ -f "$CARD" ]] || { echo "[compiler-object-shape-closeout-followup] missing card: $CARD" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[compiler-object-shape-closeout-followup] missing ObjectStoragePlan SSOT: $OBJECT_SSOT" >&2; exit 1; }
[[ -f "$CLOSEOUT_GUARD" ]] || { echo "[compiler-object-shape-closeout-followup] missing closeout guard: $CLOSEOUT_GUARD" >&2; exit 1; }
[[ -f "$OBJECT_PLAN_SRC" ]] || { echo "[compiler-object-shape-closeout-followup] missing ObjectPlan source: $OBJECT_PLAN_SRC" >&2; exit 1; }
[[ -f "$LEGACY_PROOF_CHAIN" ]] || { echo "[compiler-object-shape-closeout-followup] legacy proof-chain module must remain for retire inventory: $LEGACY_PROOF_CHAIN" >&2; exit 1; }
[[ -f "$LIVE_ARRAY_PROOF" ]] || { echo "[compiler-object-shape-closeout-followup] live array receiver proof module missing: $LIVE_ARRAY_PROOF" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[compiler-object-shape-closeout-followup] card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[compiler-object-shape-closeout-followup] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[compiler-object-shape-closeout-followup] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-compiler-object-shape-closeout-followup-v0" \
  "source_evidence=296x-832,post-closeout-audit" \
  "object_storage_plan_ssot_enum_aligned_with_code=1" \
  "object_storage_plan_variant_count=7" \
  "flattened_nested_fields_variant_documented=1" \
  "closeout_guard_executes_subguards=1" \
  "closeout_subguard_count=7" \
  "backend_method_name_selfproof_investigation_required=1" \
  "proof_chain_retire_project_required=1" \
  "risky_code_change_count=0" \
  "product_default_changed=0" \
  "backend_lowering_changed=0" \
  "object_plan_execution_enabled=0" \
  "standalone_publication_plan_enabled=0" \
  "selected_next=MIMALLOC-FRESH-FRONT-SELECTION-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "FlattenedNestedFields {" "$OBJECT_SSOT" || {
  echo "[compiler-object-shape-closeout-followup] SSOT must document FlattenedNestedFields" >&2
  exit 1
}
grep -F -q "FlattenedNestedFields {" "$OBJECT_PLAN_SRC" || {
  echo "[compiler-object-shape-closeout-followup] source must still expose FlattenedNestedFields" >&2
  exit 1
}
grep -F -q 'bash "$script"' "$CLOSEOUT_GUARD" || {
  echo "[compiler-object-shape-closeout-followup] closeout guard must execute source sub-guards" >&2
  exit 1
}

for token in \
  "BACKEND-METHOD-NAME-PROOF-AUDIT-001" \
  "ARRAY-RECEIVER-RESIDENCE-PROOF-CHAIN-RETIRE-INVENTORY-001" \
  "do not drive-by rewrite flattened_nested_fields.py" \
  "do not delete src/array_receiver_representation_source.rs in this row" \
  "do not touch live src/mir/array_receiver_proof.rs"; do
  grep -F -q "$token" "$CARD" || {
    echo "[compiler-object-shape-closeout-followup] missing follow-up token: $token" >&2
    exit 1
  }
done

echo "[compiler-object-shape-closeout-followup] ok"
