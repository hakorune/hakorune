#!/usr/bin/env bash
# Shared checks for the allocator provider activation safety gate.
#
# Callers must define:
# - fail()
# - require_file()
# - require_text()
# before invoking these helpers.

allocator_provider_activation_safety_gate_require_texts() {
  local file="$1"
  shift
  local needle
  for needle in "$@"; do
    require_text "$file" "$needle"
  done
}

allocator_provider_activation_safety_gate_check_docs() {
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
  require_file "$INDEX"

  allocator_provider_activation_safety_gate_require_texts "$SSOT" \
    "Allocator Provider Activation Safety Gate (SSOT)" \
    "allocator-provider-activation-safety-gate-v0.toml" \
    "activation_safety_gate = \"inactive\"" \
    "safety_status = \"reserved_gate_closed\"" \
    "activation_gate_open = false" \
    "would_open_activation_gate = false" \
    "would_activate = false" \
    "activation = \"future_row_required\"" \
    "[allocator-provider/activation-safety-gate-missing]" \
    "[allocator-provider/activation-safety-entry-missing]" \
    "[allocator-provider/activation-safety-readiness-missing]" \
    "[allocator-provider/activation-safety-combined-dry-run-missing]" \
    "[allocator-provider/activation-safety-registry-missing]" \
    "[allocator-provider/activation-safety-selection-missing]" \
    "[allocator-provider/activation-safety-proof-bundle-missing]" \
    "[allocator-provider/activation-safety-rollback-missing]" \
    "[allocator-provider/activation-safety-hook-plan-missing]" \
    "[allocator-provider/activation-safety-preflight-missing]" \
    "[allocator-provider/activation-safety-proof-missing]" \
    "[allocator-provider/activation-safety-target-missing]" \
    "[allocator-provider/activation-safety-blocked]"

  allocator_provider_activation_safety_gate_require_texts "$ACTIVATION_ENTRY_SSOT" \
    "provider proof bundle consumption"
  allocator_provider_activation_safety_gate_require_texts "$ACTIVATION_ENTRY_FIXTURE" \
    "rollback_behavior_named"
  allocator_provider_activation_safety_gate_require_texts "$READINESS_SSOT" \
    "would_select_provider = false"
  allocator_provider_activation_safety_gate_require_texts "$COMBINED_DRY_RUN_SSOT" \
    "would_activate=false"
  allocator_provider_activation_safety_gate_require_texts "$REGISTRY_SNAPSHOT_FIXTURE" \
    'provider_id = "native_mimalloc"'
  allocator_provider_activation_safety_gate_require_texts "$SELECTION_DECISION_FIXTURE" \
    'selected_provider_id = "none_reserved"'
  allocator_provider_activation_safety_gate_require_texts "$PROOF_BUNDLE_FIXTURE" \
    'proof_bundle_consumption = "inactive"'
  allocator_provider_activation_safety_gate_require_texts "$ROLLBACK_PREFLIGHT_SSOT" \
    "rollback_preflight = \"inactive\""
  allocator_provider_activation_safety_gate_require_texts "$ROLLBACK_PREFLIGHT_FIXTURE" \
    'rollback_status = "reserved_no_rollback"'
  allocator_provider_activation_safety_gate_require_texts "$HOOK_PLAN_FIXTURE" \
    'activation = "future_row_required"'
  allocator_provider_activation_safety_gate_require_texts "$HOOK_ACTIVATION_PREFLIGHT_SSOT" \
    "AllocatorHookActivationPreflightReport"
  allocator_provider_activation_safety_gate_require_texts "$HOOK_ACTIVATION_PROOF_SSOT" \
    "rollback_condition_named"
  allocator_provider_activation_safety_gate_require_texts "$HOOK_ACTIVATION_PROOF_FIXTURE" \
    "rollback_condition_named"
  allocator_provider_activation_safety_gate_require_texts "$TASK_BREAKDOWN" \
    "M81 | activation safety gate contract"
  allocator_provider_activation_safety_gate_require_texts "$TASKBOARD" \
    '| `M81 allocator provider activation safety gate` | `live-docs` |' \
    '104. `M81 allocator provider activation safety gate`'
  allocator_provider_activation_safety_gate_require_texts "$CARD" \
    "293x-133 M81 Allocator Provider Activation Safety Gate" \
    "activation_safety_gate = \"inactive\"" \
    "safety_status = \"reserved_gate_closed\"" \
    "activation_gate_open = false" \
    "would_open_activation_gate = false"
  allocator_provider_activation_safety_gate_require_texts "$INDEX" \
    "tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh"
}

allocator_provider_activation_safety_gate_check_fixture() {
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
PY
}

allocator_provider_activation_safety_gate_check_forbidden() {
  allocator_provider_forbid_activation_gate_open "$TAG"
  allocator_provider_forbid_selection "$TAG"
  allocator_provider_forbid_proof_consumption "$TAG"
  allocator_provider_forbid_rollback_preparation "$TAG"
  allocator_provider_forbid_hook_activation "$TAG"
  allocator_provider_forbid_global_allocator "$TAG"

  if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
    cat /tmp/"$TAG".runner >&2
    rm -f /tmp/"$TAG".runner
    fail "runner must not own allocator provider activation safety behavior"
  fi
  rm -f /tmp/"$TAG".runner

  allocator_provider_forbid_inc_matchers "$TAG"
}
