#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="guard-manifest-inventory-guard"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

MANIFEST="tools/checks/guard_rows.toml"
INVENTORY="tools/checks/guard_manifest_inventory.py"
DESIGN="docs/development/current/main/design/guard-manifest-migration-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-647-GUARD-MANIFEST-012-BATCH-MIGRATION-INVENTORY.md"
INDEX="docs/tools/check-scripts-index.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$MANIFEST" "$INVENTORY" "$DESIGN" "$CARD" "$INDEX"
guard_require_exec_files "$TAG" "$0" "$INVENTORY"

guard_expect_in_file "$TAG" "Batch Migration Inventory" "$DESIGN" \
  "guard manifest SSOT must describe batch migration inventory"
guard_expect_in_file "$TAG" "guard_manifest_inventory.py" "$DESIGN" \
  "guard manifest SSOT must name inventory owner"
guard_expect_in_file "$TAG" "guard_manifest_inventory_guard.sh" "$INDEX" \
  "check index must list the inventory guard"
guard_expect_in_file "$TAG" "guard-manifest-inventory" "$MANIFEST" \
  "guard rows manifest must register the inventory guard"

out="$(mktemp "/tmp/${TAG}.XXXXXX")"
python3 "$INVENTORY" \
  --root "$ROOT_DIR" \
  --min-guard-rows 35 \
  --min-impl-files 25 \
  --min-public-k2-wide 300 \
  --require-hako-alloc-closeout-covered \
  >"$out"
cat "$out"

guard_expect_in_file "$TAG" "non_manifest_hako_alloc_closeout_wrappers=0" "$out" \
  "hako_alloc closeout wrappers must be manifest-backed"
guard_expect_in_file "$TAG" "missing_manifest_hako_alloc_closeout_wrappers=0" "$out" \
  "manifest hako_alloc closeout wrappers must exist"

rm -f "$out"
echo "[$TAG] ok"
