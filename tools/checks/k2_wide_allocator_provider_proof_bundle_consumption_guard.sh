#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-proof-bundle-consumption"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-provider-proof-bundle-consumption-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml"
SELECTION_DECISION_SSOT="docs/development/current/main/design/allocator-provider-selection-decision-ssot.md"
SELECTION_DECISION_FIXTURE="docs/development/current/main/design/allocator-provider-selection-decision-v0.toml"
REGISTRY_SNAPSHOT_FIXTURE="docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml"
NATIVE_SYSTEM_PROOF="docs/development/current/main/design/allocator-provider-native-system-proof-v0.toml"
NATIVE_MIMALLOC_PROOF="docs/development/current/main/design/allocator-provider-native-mimalloc-proof-v0.toml"
HAKO_MODEL_PROOF="docs/development/current/main/design/allocator-provider-hako-model-proof-v0.toml"
DEBUG_GUARDED_PROOF="docs/development/current/main/design/allocator-provider-debug-guarded-proof-v0.toml"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-131-M79-ALLOCATOR-PROVIDER-PROOF-BUNDLE-CONSUMPTION.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
FUTURE_REGISTRY_FILE="src/runtime/allocator_provider_registry.rs"

echo "[$TAG] checking M79 allocator provider proof bundle consumption"

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
require_file "$SELECTION_DECISION_SSOT"
require_file "$SELECTION_DECISION_FIXTURE"
require_file "$REGISTRY_SNAPSHOT_FIXTURE"
require_file "$NATIVE_SYSTEM_PROOF"
require_file "$NATIVE_MIMALLOC_PROOF"
require_file "$HAKO_MODEL_PROOF"
require_file "$DEBUG_GUARDED_PROOF"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Proof Bundle Consumption (SSOT)"
require_text "$SSOT" "allocator-provider-proof-bundle-consumption-v0.toml"
require_text "$SSOT" "proof_bundle_source = \"caller_provided_diagnostic_bundle\""
require_text "$SSOT" "consumption_status = \"reserved_no_consumption\""
require_text "$SSOT" "proof_bundle_consumption = \"inactive\""
require_text "$SSOT" "selected_provider_id = \"none_reserved\""
require_text "$SSOT" "proof_bundle_consumed = false"
require_text "$SSOT" "would_consume_proof_bundle = false"
require_text "$SSOT" "[allocator-provider/proof-bundle-consumption-missing]"
require_text "$SSOT" "[allocator-provider/proof-bundle-registry-missing]"
require_text "$SSOT" "[allocator-provider/proof-bundle-selection-missing]"
require_text "$SSOT" "[allocator-provider/proof-bundle-provider-proof-missing]"
require_text "$SSOT" "[allocator-provider/proof-bundle-provider-mismatch]"
require_text "$SSOT" "[allocator-provider/proof-bundle-capability-missing]"
require_text "$SSOT" "[allocator-provider/proof-bundle-activation-blocked]"
require_text "$SELECTION_DECISION_SSOT" "M79 may consume this reserved decision only as a diagnostic input"
require_text "$SELECTION_DECISION_FIXTURE" 'selected_provider_id = "none_reserved"'
require_text "$REGISTRY_SNAPSHOT_FIXTURE" 'provider_id = "native_mimalloc"'
require_text "$NATIVE_SYSTEM_PROOF" 'schema_version = "allocator_provider_native_system_proof_v0"'
require_text "$NATIVE_MIMALLOC_PROOF" 'schema_version = "allocator_provider_native_mimalloc_proof_v0"'
require_text "$HAKO_MODEL_PROOF" 'schema_version = "allocator_provider_hako_model_proof_v0"'
require_text "$DEBUG_GUARDED_PROOF" 'schema_version = "allocator_provider_debug_guarded_proof_v0"'
require_text "$TASK_BREAKDOWN" "M79 | provider proof bundle consumption"
require_text "$TASKBOARD" '| `M79 allocator provider proof bundle consumption` | `live-docs` |'
require_text "$TASKBOARD" '102. `M79 allocator provider proof bundle consumption`'
require_text "$CARD" "293x-131 M79 Allocator Provider Proof Bundle Consumption"
require_text "$CARD" "proof_bundle_consumption = \"inactive\""
require_text "$CARD" "would_consume_proof_bundle = false"
require_text "$PHASE_README" '`293x-131`'
require_text "$REAL_APP_TASKBOARD" '[x] `293x-131` M79 allocator provider proof bundle consumption'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_guard.sh"

python3 - <<'PY' "$FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(f"[k2-wide-allocator-provider-proof-bundle-consumption][fail] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("schema_version") != "allocator_provider_proof_bundle_consumption_v0":
    fail("schema_version must be allocator_provider_proof_bundle_consumption_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")
if data.get("consumption_owner") != "src/runtime/allocator_provider_registry.rs":
    fail("consumption_owner must name the future runtime owner")
if data.get("registry_snapshot_input") != "allocator_provider_registry_snapshot_report":
    fail("registry_snapshot_input must be allocator_provider_registry_snapshot_report")
if data.get("selection_decision_input") != "allocator_provider_selection_decision_report":
    fail("selection_decision_input must be allocator_provider_selection_decision_report")
if data.get("proof_bundle_source") != "caller_provided_diagnostic_bundle":
    fail("proof_bundle_source must be caller-provided")
if data.get("proof_bundle_policy") != "explicit_provider_proof_bundle_required_reserved":
    fail("proof_bundle_policy must be explicit_provider_proof_bundle_required_reserved")
if data.get("consumption_status") != "reserved_no_consumption":
    fail("consumption_status must be reserved_no_consumption")
if data.get("proof_bundle_consumption") != "inactive":
    fail("proof_bundle_consumption must be inactive")
if data.get("provider_selection") != "inactive":
    fail("provider_selection must be inactive")
if data.get("decision_status") != "reserved":
    fail("decision_status must be reserved")
if data.get("selection_status") != "reserved_no_selection":
    fail("selection_status must be reserved_no_selection")
if data.get("requested_provider_id") != "native_mimalloc":
    fail("requested_provider_id must be native_mimalloc for the reserved fixture")
if data.get("selected_provider_id") != "none_reserved":
    fail("selected_provider_id must be none_reserved in M79")
if data.get("proof_bundle_consumed") is not False:
    fail("proof_bundle_consumed must be false")
if data.get("would_build_registry") is not False:
    fail("would_build_registry must be false")
if data.get("would_select_provider") is not False:
    fail("would_select_provider must be false")
if data.get("would_consume_proof_bundle") is not False:
    fail("would_consume_proof_bundle must be false")
if data.get("would_activate") is not False:
    fail("would_activate must be false")
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")

expected_diagnostics = {
    "diagnostic": "[allocator-provider/proof-bundle-consumption-missing]",
    "missing_registry_diagnostic": "[allocator-provider/proof-bundle-registry-missing]",
    "missing_selection_diagnostic": "[allocator-provider/proof-bundle-selection-missing]",
    "missing_provider_proof_diagnostic": "[allocator-provider/proof-bundle-provider-proof-missing]",
    "provider_mismatch_diagnostic": "[allocator-provider/proof-bundle-provider-mismatch]",
    "missing_capability_diagnostic": "[allocator-provider/proof-bundle-capability-missing]",
    "activation_blocked_diagnostic": "[allocator-provider/proof-bundle-activation-blocked]",
}
for key, expected in expected_diagnostics.items():
    if data.get(key) != expected:
        fail(f"{key} must be {expected}")

if data.get("requested_operations") != ["alloc", "realloc", "free"]:
    fail("requested_operations must lock the reserved allocator request")

expected_inputs = [
    "allocator_provider_native_system_proof_v0",
    "allocator_provider_native_mimalloc_proof_v0",
    "allocator_provider_hako_model_proof_v0",
    "allocator_provider_debug_guarded_proof_v0",
]
if data.get("provider_proof_inputs") != expected_inputs:
    fail("provider_proof_inputs must preserve the proof boundary ladder")

expected_ids = [
    "native_system_malloc",
    "native_mimalloc",
    "hako_model_allocator",
    "debug_guarded_allocator",
]
if data.get("candidate_provider_ids") != expected_ids:
    fail("candidate_provider_ids must preserve registry snapshot order")

proofs = data.get("provider_proofs")
if not isinstance(proofs, list) or len(proofs) != 4:
    fail("provider_proofs must list the four reserved provider proofs")
for proof, expected_id, expected_schema in zip(proofs, expected_ids, expected_inputs):
    if proof.get("provider_id") != expected_id:
        fail(f"provider_proofs provider_id mismatch for {expected_id}")
    if proof.get("proof_schema") != expected_schema:
        fail(f"provider_proofs proof_schema mismatch for {expected_id}")
    if proof.get("state") != "reserved":
        fail(f"provider proof {expected_id} must remain reserved")
    if proof.get("consumption") != "future_row_required":
        fail(f"provider proof {expected_id} consumption must require a future row")
    operations = proof.get("operations")
    if not isinstance(operations, list) or not operations:
        fail(f"provider proof {expected_id} must list operations")

required_facts = [
    "registry_snapshot_ready",
    "selection_decision_ready",
    "proof_bundle_caller_provided",
    "requested_provider_id_explicit",
    "selected_provider_id_absent",
    "provider_proof_entries_nonempty",
    "provider_proof_ids_reserved_set",
    "provider_proof_operations_cover_request",
    "proof_bundle_policy_named",
    "fail_fast_proof_bundle_diagnostic_named",
    "no_hidden_environment_toggle",
    "no_implicit_manifest_discovery",
    "no_app_or_facade_name_matching",
    "no_inc_name_matching",
    "no_runtime_registry_implementation",
    "no_provider_selection_implementation",
    "no_runtime_hook_activation",
    "no_global_allocator_attribute",
    "no_global_alloc_trait",
    "no_activation_without_later_row",
]
facts = data.get("required_proof_bundle_consumption_facts")
if not isinstance(facts, list):
    fail("required_proof_bundle_consumption_facts must be a list")
for fact in required_facts:
    if fact not in facts:
        fail(f"missing proof bundle consumption fact: {fact}")
PY

if [[ -e "$FUTURE_REGISTRY_FILE" ]]; then
  fail "future registry owner file must remain absent in M79: $FUTURE_REGISTRY_FILE"
fi

if rg -n 'AllocatorProviderProofBundle|ProviderProofBundle|ProofBundleConsumption|allocator_provider_proof_bundle|provider_proof_bundle_consumption|consume_allocator_provider_proof_bundle|consume_provider_proof_bundle' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".proof_bundle_code 2>&1; then
  cat /tmp/"$TAG".proof_bundle_code >&2
  rm -f /tmp/"$TAG".proof_bundle_code
  fail "provider proof bundle consumption implementation must stay absent in M79"
fi
rm -f /tmp/"$TAG".proof_bundle_code

if rg -n 'AllocatorProviderRegistry|allocator_provider_registry|ProviderRegistryEntry|ProviderRegistrySnapshot|ProviderRegistryBuildInput|ProviderSelectionRequest|ProviderSelectionDecision|select_allocator_provider|allocator_provider_select|allocator_provider_selection_env|NYASH_ALLOCATOR_PROVIDER' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".provider_registry 2>&1; then
  cat /tmp/"$TAG".provider_registry >&2
  rm -f /tmp/"$TAG".provider_registry
  fail "provider registry/selection implementation must stay absent in M79"
fi
rm -f /tmp/"$TAG".provider_registry

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M79"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider proof bundle behavior"
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
