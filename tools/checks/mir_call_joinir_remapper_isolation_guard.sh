#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-joinir-remapper-isolation"
BUILDER="$ROOT_DIR/src/mir/builder.rs"
INVENTORY="$ROOT_DIR/src/mir/builder/mir_value_id_inventory.rs"
JOINIR="$ROOT_DIR/src/mir/builder/control_flow/joinir/mod.rs"
LIFECYCLE="$ROOT_DIR/src/mir/builder/emission/value_lifecycle.rs"
LIFECYCLE_DEFINITION="$ROOT_DIR/src/mir/builder/emission/value_lifecycle_definition.rs"
REMAP="$ROOT_DIR/src/mir/builder/joinir_id_remapper.rs"
MERGE="$ROOT_DIR/src/mir/builder/control_flow/joinir/merge"

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

require() {
  local file="$1"
  local token="$2"
  rg -F -q -- "$token" "$file" || fail "missing '$token' in ${file#$ROOT_DIR/}"
}

require_cfg_test_module() {
  local file="$1"
  local declaration="$2"
  awk -v declaration="$declaration" '
    $0 == declaration {
      if (previous != "#[cfg(test)]") {
        exit 1
      }
      found = 1
    }
    { previous = $0 }
    END {
      if (!found) {
        exit 1
      }
    }
  ' "$file" || fail "module is not directly gated by cfg(test): ${file#$ROOT_DIR/}"
}

for file in "$BUILDER" "$INVENTORY" "$JOINIR" "$LIFECYCLE" "$LIFECYCLE_DEFINITION" "$REMAP"; do
  [[ -f "$file" ]] || fail "missing owner ${file#$ROOT_DIR/}"
done
[[ -d "$MERGE" ]] || fail "missing reference-only merge surface"

require "$BUILDER" "mod mir_value_id_inventory;"
require "$BUILDER" "pub mod joinir_id_remapper;"
require "$INVENTORY" "pub(crate) struct MirValueIdInventory;"
require "$INVENTORY" "callee.for_each_value_operand"
require "$INVENTORY" "legacy_call_inventory_ignores_invalid_sentinel_but_keeps_real_func"
require "$LIFECYCLE" "MirValueIdInventory"
require "$LIFECYCLE_DEFINITION" "MirValueIdInventory"

require_cfg_test_module "$BUILDER" "pub mod joinir_id_remapper; // Reference-only JoinIR ID remapping (ValueId/BlockId translation)"
require_cfg_test_module "$JOINIR" "pub(in crate::mir::builder) mod merge; // Reference-only legacy JoinIR merger"

if rg -n --glob '*.rs' \
    --glob '!builder.rs' \
    --glob '!joinir_id_remapper.rs' \
    --glob '!**/control_flow/joinir/merge/**' \
    'JoinIrIdRemapper|joinir_id_remapper' "$ROOT_DIR/src/mir/builder"; then
  fail "legacy JoinIR remapper escaped its test/reference surface"
fi

if rg -n --glob '*.rs' 'joinir_id_remapper|JoinIrIdRemapper' "$LIFECYCLE" "$LIFECYCLE_DEFINITION"; then
  fail "active lifecycle collector still imports the legacy remapper"
fi

for file in "$INVENTORY" "$REMAP"; do
  lines=$(wc -l < "$file")
  (( lines < 800 )) || fail "${file#$ROOT_DIR/} reached the 800-line hard stop ($lines)"
done

echo "[$TAG] ok"
