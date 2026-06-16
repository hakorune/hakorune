#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="object-storage-plan-module-split"
CARD="docs/development/current/main/phases/phase-296x/296x-989-OBJECT-STORAGE-PLAN-MODULE-SPLIT-001.md"
INDEX="docs/tools/check-scripts-index.md"
FACADE="src/object_storage_plan.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_storage_plan_module_split_guard.sh"

for file in "$CARD" "$INDEX" "$FACADE"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
done

for file in \
  src/object_storage_plan/ids.rs \
  src/object_storage_plan/storage.rs \
  src/object_storage_plan/publication.rs \
  src/object_storage_plan/alias.rs \
  src/object_storage_plan/fastpath.rs \
  src/object_storage_plan/inventory.rs \
  src/object_storage_plan/report.rs \
  src/object_storage_plan/tests.rs; do
  [[ -f "$file" ]] || { echo "[$TAG] missing split module: $file" >&2; exit 1; }
done

grep -q '^Status: Landed$' "$CARD" || {
  echo "[$TAG] card must be Landed" >&2
  exit 1
}

grep -F -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[$TAG] check index missing guard entry" >&2
  exit 1
}

facade_lines="$(wc -l < "$FACADE")"
if (( facade_lines > 80 )); then
  echo "[$TAG] facade too large: $facade_lines lines" >&2
  exit 1
fi

for token in \
  "pub use alias::*;" \
  "pub use fastpath::*;" \
  "pub use ids::*;" \
  "pub use inventory::*;" \
  "pub use publication::*;" \
  "pub use report::*;" \
  "pub use storage::*;"; do
  grep -F -q "$token" "$FACADE" || {
    echo "[$TAG] facade missing re-export: $token" >&2
    exit 1
  }
done

require_card_line() {
  local expected="$1"
  if ! grep -F -x -q "$expected" "$CARD"; then
    echo "[$TAG] missing card line: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-object-storage-plan-module-split-v0" \
  "row_kind=boxshape_refactor" \
  "behavior_changed=0" \
  "public_api_reexport_preserved=1" \
  "facade_line_count_max=80" \
  "object_storage_plan_execution_enabled=0" \
  "backend_lowering_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=OBJECT-STORAGE-PLAN-VOCAB-AUDIT-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

cargo test --lib object_storage_plan -- --nocapture >/tmp/"$TAG".test.out
cargo check --lib >/tmp/"$TAG".check.out

echo "[$TAG] ok"
