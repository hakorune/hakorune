#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-entry-contract"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-provider-activation-entry-contract-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-activation-entry-contract-v0.toml"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-128-M76-ALLOCATOR-PROVIDER-ACTIVATION-ENTRY-CONTRACT.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M76 allocator provider activation entry contract"

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
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Activation Entry Contract (SSOT)"
require_text "$SSOT" "allocator-provider-activation-entry-contract-v0.toml"
require_text "$SSOT" "registry_selection_owner_named"
require_text "$SSOT" "activation_proof_consumed"
require_text "$SSOT" "native_provider_proof_consumed"
require_text "$SSOT" "rollback_behavior_named"
require_text "$SSOT" "[allocator-provider/activation-entry-contract-missing]"
require_text "$SSOT" "would_select_provider = false"
require_text "$SSOT" "would_activate = false"
require_text "$TASK_BREAKDOWN" "M76 | activation entry contract"
require_text "$TASK_BREAKDOWN" "Post-M75 Activation Entry Ladder"
require_text "$TASKBOARD" '| `M76 allocator provider activation entry contract` | `live-docs` |'
require_text "$TASKBOARD" '99. `M76 allocator provider activation entry contract`'
require_text "$PHASE_README" '`293x-128`'
require_text "$REAL_APP_TASKBOARD" '[x] `293x-128` M76 allocator provider activation entry contract'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_entry_contract_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_activation_entry_contract_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_activation_entry_contract_guard.sh"

python3 - <<'PY' "$FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(f"[k2-wide-allocator-provider-activation-entry-contract][fail] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("schema_version") != "allocator_provider_activation_entry_contract_v0":
    fail("schema_version must be allocator_provider_activation_entry_contract_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")
if data.get("provider_selection") != "inactive":
    fail("provider_selection must be inactive")
if data.get("would_select_provider") is not False:
    fail("would_select_provider must be false")
if data.get("would_activate") is not False:
    fail("would_activate must be false")
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")
if data.get("diagnostic") != "[allocator-provider/activation-entry-contract-missing]":
    fail("unexpected diagnostic")

owners = {
    "registry_owner": "src/runtime/allocator_provider_registry.rs",
    "selection_owner": "src/runtime/allocator_provider_registry.rs",
    "activation_preflight_owner": "src/runtime/allocator_hook_dry_run.rs",
    "provider_manifest_owner": "src/runtime/allocator_provider_manifest.rs",
}
for key, expected in owners.items():
    if data.get(key) != expected:
        fail(f"{key} must be {expected}")

required = [
    "registry_selection_owner_named",
    "explicit_provider_manifest_fact",
    "provider_readiness_preflight_ready",
    "combined_dry_run_ready",
    "activation_proof_consumed",
    "native_provider_proof_consumed",
    "fail_fast_selection_diagnostic_named",
    "rollback_behavior_named",
    "no_hidden_environment_toggle",
    "no_implicit_manifest_discovery",
    "no_app_or_facade_name_matching",
    "no_global_allocator_attribute",
    "no_process_allocator_replacement_without_later_row",
]
facts = data.get("required_activation_entry_facts")
if not isinstance(facts, list):
    fail("required_activation_entry_facts must be a list")
for fact in required:
    if fact not in facts:
        fail(f"missing activation entry fact: {fact}")

future_rows = data.get("future_rows")
if not isinstance(future_rows, list) or len(future_rows) != 4:
    fail("future_rows must list the four post-M76 rows")
PY


if rg -n '(^|[^A-Za-z0-9_])select_allocator_provider([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])allocator_provider_select([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])allocator_provider_selection_env([^A-Za-z0-9_]|$)|NYASH_ALLOCATOR_PROVIDER' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".provider_selection 2>&1; then
  cat /tmp/"$TAG".provider_selection >&2
  rm -f /tmp/"$TAG".provider_selection
  fail "provider selection implementation/env toggle must stay absent in M76"
fi
rm -f /tmp/"$TAG".provider_selection

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M76"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider activation behavior"
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
