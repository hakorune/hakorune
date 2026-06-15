#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-734-AGG-STORAGE-PLAN-000.md"
MODULE="src/aggregate_storage_plan.rs"
LIB="src/lib.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_agg_storage_plan_guard.sh"

[[ -f "$CARD" ]] || { echo "[agg-storage-plan] missing card: $CARD" >&2; exit 1; }
[[ -f "$MODULE" ]] || { echo "[agg-storage-plan] missing module: $MODULE" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[agg-storage-plan] row734 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[agg-storage-plan] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[agg-storage-plan] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-aggregate-storage-plan-v0" \
  "record_box_surface_model=two_surface_one_substrate" \
  "aggregate_storage_plan_vocabulary_defined=1" \
  "aggregate_storage_plan_execution_enabled=0" \
  "aggregate_subject_record_enabled=1" \
  "aggregate_subject_enum_payload_enabled=1" \
  "aggregate_subject_tuple_payload_enabled=1" \
  "aggregate_subject_closure_env_enabled=1" \
  "object_storage_plan_shared_substrate=1" \
  "mirbuilder_representation_owner=0" \
  "product_default_changed=0" \
  "selected_next=AGG-OBJECT-STORAGE-BRIDGE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "pub mod aggregate_storage_plan;" "$LIB" || {
  echo "[agg-storage-plan] src/lib.rs must export aggregate_storage_plan" >&2
  exit 1
}
grep -F -q "pub enum AggregateSubjectKind" "$MODULE" || { echo "[agg-storage-plan] missing AggregateSubjectKind" >&2; exit 1; }
grep -F -q "pub enum AggregateStoragePlan" "$MODULE" || { echo "[agg-storage-plan] missing AggregateStoragePlan" >&2; exit 1; }
grep -F -q "aggregate_storage_plan_execution_enabled" "$MODULE" || {
  echo "[agg-storage-plan] missing execution-disabled report field" >&2
  exit 1
}
grep -F -q "does not collapse the source-level \`record\` / \`box\`" "$MODULE" || {
  echo "[agg-storage-plan] module must document source split preservation" >&2
  exit 1
}

cargo test -q aggregate_storage_plan --lib

echo "[agg-storage-plan] ok"
