#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-safety-gate"
cd "$ROOT_DIR"

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
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
FUTURE_REGISTRY_FILE="src/runtime/allocator_provider_registry.rs"

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

require_file "$SSOT"
require_file "$FIXTURE"
require_file "$ACTIVATION_ENTRY_SSOT"
require_file "$ACTIVATION_ENTRY_FIXTURE"
require_file "$READINESS_SSOT"
require_file "$COMBINED_DRY_RUN_SSOT"
require_file "$REGISTRY_SNAPSHOT_FIXTURE"
require_file "$SELECTION_DECISION_FIXTURE"
require_file "$PROOF_BUNDLE_FIXTURE"
require_file "$ROLLBACK_PREFLIGHT_SSOT"
require_file "$ROLLBACK_PREFLIGHT_FIXTURE"
require_file "$HOOK_PLAN_FIXTURE"
require_file "$HOOK_ACTIVATION_PREFLIGHT_SSOT"
require_file "$HOOK_ACTIVATION_PROOF_SSOT"
require_file "$HOOK_ACTIVATION_PROOF_FIXTURE"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Activation Safety Gate (SSOT)"
require_text "$SSOT" "allocator-provider-activation-safety-gate-v0.toml"
require_text "$SSOT" "activation_safety_gate = \"inactive\""
require_text "$SSOT" "safety_status = \"reserved_gate_closed\""
require_text "$SSOT" "activation_gate_open = false"
require_text "$SSOT" "would_open_activation_gate = false"
require_text "$SSOT" "would_activate = false"
require_text "$SSOT" "activation = \"future_row_required\""
require_text "$SSOT" "[allocator-provider/activation-safety-gate-missing]"
require_text "$SSOT" "[allocator-provider/activation-safety-entry-missing]"
require_text "$SSOT" "[allocator-provider/activation-safety-readiness-missing]"
require_text "$SSOT" "[allocator-provider/activation-safety-combined-dry-run-missing]"
require_text "$SSOT" "[allocator-provider/activation-safety-registry-missing]"
require_text "$SSOT" "[allocator-provider/activation-safety-selection-missing]"
require_text "$SSOT" "[allocator-provider/activation-safety-proof-bundle-missing]"
require_text "$SSOT" "[allocator-provider/activation-safety-rollback-missing]"
require_text "$SSOT" "[allocator-provider/activation-safety-hook-plan-missing]"
require_text "$SSOT" "[allocator-provider/activation-safety-preflight-missing]"
require_text "$SSOT" "[allocator-provider/activation-safety-proof-missing]"
require_text "$SSOT" "[allocator-provider/activation-safety-target-missing]"
require_text "$SSOT" "[allocator-provider/activation-safety-blocked]"
require_text "$ACTIVATION_ENTRY_SSOT" "provider proof bundle consumption"
require_text "$ACTIVATION_ENTRY_FIXTURE" 'rollback_behavior_named'
require_text "$READINESS_SSOT" "would_select_provider = false"
require_text "$COMBINED_DRY_RUN_SSOT" "would_activate=false"
require_text "$REGISTRY_SNAPSHOT_FIXTURE" 'provider_id = "native_mimalloc"'
require_text "$SELECTION_DECISION_FIXTURE" 'selected_provider_id = "none_reserved"'
require_text "$PROOF_BUNDLE_FIXTURE" 'proof_bundle_consumption = "inactive"'
require_text "$ROLLBACK_PREFLIGHT_SSOT" "rollback_preflight = \"inactive\""
require_text "$ROLLBACK_PREFLIGHT_FIXTURE" 'rollback_status = "reserved_no_rollback"'
require_text "$HOOK_PLAN_FIXTURE" 'activation = "future_row_required"'
require_text "$HOOK_ACTIVATION_PREFLIGHT_SSOT" "AllocatorHookActivationPreflightReport"
require_text "$HOOK_ACTIVATION_PROOF_SSOT" "rollback_condition_named"
require_text "$HOOK_ACTIVATION_PROOF_FIXTURE" 'rollback_condition_named'
require_text "$TASK_BREAKDOWN" "M81 | activation safety gate contract"
require_text "$TASK_BREAKDOWN" "The next safe row is M82 activation safety gate diagnostic owner."
require_text "$TASKBOARD" '| `M81 allocator provider activation safety gate` | `live-docs` |'
require_text "$TASKBOARD" '104. `M81 allocator provider activation safety gate`'
require_text "$CARD" "293x-133 M81 Allocator Provider Activation Safety Gate"
require_text "$CARD" "activation_safety_gate = \"inactive\""
require_text "$CARD" "safety_status = \"reserved_gate_closed\""
require_text "$CARD" "activation_gate_open = false"
require_text "$CARD" "would_open_activation_gate = false"
require_text "$PHASE_README" '`293x-133`'
require_text "$REAL_APP_TASKBOARD" '[x] `293x-133` M81 allocator provider activation safety gate'
require_text "$CURRENT_STATE" 'latest_card = "293x-133-M81-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-GATE"'
require_text "$CURRENT_STATE" 'latest_card_path = "docs/development/current/main/phases/phase-293x/293x-133-M81-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-GATE.md"'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh"

python3 - <<'PY' "$FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(f"[k2-wide-allocator-provider-activation-safety-gate][fail] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("schema_version") != "allocator_provider_activation_safety_gate_v0":
    fail("schema_version must be allocator_provider_activation_safety_gate_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")
if data.get("safety_gate_owner") != "src/runtime/allocator_provider_registry.rs":
    fail("safety_gate_owner must name the future runtime owner")
expected_inputs = {
    "activation_entry_input": "allocator_provider_activation_entry_contract",
    "provider_readiness_input": "allocator_provider_readiness_preflight_report",
    "combined_dry_run_input": "allocator_provider_combined_dry_run_report",
    "registry_snapshot_input": "allocator_provider_registry_snapshot_report",
    "selection_decision_input": "allocator_provider_selection_decision_report",
    "proof_bundle_input": "allocator_provider_proof_bundle_consumption_report",
    "rollback_preflight_input": "allocator_provider_rollback_preflight_report",
    "hook_plan_input": "allocator_hook_plan_report",
    "hook_activation_preflight_input": "allocator_hook_activation_preflight_report",
    "activation_proof_input": "allocator_hook_activation_proof_v0",
    "activation_target_source": "caller_provided_diagnostic_target",
}
for key, expected in expected_inputs.items():
    if data.get(key) != expected:
        fail(f"{key} must be {expected}")
if data.get("activation_target_provider_id") != "native_mimalloc":
    fail("activation_target_provider_id must be native_mimalloc for the reserved fixture")
if data.get("rollback_target_provider_id") != "native_mimalloc":
    fail("rollback_target_provider_id must be native_mimalloc for the reserved fixture")
if data.get("current_provider_id") != "none_reserved":
    fail("current_provider_id must be none_reserved in M81")
if data.get("selected_provider_id") != "none_reserved":
    fail("selected_provider_id must be none_reserved in M81")
if data.get("safety_gate_policy") != "explicit_activation_evidence_bundle_required_reserved":
    fail("safety_gate_policy must be explicit_activation_evidence_bundle_required_reserved")
if data.get("provider_selection") != "inactive":
    fail("provider_selection must be inactive")
if data.get("proof_bundle_consumption") != "inactive":
    fail("proof_bundle_consumption must be inactive")
if data.get("rollback_preflight") != "inactive":
    fail("rollback_preflight must be inactive")
if data.get("hook_activation") != "inactive":
    fail("hook_activation must be inactive")
if data.get("activation_safety_gate") != "inactive":
    fail("activation_safety_gate must be inactive")
if data.get("safety_status") != "reserved_gate_closed":
    fail("safety_status must be reserved_gate_closed")
for key in [
    "activation_gate_open",
    "would_build_registry",
    "would_select_provider",
    "would_consume_proof_bundle",
    "would_prepare_rollback",
    "would_open_activation_gate",
    "would_activate_hook",
    "would_activate",
]:
    if data.get(key) is not False:
        fail(f"{key} must be false")
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")

expected_diagnostics = {
    "diagnostic": "[allocator-provider/activation-safety-gate-missing]",
    "missing_entry_diagnostic": "[allocator-provider/activation-safety-entry-missing]",
    "missing_readiness_diagnostic": "[allocator-provider/activation-safety-readiness-missing]",
    "missing_combined_dry_run_diagnostic": "[allocator-provider/activation-safety-combined-dry-run-missing]",
    "missing_registry_diagnostic": "[allocator-provider/activation-safety-registry-missing]",
    "missing_selection_diagnostic": "[allocator-provider/activation-safety-selection-missing]",
    "missing_proof_bundle_diagnostic": "[allocator-provider/activation-safety-proof-bundle-missing]",
    "missing_rollback_diagnostic": "[allocator-provider/activation-safety-rollback-missing]",
    "missing_hook_plan_diagnostic": "[allocator-provider/activation-safety-hook-plan-missing]",
    "missing_preflight_diagnostic": "[allocator-provider/activation-safety-preflight-missing]",
    "missing_activation_proof_diagnostic": "[allocator-provider/activation-safety-proof-missing]",
    "missing_activation_target_diagnostic": "[allocator-provider/activation-safety-target-missing]",
    "activation_blocked_diagnostic": "[allocator-provider/activation-safety-blocked]",
}
for key, expected in expected_diagnostics.items():
    if data.get(key) != expected:
        fail(f"{key} must be {expected}")

if data.get("required_operations") != ["alloc", "realloc", "free"]:
    fail("required_operations must lock the reserved allocator request")
if data.get("activation_target_operations") != ["alloc", "realloc", "free"]:
    fail("activation_target_operations must lock the reserved activation target")

expected_ids = [
    "native_system_malloc",
    "native_mimalloc",
    "hako_model_allocator",
    "debug_guarded_allocator",
]
if data.get("candidate_provider_ids") != expected_ids:
    fail("candidate_provider_ids must preserve registry snapshot order")

required_facts = [
    "activation_entry_contract_ready",
    "provider_readiness_preflight_ready",
    "combined_dry_run_ready",
    "registry_snapshot_ready",
    "selection_decision_ready",
    "selected_provider_id_absent",
    "proof_bundle_ready",
    "rollback_preflight_ready",
    "hook_plan_ready",
    "hook_activation_preflight_ready",
    "activation_proof_ready",
    "rollback_target_explicit",
    "activation_target_provider_id_explicit",
    "safety_gate_policy_named",
    "activation_gate_closed",
    "fail_fast_activation_safety_diagnostic_named",
    "no_hidden_environment_toggle",
    "no_implicit_manifest_discovery",
    "no_implicit_proof_discovery",
    "no_app_or_facade_name_matching",
    "no_inc_name_matching",
    "no_runtime_registry_implementation",
    "no_provider_selection_implementation",
    "no_proof_consumption_implementation",
    "no_rollback_preparation_implementation",
    "no_hook_activation_implementation",
    "no_global_allocator_attribute",
    "no_global_alloc_trait",
    "no_process_allocator_replacement",
    "no_route_widening",
]
facts = data.get("reserved_activation_safety_facts")
if not isinstance(facts, list):
    fail("reserved_activation_safety_facts must be a list")
for fact in required_facts:
    if fact not in facts:
        fail(f"missing activation safety fact: {fact}")

expected_safety_inputs = {
    "activation_entry": "[allocator-provider/activation-safety-entry-missing]",
    "provider_readiness": "[allocator-provider/activation-safety-readiness-missing]",
    "combined_dry_run": "[allocator-provider/activation-safety-combined-dry-run-missing]",
    "registry_snapshot": "[allocator-provider/activation-safety-registry-missing]",
    "selection_decision": "[allocator-provider/activation-safety-selection-missing]",
    "proof_bundle": "[allocator-provider/activation-safety-proof-bundle-missing]",
    "rollback_preflight": "[allocator-provider/activation-safety-rollback-missing]",
    "hook_plan": "[allocator-provider/activation-safety-hook-plan-missing]",
    "hook_activation_preflight": "[allocator-provider/activation-safety-preflight-missing]",
    "activation_proof": "[allocator-provider/activation-safety-proof-missing]",
    "activation_target": "[allocator-provider/activation-safety-target-missing]",
}
inputs = data.get("safety_inputs")
if not isinstance(inputs, list) or len(inputs) != len(expected_safety_inputs):
    fail("safety_inputs must list the eleven reserved input diagnostics")
seen = set()
for item in inputs:
    name = item.get("name")
    seen.add(name)
    if name not in expected_safety_inputs:
        fail(f"unexpected safety input: {name}")
    if item.get("required") is not True:
        fail(f"safety input {name} must be required")
    if item.get("missing_diagnostic") != expected_safety_inputs[name]:
        fail(f"safety input {name} diagnostic mismatch")
if seen != set(expected_safety_inputs):
    fail("safety_inputs must cover all reserved diagnostics")
PY

if [[ -e "$FUTURE_REGISTRY_FILE" ]]; then
  fail "future registry owner file must remain absent in M81: $FUTURE_REGISTRY_FILE"
fi

if rg -n 'ActivationSafetyGate|activation_safety_gate|allocator_provider_activation_safety|activation_gate_open|open_activation_gate|would_open_activation_gate' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".safety_impl 2>&1; then
  cat /tmp/"$TAG".safety_impl >&2
  rm -f /tmp/"$TAG".safety_impl
  fail "activation safety gate implementation must remain docs/fixture-only in M81"
fi
rm -f /tmp/"$TAG".safety_impl

if rg -n 'ProviderSelectionRequest|ProviderSelectionDecision|select_allocator_provider|allocator_provider_select|allocator_provider_selection_env|NYASH_ALLOCATOR_PROVIDER' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".provider_selection 2>&1; then
  cat /tmp/"$TAG".provider_selection >&2
  rm -f /tmp/"$TAG".provider_selection
  fail "provider selection implementation must stay absent in M81"
fi
rm -f /tmp/"$TAG".provider_selection

if rg -n 'proof_bundle_consumed|consume_allocator_provider_proof|allocator_provider_proof_bundle_consume|ProofBundleConsumption' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".proof_consumption 2>&1; then
  cat /tmp/"$TAG".proof_consumption >&2
  rm -f /tmp/"$TAG".proof_consumption
  fail "proof consumption implementation must stay absent in M81"
fi
rm -f /tmp/"$TAG".proof_consumption

if rg -n 'RollbackPreflight|rollback_preflight|allocator_provider_rollback|rollback_target|prepare_rollback|would_prepare_rollback' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".rollback_impl 2>&1; then
  cat /tmp/"$TAG".rollback_impl >&2
  rm -f /tmp/"$TAG".rollback_impl
  fail "rollback preparation implementation must stay absent in M81"
fi
rm -f /tmp/"$TAG".rollback_impl

if rg -n 'allocator_hook_activate|activate_allocator|install_allocator_hook|replace_allocator' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".hook_activation 2>&1; then
  cat /tmp/"$TAG".hook_activation >&2
  rm -f /tmp/"$TAG".hook_activation
  fail "hook activation/process allocator replacement must stay absent in M81"
fi
rm -f /tmp/"$TAG".hook_activation

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M81"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider activation safety behavior"
fi
rm -f /tmp/"$TAG".runner

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan|allocator_hook_activate|activate_allocator|debug_guarded_allocator|hako_model_allocator|native_mimalloc|native_system_malloc' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  fail "allocator provider/hook/facade/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc

echo "[$TAG] ok"
