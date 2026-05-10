#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-selection-decision"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-provider-selection-decision-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-selection-decision-v0.toml"
REGISTRY_SNAPSHOT_SSOT="docs/development/current/main/design/allocator-provider-registry-snapshot-ssot.md"
REGISTRY_SNAPSHOT_FIXTURE="docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml"
ACTIVATION_ENTRY_SSOT="docs/development/current/main/design/allocator-provider-activation-entry-contract-ssot.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-130-M78-ALLOCATOR-PROVIDER-SELECTION-DECISION.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M78 allocator provider selection decision"

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
require_file "$REGISTRY_SNAPSHOT_SSOT"
require_file "$REGISTRY_SNAPSHOT_FIXTURE"
require_file "$ACTIVATION_ENTRY_SSOT"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Selection Decision (SSOT)"
require_text "$SSOT" "allocator-provider-selection-decision-v0.toml"
require_text "$SSOT" "selection_policy = \"explicit_provider_id_required_reserved\""
require_text "$SSOT" "selection_status = \"reserved_no_selection\""
require_text "$SSOT" "selected_provider_id = \"none_reserved\""
require_text "$SSOT" "[allocator-provider/selection-decision-missing]"
require_text "$SSOT" "[allocator-provider/selection-registry-missing]"
require_text "$SSOT" "[allocator-provider/selection-request-missing]"
require_text "$SSOT" "[allocator-provider/selection-unsupported-provider]"
require_text "$SSOT" "[allocator-provider/selection-capability-missing]"
require_text "$SSOT" "[allocator-provider/selection-ambiguous]"
require_text "$SSOT" "would_build_registry = false"
require_text "$SSOT" "would_select_provider = false"
require_text "$SSOT" "would_activate = false"
require_text "$REGISTRY_SNAPSHOT_SSOT" "allocator-provider-registry-snapshot-v0.toml"
require_text "$REGISTRY_SNAPSHOT_FIXTURE" 'provider_id = "native_mimalloc"'
require_text "$ACTIVATION_ENTRY_SSOT" "M78 | selection decision diagnostic shape"
require_text "$TASK_BREAKDOWN" "M78 | selection decision diagnostic shape"
require_text "$TASKBOARD" '| `M78 allocator provider selection decision` | `live-docs` |'
require_text "$TASKBOARD" '101. `M78 allocator provider selection decision`'
require_text "$PHASE_README" '`293x-130`'
require_text "$REAL_APP_TASKBOARD" '[x] `293x-130` M78 allocator provider selection decision'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_selection_decision_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_selection_decision_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_selection_decision_guard.sh"

python3 - <<'PY' "$FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(f"[k2-wide-allocator-provider-selection-decision][fail] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("schema_version") != "allocator_provider_selection_decision_v0":
    fail("schema_version must be allocator_provider_selection_decision_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")
if data.get("selection_owner") != "src/runtime/allocator_provider_registry.rs":
    fail("selection_owner must name the future runtime owner")
if data.get("registry_snapshot_input") != "allocator_provider_registry_snapshot_report":
    fail("registry_snapshot_input must be allocator_provider_registry_snapshot_report")
if data.get("selection_request_source") != "caller_provided_diagnostic_request":
    fail("selection_request_source must be caller-provided")
if data.get("selection_policy") != "explicit_provider_id_required_reserved":
    fail("selection_policy must be explicit_provider_id_required_reserved")
if data.get("selection_status") != "reserved_no_selection":
    fail("selection_status must be reserved_no_selection")
if data.get("deterministic_provider_order") != "registry_snapshot_order":
    fail("deterministic_provider_order must be registry_snapshot_order")
if data.get("provider_selection") != "inactive":
    fail("provider_selection must be inactive")
if data.get("decision_status") != "reserved":
    fail("decision_status must be reserved")
if data.get("requested_provider_id") != "native_mimalloc":
    fail("requested_provider_id must be native_mimalloc for the reserved fixture")
if data.get("selected_provider_id") != "none_reserved":
    fail("selected_provider_id must be none_reserved in M78")
if data.get("would_build_registry") is not False:
    fail("would_build_registry must be false")
if data.get("would_select_provider") is not False:
    fail("would_select_provider must be false")
if data.get("would_activate") is not False:
    fail("would_activate must be false")
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")

expected_diagnostics = {
    "diagnostic": "[allocator-provider/selection-decision-missing]",
    "missing_registry_diagnostic": "[allocator-provider/selection-registry-missing]",
    "missing_request_diagnostic": "[allocator-provider/selection-request-missing]",
    "unsupported_provider_diagnostic": "[allocator-provider/selection-unsupported-provider]",
    "missing_capability_diagnostic": "[allocator-provider/selection-capability-missing]",
    "ambiguous_provider_diagnostic": "[allocator-provider/selection-ambiguous]",
}
for key, expected in expected_diagnostics.items():
    if data.get(key) != expected:
        fail(f"{key} must be {expected}")

required_ops = data.get("required_operations")
if required_ops != ["alloc", "realloc", "free"]:
    fail("required_operations must lock the reserved allocator request")

expected_ids = [
    "native_system_malloc",
    "native_mimalloc",
    "hako_model_allocator",
    "debug_guarded_allocator",
]
if data.get("candidate_provider_ids") != expected_ids:
    fail("candidate_provider_ids must preserve registry snapshot order")

required_facts = [
    "registry_snapshot_ready",
    "selection_request_caller_provided",
    "requested_provider_id_explicit",
    "required_operations_nonempty",
    "candidate_provider_ids_reserved_set",
    "deterministic_provider_order_named",
    "selection_policy_named",
    "fail_fast_selection_diagnostic_named",
    "no_selected_provider_id",
    "no_hidden_environment_toggle",
    "no_implicit_manifest_discovery",
    "no_app_or_facade_name_matching",
    "no_runtime_registry_implementation",
    "no_runtime_hook_activation",
    "no_global_allocator_attribute",
    "no_activation_without_later_row",
]
facts = data.get("required_selection_decision_facts")
if not isinstance(facts, list):
    fail("required_selection_decision_facts must be a list")
for fact in required_facts:
    if fact not in facts:
        fail(f"missing selection decision fact: {fact}")
PY


if rg -n '(^|[^A-Za-z0-9_])select_allocator_provider([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])allocator_provider_select([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])allocator_provider_selection_env([^A-Za-z0-9_]|$)|NYASH_ALLOCATOR_PROVIDER' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".provider_selection 2>&1; then
  cat /tmp/"$TAG".provider_selection >&2
  rm -f /tmp/"$TAG".provider_selection
  fail "provider selection implementation/env toggle must stay absent in M78"
fi
rm -f /tmp/"$TAG".provider_selection

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M78"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider selection behavior"
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
