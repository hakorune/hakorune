#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-proof-bundle-consumption-entry-contract"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-proof-bundle-consumption-entry-contract-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-proof-bundle-consumption-entry-contract-v0.toml"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-156-M100-ALLOCATOR-PROVIDER-PROOF-BUNDLE-CONSUMPTION-ENTRY-CONTRACT.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M100 allocator provider proof bundle consumption entry contract"

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
require_file "$TASK_BREAKDOWN"
require_file "$CARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Proof Bundle Consumption Entry Contract (SSOT)"
require_text "$SSOT" "M100 is docs/fixture/guard only"
require_text "$SSOT" "src/runtime/allocator_provider_activation.rs"
require_text "$SSOT" "allocator_provider_proof_bundle_consumption_attempt"
require_text "$SSOT" "src/cli/allocator_provider_proof_bundle_consumption.rs"
require_text "$SSOT" "[allocator-provider/proof-bundle-consumption-entry-missing]"
require_text "$SSOT" "proof_bundle_consumption_implementation_status = \"owner_entry_reserved\""
require_text "$SSOT" "would_consume_proof_bundle = false"
require_text "$TASK_BREAKDOWN" "M100 | proof bundle consumption entry contract"
require_text "$TASK_BREAKDOWN" "M100 proof bundle consumption entry contract"
require_text "$CARD" "293x-156 M100 Allocator Provider Proof Bundle Consumption Entry Contract"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_entry_contract_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_entry_contract_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_entry_contract_guard.sh"

python3 - <<'PY' "$FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())


def fail(message: str) -> None:
    print(
        f"[k2-wide-allocator-provider-proof-bundle-consumption-entry-contract][fail] {message}",
        file=sys.stderr,
    )
    raise SystemExit(1)


if data.get("schema_version") != "allocator_provider_proof_bundle_consumption_entry_contract_v0":
    fail("schema_version must be allocator_provider_proof_bundle_consumption_entry_contract_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")
if data.get("proof_bundle_consumption_implementation_status") != "owner_entry_reserved":
    fail("proof_bundle_consumption_implementation_status must be owner_entry_reserved")
if data.get("activation_owner") != "src/runtime/allocator_provider_activation.rs":
    fail("activation_owner must name the activation owner")
if data.get("proof_bundle_consumption_owner") != "src/runtime/allocator_provider_activation.rs":
    fail("proof_bundle_consumption_owner must stay under the activation owner")
if data.get("proof_bundle_consumption_entry") != "allocator_provider_proof_bundle_consumption_attempt":
    fail("proof_bundle_consumption_entry must be allocator_provider_proof_bundle_consumption_attempt")
if data.get("diagnostic_cli_owner") != "src/cli/allocator_provider_proof_bundle_consumption.rs":
    fail("diagnostic_cli_owner must remain the M99 diagnostic CLI owner")
if data.get("selected_provider_requirement") != "future_selected_provider_required":
    fail("selected_provider_requirement must require a future selected provider")
if data.get("provider_proof_bundle_source") != "caller_provided_explicit_bundle":
    fail("provider_proof_bundle_source must be caller_provided_explicit_bundle")
if data.get("proof_bundle_consumption") != "future_row_required":
    fail("proof_bundle_consumption must require a future row")
if data.get("provider_selection") != "inactive":
    fail("provider_selection must stay inactive")
if data.get("rollback_preparation") != "future_row_required":
    fail("rollback_preparation must require a future row")
if data.get("activation_gate") != "closed_reserved":
    fail("activation_gate must stay closed_reserved")
if data.get("hook_activation") != "future_row_required":
    fail("hook_activation must require a future row")
if data.get("process_allocator_replacement") != "future_row_required":
    fail("process_allocator_replacement must require a future row")
if data.get("proof_bundle_consumed") is not False:
    fail("proof_bundle_consumed must be false")
for key in [
    "would_build_registry",
    "would_select_provider",
    "would_consume_proof_bundle",
    "would_prepare_rollback",
    "would_open_activation_gate",
    "would_install_hook",
    "would_replace_process_allocator",
    "would_activate",
]:
    if data.get(key) is not False:
        fail(f"{key} must be false")
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")
if data.get("diagnostic") != "[allocator-provider/proof-bundle-consumption-entry-missing]":
    fail("unexpected diagnostic")

required = [
    "activation_owner_named",
    "proof_bundle_consumption_owner_named",
    "proof_bundle_consumption_entry_named",
    "activation_decision_report_explicit",
    "registry_snapshot_report_explicit",
    "selection_decision_report_explicit",
    "proof_bundle_consumption_report_explicit",
    "selected_provider_required_before_consumption",
    "provider_proof_bundle_explicit",
    "fail_fast_proof_bundle_entry_diagnostic_named",
    "no_hidden_environment_toggle",
    "no_implicit_manifest_discovery",
    "no_implicit_report_discovery",
    "no_implicit_proof_discovery",
    "no_provider_selection_implementation",
    "no_proof_consumption_implementation",
    "no_rollback_preparation_implementation",
    "no_activation_gate_opening",
    "no_hook_activation_implementation",
    "no_process_allocator_replacement",
]
facts = data.get("required_proof_bundle_consumption_entry_facts")
if not isinstance(facts, list):
    fail("required_proof_bundle_consumption_entry_facts must be a list")
for fact in required:
    if fact not in facts:
        fail(f"missing proof bundle consumption entry fact: {fact}")
PY

allocator_provider_forbid_activation_gate_open "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider proof bundle behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
