#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-safety-gate"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/allocator_provider_forbidden_patterns.sh"
source "$ROOT_DIR/tools/checks/lib/allocator_provider_activation_safety_gate_sections.sh"

SSOT="docs/development/current/main/design/allocator-provider-activation-safety-gate-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml"
ACTIVATION_ENTRY_SSOT="docs/development/current/main/design/allocator-provider-activation-entry-contract-ssot.md"
ACTIVATION_ENTRY_FIXTURE="docs/development/current/main/design/allocator-provider-activation-entry-contract-v0.toml"
READINESS_SSOT="docs/development/current/main/design/allocator-provider-readiness-preflight-ssot.md"
COMBINED_DRY_RUN_SSOT="docs/development/current/main/design/allocator-provider-combined-dry-run-ssot.md"
REGISTRY_SNAPSHOT_FIXTURE="docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml"
SELECTION_DECISION_FIXTURE="docs/development/current/main/design/allocator-provider-selection-decision-v0.toml"
PROOF_BUNDLE_FIXTURE="docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml"
ROLLBACK_PREFLIGHT_SSOT="docs/development/current/main/design/allocator-provider-rollback-preflight-ssot.md"
ROLLBACK_PREFLIGHT_FIXTURE="docs/development/current/main/design/allocator-provider-rollback-preflight-v0.toml"
HOOK_PLAN_FIXTURE="docs/development/current/main/design/allocator-hook-plan-v0.toml"
HOOK_ACTIVATION_PREFLIGHT_SSOT="docs/development/current/main/design/allocator-hook-activation-preflight-shape-ssot.md"
HOOK_ACTIVATION_PROOF_SSOT="docs/development/current/main/design/allocator-hook-activation-proof-ssot.md"
HOOK_ACTIVATION_PROOF_FIXTURE="docs/development/current/main/design/allocator-hook-activation-proof-v0.toml"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-133-M81-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-GATE.md"
INDEX="docs/tools/check-scripts-index.md"

echo "[$TAG] checking M81 allocator provider activation safety gate"

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

allocator_provider_activation_safety_gate_check_docs
allocator_provider_activation_safety_gate_check_fixture
allocator_provider_activation_safety_gate_check_forbidden

echo "[$TAG] ok"
