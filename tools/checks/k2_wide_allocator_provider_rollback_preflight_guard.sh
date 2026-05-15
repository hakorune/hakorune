#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-rollback-preflight"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-rollback-preflight-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-rollback-preflight-v0.toml"
ACTIVATION_ENTRY_SSOT="docs/development/current/main/design/allocator-provider-activation-entry-contract-ssot.md"
ACTIVATION_ENTRY_FIXTURE="docs/development/current/main/design/allocator-provider-activation-entry-contract-v0.toml"
PROOF_BUNDLE_SSOT="docs/development/current/main/design/allocator-provider-proof-bundle-consumption-ssot.md"
PROOF_BUNDLE_FIXTURE="docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml"
SELECTION_DECISION_FIXTURE="docs/development/current/main/design/allocator-provider-selection-decision-v0.toml"
REGISTRY_SNAPSHOT_FIXTURE="docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml"
HOOK_PLAN_SSOT="docs/development/current/main/design/allocator-hook-plan-v0-ssot.md"
HOOK_PLAN_FIXTURE="docs/development/current/main/design/allocator-hook-plan-v0.toml"
HOOK_ACTIVATION_PREFLIGHT_SSOT="docs/development/current/main/design/allocator-hook-activation-preflight-shape-ssot.md"
HOOK_ACTIVATION_PROOF_SSOT="docs/development/current/main/design/allocator-hook-activation-proof-ssot.md"
HOOK_ACTIVATION_PROOF_FIXTURE="docs/development/current/main/design/allocator-hook-activation-proof-v0.toml"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-132-M80-ALLOCATOR-PROVIDER-ROLLBACK-PREFLIGHT.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M80 allocator provider rollback preflight"

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
require_file "$FIXTURE"
require_file "$ACTIVATION_ENTRY_SSOT"
require_file "$ACTIVATION_ENTRY_FIXTURE"
require_file "$PROOF_BUNDLE_SSOT"
require_file "$PROOF_BUNDLE_FIXTURE"
require_file "$SELECTION_DECISION_FIXTURE"
require_file "$REGISTRY_SNAPSHOT_FIXTURE"
require_file "$HOOK_PLAN_SSOT"
require_file "$HOOK_PLAN_FIXTURE"
require_file "$HOOK_ACTIVATION_PREFLIGHT_SSOT"
require_file "$HOOK_ACTIVATION_PROOF_SSOT"
require_file "$HOOK_ACTIVATION_PROOF_FIXTURE"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Rollback Preflight (SSOT)"
require_text "$SSOT" "allocator-provider-rollback-preflight-v0.toml"
require_text "$SSOT" "rollback_preflight = \"inactive\""
require_text "$SSOT" "rollback_status = \"reserved_no_rollback\""
require_text "$SSOT" "would_prepare_rollback = false"
require_text "$SSOT" "would_activate = false"
require_text "$SSOT" "activation = \"future_row_required\""
require_text "$SSOT" "[allocator-provider/rollback-preflight-missing]"
require_text "$SSOT" "[allocator-provider/rollback-input-missing]"
require_text "$SSOT" "[allocator-provider/rollback-snapshot-missing]"
require_text "$SSOT" "[allocator-provider/rollback-selection-missing]"
require_text "$SSOT" "[allocator-provider/rollback-proof-bundle-missing]"
require_text "$SSOT" "[allocator-provider/rollback-hook-plan-missing]"
require_text "$SSOT" "[allocator-provider/rollback-activation-preflight-missing]"
require_text "$SSOT" "[allocator-provider/rollback-activation-proof-missing]"
require_text "$SSOT" "[allocator-provider/rollback-target-missing]"
require_text "$SSOT" "[allocator-provider/rollback-activation-blocked]"
require_text "$ACTIVATION_ENTRY_SSOT" "M80 | rollback preflight contract"
require_text "$ACTIVATION_ENTRY_FIXTURE" 'rollback_behavior_named'
require_text "$PROOF_BUNDLE_SSOT" "M80 may consume this fixture only as an explicit diagnostic input"
require_text "$PROOF_BUNDLE_SSOT" "M79"
require_text "$PROOF_BUNDLE_FIXTURE" 'proof_bundle_consumption = "inactive"'
require_text "$PROOF_BUNDLE_FIXTURE" 'would_consume_proof_bundle = false'
require_text "$SELECTION_DECISION_FIXTURE" 'selected_provider_id = "none_reserved"'
require_text "$REGISTRY_SNAPSHOT_FIXTURE" 'provider_id = "native_mimalloc"'
require_text "$HOOK_PLAN_SSOT" "Allocator HookPlan v0 (SSOT)"
require_text "$HOOK_PLAN_FIXTURE" 'activation = "future_row_required"'
require_text "$HOOK_ACTIVATION_PREFLIGHT_SSOT" "AllocatorHookActivationPreflightReport"
require_text "$HOOK_ACTIVATION_PROOF_SSOT" "rollback_condition_named"
require_text "$HOOK_ACTIVATION_PROOF_FIXTURE" 'rollback_condition_named'
require_text "$TASK_BREAKDOWN" "M80 | rollback preflight contract"
require_text "$TASKBOARD" '| `M80 allocator provider rollback preflight` | `live-docs` |'
require_text "$TASKBOARD" '103. `M80 allocator provider rollback preflight`'
require_text "$CARD" "293x-132 M80 Allocator Provider Rollback Preflight"
require_text "$CARD" "rollback_preflight = \"inactive\""
require_text "$CARD" "rollback_status = \"reserved_no_rollback\""
require_text "$CARD" "would_prepare_rollback = false"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_rollback_preflight_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_rollback_preflight_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_rollback_preflight_guard.sh"

python3 - <<'PY' "$FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(f"[k2-wide-allocator-provider-rollback-preflight][fail] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("schema_version") != "allocator_provider_rollback_preflight_v0":
    fail("schema_version must be allocator_provider_rollback_preflight_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")
if data.get("rollback_owner") != "src/runtime/allocator_provider_registry.rs":
    fail("rollback_owner must name the future runtime owner")
if data.get("preflight_input") != "caller_provided_diagnostic_preflight":
    fail("preflight_input must be caller-provided")
if data.get("activation_entry_input") != "allocator_provider_activation_entry_contract":
    fail("activation_entry_input must be allocator_provider_activation_entry_contract")
if data.get("registry_snapshot_input") != "allocator_provider_registry_snapshot_report":
    fail("registry_snapshot_input must be allocator_provider_registry_snapshot_report")
if data.get("selection_decision_input") != "allocator_provider_selection_decision_report":
    fail("selection_decision_input must be allocator_provider_selection_decision_report")
if data.get("proof_bundle_input") != "allocator_provider_proof_bundle_consumption_report":
    fail("proof_bundle_input must be allocator_provider_proof_bundle_consumption_report")
if data.get("hook_plan_input") != "allocator_hook_plan_report":
    fail("hook_plan_input must be allocator_hook_plan_report")
if data.get("hook_activation_preflight_input") != "allocator_hook_activation_preflight_report":
    fail("hook_activation_preflight_input must be allocator_hook_activation_preflight_report")
if data.get("activation_proof_input") != "allocator_hook_activation_proof_v0":
    fail("activation_proof_input must be allocator_hook_activation_proof_v0")
if data.get("rollback_target_source") != "caller_provided_diagnostic_target":
    fail("rollback_target_source must be caller-provided")
if data.get("rollback_target_provider_id") != "native_mimalloc":
    fail("rollback_target_provider_id must be native_mimalloc for the reserved fixture")
if data.get("current_provider_id") != "none_reserved":
    fail("current_provider_id must be none_reserved in M80")
if data.get("selected_provider_id") != "none_reserved":
    fail("selected_provider_id must be none_reserved in M80")
if data.get("provider_selection") != "inactive":
    fail("provider_selection must be inactive")
if data.get("proof_bundle_consumption") != "inactive":
    fail("proof_bundle_consumption must be inactive")
if data.get("hook_activation") != "inactive":
    fail("hook_activation must be inactive")
if data.get("rollback_preflight") != "inactive":
    fail("rollback_preflight must be inactive")
if data.get("rollback_status") != "reserved_no_rollback":
    fail("rollback_status must be reserved_no_rollback")
if data.get("would_build_registry") is not False:
    fail("would_build_registry must be false")
if data.get("would_select_provider") is not False:
    fail("would_select_provider must be false")
if data.get("would_consume_proof_bundle") is not False:
    fail("would_consume_proof_bundle must be false")
if data.get("would_prepare_rollback") is not False:
    fail("would_prepare_rollback must be false")
if data.get("would_activate_hook") is not False:
    fail("would_activate_hook must be false")
if data.get("would_activate") is not False:
    fail("would_activate must be false")
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")

expected_diagnostics = {
    "diagnostic": "[allocator-provider/rollback-preflight-missing]",
    "missing_preflight_diagnostic": "[allocator-provider/rollback-preflight-missing]",
    "missing_input_diagnostic": "[allocator-provider/rollback-input-missing]",
    "missing_snapshot_diagnostic": "[allocator-provider/rollback-snapshot-missing]",
    "missing_selection_diagnostic": "[allocator-provider/rollback-selection-missing]",
    "missing_proof_bundle_diagnostic": "[allocator-provider/rollback-proof-bundle-missing]",
    "missing_hook_plan_diagnostic": "[allocator-provider/rollback-hook-plan-missing]",
    "missing_activation_preflight_diagnostic": "[allocator-provider/rollback-activation-preflight-missing]",
    "missing_activation_proof_diagnostic": "[allocator-provider/rollback-activation-proof-missing]",
    "missing_rollback_target_diagnostic": "[allocator-provider/rollback-target-missing]",
    "activation_blocked_diagnostic": "[allocator-provider/rollback-activation-blocked]",
}
for key, expected in expected_diagnostics.items():
    if data.get(key) != expected:
        fail(f"{key} must be {expected}")

if data.get("required_operations") != ["alloc", "realloc", "free"]:
    fail("required_operations must lock the reserved allocator request")
if data.get("rollback_target_operations") != ["alloc", "realloc", "free"]:
    fail("rollback_target_operations must lock the reserved rollback target")

expected_ids = [
    "native_system_malloc",
    "native_mimalloc",
    "hako_model_allocator",
    "debug_guarded_allocator",
]
if data.get("candidate_provider_ids") != expected_ids:
    fail("candidate_provider_ids must preserve registry snapshot order")

required_facts = [
    "preflight_input_caller_provided",
    "activation_entry_contract_ready",
    "registry_snapshot_ready",
    "selection_decision_ready",
    "proof_bundle_ready",
    "hook_plan_ready",
    "hook_activation_preflight_ready",
    "activation_proof_ready",
    "rollback_target_explicit",
    "rollback_target_provider_id_explicit",
    "previous_allocator_state_snapshot_required",
    "rollback_status_reserved",
    "rollback_preflight_inactive",
    "fail_fast_rollback_diagnostic_named",
    "activation_blocked_until_future_row",
    "no_hidden_environment_toggle",
    "no_implicit_manifest_discovery",
    "no_implicit_proof_discovery",
    "no_app_or_facade_name_matching",
    "no_inc_name_matching",
    "no_runtime_registry_implementation",
    "no_provider_selection_implementation",
    "no_proof_consumption_implementation",
    "no_hook_activation_implementation",
    "no_global_allocator_attribute",
    "no_global_alloc_trait",
    "no_process_allocator_replacement",
]
facts = data.get("reserved_rollback_facts")
if not isinstance(facts, list):
    fail("reserved_rollback_facts must be a list")
for fact in required_facts:
    if fact not in facts:
        fail(f"missing rollback preflight fact: {fact}")

expected_inputs = {
    "preflight": "[allocator-provider/rollback-preflight-missing]",
    "activation_entry": "[allocator-provider/rollback-input-missing]",
    "input": "[allocator-provider/rollback-input-missing]",
    "snapshot": "[allocator-provider/rollback-snapshot-missing]",
    "selection": "[allocator-provider/rollback-selection-missing]",
    "proof_bundle": "[allocator-provider/rollback-proof-bundle-missing]",
    "hook_plan": "[allocator-provider/rollback-hook-plan-missing]",
    "activation_preflight": "[allocator-provider/rollback-activation-preflight-missing]",
    "activation_proof": "[allocator-provider/rollback-activation-proof-missing]",
    "rollback_target": "[allocator-provider/rollback-target-missing]",
}
inputs = data.get("rollback_inputs")
if not isinstance(inputs, list) or len(inputs) != len(expected_inputs):
    fail("rollback_inputs must list the seven reserved input diagnostics")
seen = set()
for item in inputs:
    name = item.get("name")
    seen.add(name)
    if name not in expected_inputs:
        fail(f"unexpected rollback input: {name}")
    if item.get("required") is not True:
        fail(f"rollback input {name} must be required")
    if item.get("missing_diagnostic") != expected_inputs[name]:
        fail(f"rollback input {name} diagnostic mismatch")
if seen != set(expected_inputs):
    fail("rollback_inputs must cover all reserved diagnostics")
PY


allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider rollback behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
