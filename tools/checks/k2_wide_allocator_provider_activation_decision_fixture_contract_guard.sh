#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-decision-fixture-contract"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

PROPOSAL_SSOT="docs/development/current/main/design/allocator-provider-activation-decision-surface-proposal-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-activation-decision-v0.toml"
LIGHTWEIGHT_DOCS_SSOT="docs/development/current/main/design/allocator-provider-lightweight-doc-sync-policy-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-140-M87-ALLOCATOR-PROVIDER-ACTIVATION-DECISION-FIXTURE-CONTRACT.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M87 allocator provider activation decision fixture contract"

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

require_file "$PROPOSAL_SSOT"
require_file "$FIXTURE"
require_file "$LIGHTWEIGHT_DOCS_SSOT"
require_file "$CARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$PROPOSAL_SSOT" "The next safe row is M87 activation decision fixture contract."
require_text "$PROPOSAL_SSOT" 'surface_version = "allocator_provider_activation_decision_v0"'
require_text "$FIXTURE" 'surface_version = "allocator_provider_activation_decision_v0"'
require_text "$FIXTURE" 'input_source = "caller_provided_activation_decision_bundle"'
require_text "$FIXTURE" 'operator_intent = "diagnose"'
require_text "$FIXTURE" 'activation_decision_allowed = false'
require_text "$FIXTURE" 'would_select_provider = false'
require_text "$FIXTURE" 'would_consume_proof = false'
require_text "$FIXTURE" 'would_prepare_rollback = false'
require_text "$FIXTURE" 'would_open_activation_gate = false'
require_text "$FIXTURE" 'would_install_hook = false'
require_text "$FIXTURE" 'would_replace_process_allocator = false'
require_text "$FIXTURE" 'would_activate = false'
require_text "$FIXTURE" '[allocator-provider/activation-decision-blocked]'
require_text "$LIGHTWEIGHT_DOCS_SSOT" "M87+ row guards must not require phase README"
require_text "$CARD" "293x-140 M87 Allocator Provider Activation Decision Fixture Contract"
require_text "$CARD" "activation_decision_allowed = false"
require_text "$CARD" "This is docs/fixture/guard only."
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_decision_fixture_contract_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_activation_decision_fixture_contract_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_activation_decision_fixture_contract_guard.sh"

python3 - <<'PY' "$FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(
        f"[k2-wide-allocator-provider-activation-decision-fixture-contract][fail] {message}",
        file=sys.stderr,
    )
    raise SystemExit(1)

if data.get("surface_version") != "allocator_provider_activation_decision_v0":
    fail("surface_version must be allocator_provider_activation_decision_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")
if data.get("decision_surface_owner") != "future_row_required":
    fail("decision_surface_owner must require a future row")
if data.get("input_source") != "caller_provided_activation_decision_bundle":
    fail("input_source must be caller-provided")
if data.get("operator_intent") != "diagnose":
    fail("operator_intent must remain diagnose-only")
if data.get("requested_provider_id") != "mimalloc":
    fail("requested_provider_id must lock the reserved mimalloc request")

expected_paths = {
    "activation_safety_gate_report_path": "activation-safety-gate-v0.toml",
    "registry_snapshot_path": "registry-snapshot-v0.toml",
    "selection_decision_path": "selection-decision-v0.toml",
    "proof_bundle_report_path": "proof-bundle-consumption-v0.toml",
    "rollback_preflight_report_path": "rollback-preflight-v0.toml",
}
for key, expected in expected_paths.items():
    if data.get(key) != expected:
        fail(f"{key} must be {expected}")

expected_inactive = {
    "activation_decision_surface_status": "reserved_fixture",
    "provider_selection": "inactive",
    "proof_bundle_consumption": "inactive",
    "rollback_preparation": "inactive",
    "activation_gate": "closed_reserved",
    "hook_activation": "inactive",
    "process_allocator_replacement": "inactive",
    "activation": "future_row_required",
}
for key, expected in expected_inactive.items():
    if data.get(key) != expected:
        fail(f"{key} must be {expected}")

for key in [
    "activation_decision_allowed",
    "would_select_provider",
    "would_consume_proof",
    "would_prepare_rollback",
    "would_open_activation_gate",
    "would_install_hook",
    "would_replace_process_allocator",
    "would_activate",
]:
    if data.get(key) is not False:
        fail(f"{key} must be false")

expected_diagnostics = {
    "diagnostic": "[allocator-provider/activation-decision-reserved]",
    "missing_activation_safety_gate_diagnostic": "[allocator-provider/activation-decision-safety-gate-missing]",
    "missing_registry_snapshot_diagnostic": "[allocator-provider/activation-decision-registry-missing]",
    "missing_selection_decision_diagnostic": "[allocator-provider/activation-decision-selection-missing]",
    "missing_proof_bundle_diagnostic": "[allocator-provider/activation-decision-proof-bundle-missing]",
    "missing_rollback_preflight_diagnostic": "[allocator-provider/activation-decision-rollback-missing]",
    "activation_blocked_diagnostic": "[allocator-provider/activation-decision-blocked]",
}
for key, expected in expected_diagnostics.items():
    if data.get(key) != expected:
        fail(f"{key} must be {expected}")

required_facts = [
    "activation_decision_bundle_caller_provided",
    "operator_intent_diagnose_only",
    "requested_provider_id_explicit",
    "activation_safety_gate_report_path_explicit",
    "registry_snapshot_path_explicit",
    "selection_decision_path_explicit",
    "proof_bundle_report_path_explicit",
    "rollback_preflight_report_path_explicit",
    "activation_decision_allowed_false",
    "activation_gate_closed",
    "fail_fast_activation_decision_diagnostic_named",
    "no_hidden_environment_toggle",
    "no_implicit_manifest_discovery",
    "no_implicit_proof_discovery",
    "no_implicit_report_discovery",
    "no_app_or_facade_name_matching",
    "no_inc_name_matching",
    "no_runtime_decision_parser",
    "no_cli_route",
    "no_runtime_registry_implementation",
    "no_provider_selection_implementation",
    "no_proof_consumption_implementation",
    "no_rollback_preparation_implementation",
    "no_activation_gate_opening",
    "no_hook_activation_implementation",
    "no_global_allocator_attribute",
    "no_global_alloc_trait",
    "no_process_allocator_replacement",
    "no_route_widening",
]
facts = data.get("reserved_activation_decision_facts")
if not isinstance(facts, list):
    fail("reserved_activation_decision_facts must be a list")
for fact in required_facts:
    if fact not in facts:
        fail(f"missing activation decision fact: {fact}")

expected_inputs = {
    "activation_safety_gate": (
        "activation_safety_gate_report_path",
        "[allocator-provider/activation-decision-safety-gate-missing]",
    ),
    "registry_snapshot": (
        "registry_snapshot_path",
        "[allocator-provider/activation-decision-registry-missing]",
    ),
    "selection_decision": (
        "selection_decision_path",
        "[allocator-provider/activation-decision-selection-missing]",
    ),
    "proof_bundle": (
        "proof_bundle_report_path",
        "[allocator-provider/activation-decision-proof-bundle-missing]",
    ),
    "rollback_preflight": (
        "rollback_preflight_report_path",
        "[allocator-provider/activation-decision-rollback-missing]",
    ),
}
inputs = data.get("activation_decision_inputs")
if not isinstance(inputs, list) or len(inputs) != len(expected_inputs):
    fail("activation_decision_inputs must list the five explicit diagnostic inputs")
seen = set()
for item in inputs:
    name = item.get("name")
    seen.add(name)
    if name not in expected_inputs:
        fail(f"unexpected activation decision input: {name}")
    expected_path_key, expected_diagnostic = expected_inputs[name]
    if item.get("source") != "caller_provided_path":
        fail(f"activation decision input {name} must be caller-provided")
    if item.get("path_key") != expected_path_key:
        fail(f"activation decision input {name} path_key mismatch")
    if item.get("required") is not True:
        fail(f"activation decision input {name} must be required")
    if item.get("missing_diagnostic") != expected_diagnostic:
        fail(f"activation decision input {name} diagnostic mismatch")
if seen != set(expected_inputs):
    fail("activation_decision_inputs must cover all explicit diagnostics")
PY

if rg -n -e '--allocator-provider-activation-decision|activation_decision|ActivationDecision' src -g '*.rs' >/tmp/"$TAG".src 2>&1; then
  cat /tmp/"$TAG".src >&2
  rm -f /tmp/"$TAG".src
  fail "M87 fixture contract must not add activation decision runtime or CLI code"
fi
rm -f /tmp/"$TAG".src

allocator_provider_forbid_activation_gate_open "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
