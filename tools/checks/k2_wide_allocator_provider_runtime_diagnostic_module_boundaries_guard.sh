#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-runtime-diagnostic-module-boundaries"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-runtime-diagnostic-module-boundaries-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-154-M98B-ALLOCATOR-PROVIDER-RUNTIME-DIAGNOSTIC-MODULE-BOUNDARIES.md"
FACADE="src/runtime/allocator_provider_registry.rs"
COMMON="src/runtime/allocator_provider_registry_common.rs"
REGISTRY_SNAPSHOT="src/runtime/allocator_provider_registry_snapshot.rs"
SELECTION_DECISION="src/runtime/allocator_provider_selection_decision.rs"
PROOF_BUNDLE="src/runtime/allocator_provider_proof_bundle_consumption.rs"
ACTIVATION_SAFETY="src/runtime/allocator_provider_activation_safety.rs"
RUNTIME_MOD="src/runtime/mod.rs"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M98B allocator provider runtime diagnostic module boundaries"

fail() {
  echo "[$TAG] ERROR: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing file: $path"
}

require_text() {
  local file="$1"
  local needle="$2"
  rg -F -q "$needle" "$file" || fail "missing text in $file: $needle"
}

require_file "$SSOT"
require_file "$CARD"
require_file "$FACADE"
require_file "$COMMON"
require_file "$REGISTRY_SNAPSHOT"
require_file "$SELECTION_DECISION"
require_file "$PROOF_BUNDLE"
require_file "$ACTIVATION_SAFETY"
require_file "$RUNTIME_MOD"
require_file "$TASK_BREAKDOWN"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Runtime Diagnostic Module Boundaries (SSOT)"
require_text "$SSOT" 'allocator_provider_registry.rs` acting only as the stable facade'
require_text "$SSOT" "allocator_provider_registry_snapshot.rs"
require_text "$SSOT" "allocator_provider_selection_decision.rs"
require_text "$SSOT" "allocator_provider_proof_bundle_consumption.rs"
require_text "$SSOT" "allocator_provider_activation_safety.rs"
require_text "$SSOT" "allocator_provider_registry_common.rs"
require_text "$CARD" "293x-154 M98B Allocator Provider Runtime Diagnostic Module Boundaries"

for module in \
  "$COMMON" \
  "$REGISTRY_SNAPSHOT" \
  "$SELECTION_DECISION" \
  "$PROOF_BUNDLE" \
  "$ACTIVATION_SAFETY"; do
  require_text "$RUNTIME_MOD" "$(basename "$module" .rs)"
done

line_count="$(wc -l < "$FACADE")"
if (( line_count >= 1000 )); then
  fail "$FACADE must stay under 1000 lines; got $line_count"
fi

require_text "$FACADE" "Diagnostic-only allocator provider registry facade"
require_text "$FACADE" "pub use super::allocator_provider_registry_snapshot"
require_text "$FACADE" "pub use super::allocator_provider_selection_decision"
require_text "$FACADE" "pub use super::allocator_provider_proof_bundle_consumption"
require_text "$FACADE" "pub use super::allocator_provider_activation_safety"
require_text "$REGISTRY_SNAPSHOT" "validate_allocator_provider_registry_snapshot_from_text"
require_text "$SELECTION_DECISION" "validate_allocator_provider_selection_decision_from_text"
require_text "$PROOF_BUNDLE" "validate_allocator_provider_proof_bundle_consumption_from_text"
require_text "$ACTIVATION_SAFETY" "validate_allocator_provider_activation_safety_gate_from_text"
require_text "$COMMON" 'OWNER_PATH: &str = "src/runtime/allocator_provider_registry.rs"'
require_text "$COMMON" "EXPECTED_PROVIDER_IDS"
require_text "$TASK_BREAKDOWN" "M98B | runtime diagnostic module boundaries"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_runtime_diagnostic_module_boundaries_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_runtime_diagnostic_module_boundaries_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_runtime_diagnostic_module_boundaries_guard.sh"

cargo test -q registry_snapshot -- --nocapture
cargo test -q selection_decision -- --nocapture
cargo test -q proof_bundle_consumption -- --nocapture
cargo test -q activation_safety -- --nocapture

allocator_provider_forbid_activation_gate_open "$TAG"
allocator_provider_forbid_selection "$TAG"
allocator_provider_forbid_proof_consumption "$TAG"
allocator_provider_forbid_rollback_preparation "$TAG"
allocator_provider_forbid_hook_activation "$TAG"
allocator_provider_forbid_global_allocator "$TAG"
allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
