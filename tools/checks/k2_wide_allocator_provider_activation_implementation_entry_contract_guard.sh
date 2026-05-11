#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-implementation-entry-contract"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-v0.toml"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-145-M92-ALLOCATOR-PROVIDER-ACTIVATION-IMPLEMENTATION-ENTRY-CONTRACT.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M92 allocator provider activation implementation entry contract"

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
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Activation Implementation Entry Contract (SSOT)"
require_text "$SSOT" "M92 is docs/fixture/guard only"
require_text "$SSOT" "src/runtime/allocator_provider_activation.rs"
require_text "$SSOT" "allocator_provider_activation_attempt"
require_text "$SSOT" "[allocator-provider/activation-implementation-entry-missing]"
require_text "$SSOT" "M93 | registry snapshot diagnostic report"
require_text "$SSOT" "M95 | activation diagnostic closeout inventory"
require_text "$SSOT" "M96 | selection decision diagnostic report"
require_text "$SSOT" "would_build_registry = false"
require_text "$SSOT" "would_select_provider = false"
require_text "$SSOT" "would_consume_proof = false"
require_text "$SSOT" "would_prepare_rollback = false"
require_text "$SSOT" "would_open_activation_gate = false"
require_text "$SSOT" "would_install_hook = false"
require_text "$SSOT" "would_replace_process_allocator = false"
require_text "$SSOT" "would_activate = false"
require_text "$TASK_BREAKDOWN" "M92 | activation implementation entry contract"
require_text "$TASK_BREAKDOWN" "M93 registry snapshot diagnostic report"
require_text "$CARD" "293x-145 M92 Allocator Provider Activation Implementation Entry Contract"
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_implementation_entry_contract_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_activation_implementation_entry_contract_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_activation_implementation_entry_contract_guard.sh"

python3 - <<'PY' "$FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(
        f"[k2-wide-allocator-provider-activation-implementation-entry-contract][fail] {message}",
        file=sys.stderr,
    )
    raise SystemExit(1)

if data.get("schema_version") != "allocator_provider_activation_implementation_entry_contract_v0":
    fail("schema_version must be allocator_provider_activation_implementation_entry_contract_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")
if data.get("activation_implementation_status") != "owner_entry_reserved":
    fail("activation_implementation_status must be owner_entry_reserved")
if data.get("activation_owner") != "src/runtime/allocator_provider_activation.rs":
    fail("activation_owner must name the single future activation owner")
if data.get("activation_entry") != "allocator_provider_activation_attempt":
    fail("activation_entry must be allocator_provider_activation_attempt")
if data.get("provider_selection") != "inactive":
    fail("provider_selection must stay inactive")
if data.get("proof_bundle_consumption") != "future_row_required":
    fail("proof_bundle_consumption must require a future row")
if data.get("rollback_preparation") != "future_row_required":
    fail("rollback_preparation must require a future row")
if data.get("activation_gate") != "closed_reserved":
    fail("activation_gate must stay closed_reserved")
if data.get("hook_activation") != "future_row_required":
    fail("hook_activation must require a future row")
if data.get("process_allocator_replacement") != "future_row_required":
    fail("process_allocator_replacement must require a future row")
for key in [
    "would_build_registry",
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
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")
if data.get("diagnostic") != "[allocator-provider/activation-implementation-entry-missing]":
    fail("unexpected diagnostic")

required = [
    "activation_implementation_owner_named",
    "activation_attempt_entry_named",
    "activation_decision_report_explicit",
    "registry_snapshot_report_explicit",
    "selection_decision_report_explicit",
    "proof_bundle_report_explicit",
    "rollback_preflight_report_explicit",
    "activation_safety_gate_report_explicit",
    "fail_fast_activation_attempt_diagnostic_named",
    "no_hidden_environment_toggle",
    "no_implicit_manifest_discovery",
    "no_implicit_report_discovery",
    "no_provider_selection_implementation",
    "no_proof_consumption_implementation",
    "no_rollback_preparation_implementation",
    "no_activation_gate_opening",
    "no_hook_activation_implementation",
    "no_process_allocator_replacement",
]
facts = data.get("required_activation_implementation_entry_facts")
if not isinstance(facts, list):
    fail("required_activation_implementation_entry_facts must be a list")
for fact in required:
    if fact not in facts:
        fail(f"missing activation implementation entry fact: {fact}")

future_rows = data.get("future_rows")
expected_future_rows = [
    "M93 registry snapshot diagnostic report",
    "M94 registry snapshot CLI surface",
    "M95 activation diagnostic closeout inventory",
    "M96 selection decision diagnostic report",
    "M97 selection decision CLI surface",
]
if future_rows != expected_future_rows:
    fail("future_rows must list M93-M97 in fixed order")
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
  fail "runner must not own allocator provider activation implementation entry behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
