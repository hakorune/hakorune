#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-decision-surface-proposal"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-activation-decision-surface-proposal-ssot.md"
PREVIOUS_SSOT="docs/development/current/main/design/allocator-provider-activation-safety-closeout-inventory-ssot.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-138-M86-ALLOCATOR-PROVIDER-ACTIVATION-DECISION-SURFACE-PROPOSAL.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
PREVIOUS_GUARD="tools/checks/k2_wide_allocator_provider_activation_safety_closeout_guard.sh"
RUNTIME_OWNER="src/runtime/allocator_provider_registry.rs"
CLI_ARGS="src/cli/args.rs"

echo "[$TAG] checking M86 allocator provider activation decision surface proposal"

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
require_file "$PREVIOUS_SSOT"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"
require_file "$PREVIOUS_GUARD"
require_file "$RUNTIME_OWNER"
require_file "$CLI_ARGS"

require_text "$SSOT" "Allocator Provider Activation Decision Surface Proposal (SSOT)"
require_text "$SSOT" "M86 is proposal-only"
require_text "$SSOT" "hakorune --allocator-provider-activation-decision <ACTIVATION_DECISION_TOML>"
require_text "$SSOT" 'surface_version = "allocator_provider_activation_decision_v0"'
require_text "$SSOT" "activation_decision_surface_status=proposal_only"
require_text "$SSOT" "activation_decision_allowed=false"
require_text "$SSOT" "would_select_provider=false"
require_text "$SSOT" "would_consume_proof=false"
require_text "$SSOT" "would_prepare_rollback=false"
require_text "$SSOT" "would_open_activation_gate=false"
require_text "$SSOT" "would_install_hook=false"
require_text "$SSOT" "would_replace_process_allocator=false"
require_text "$SSOT" "would_activate=false"
require_text "$SSOT" "The next safe row is M87 activation decision fixture contract."
require_text "$TASK_BREAKDOWN" "M86 | activation decision surface proposal"
require_text "$TASK_BREAKDOWN" "safe row is M87 activation decision fixture contract."
require_text "$TASKBOARD" '| `M86 allocator provider activation decision surface proposal` | `live-docs` |'
require_text "$TASKBOARD" '109. `M86 allocator provider activation decision surface proposal`'
require_text "$CARD" "293x-138 M86 Allocator Provider Activation Decision Surface Proposal"
require_text "$PHASE_README" '`293x-138`'
require_text "$REAL_APP_TASKBOARD" '[x] `293x-138` M86 allocator provider activation decision surface proposal'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_decision_surface_proposal_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_activation_decision_surface_proposal_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_activation_decision_surface_proposal_guard.sh"

if rg -n 'require_text "\$CURRENT_STATE".*latest_card|latest_card = |latest_card_path =' "$PREVIOUS_GUARD" >/tmp/"$TAG".past_guard_pin 2>&1; then
  cat /tmp/"$TAG".past_guard_pin >&2
  rm -f /tmp/"$TAG".past_guard_pin
  fail "M85 guard must not pin CURRENT_STATE latest-card pointers after M86"
fi
rm -f /tmp/"$TAG".past_guard_pin

if rg -n -e '--allocator-provider-activation-decision|activation_decision|ActivationDecision' src -g '*.rs' >/tmp/"$TAG".src 2>&1; then
  cat /tmp/"$TAG".src >&2
  rm -f /tmp/"$TAG".src
  fail "M86 is docs-first only and must not add activation decision runtime or CLI code"
fi
rm -f /tmp/"$TAG".src

if rg -n 'NYASH_ALLOCATOR_PROVIDER|HAKO_ALLOCATOR_PROVIDER|ALLOCATOR_PROVIDER_' "$CLI_ARGS" "$RUNTIME_OWNER" >/tmp/"$TAG".env 2>&1; then
  cat /tmp/"$TAG".env >&2
  rm -f /tmp/"$TAG".env
  fail "activation decision proposal must not add hidden environment toggles"
fi
rm -f /tmp/"$TAG".env

allocator_provider_forbid_activation_gate_open "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider activation decision proposal behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
