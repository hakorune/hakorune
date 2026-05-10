#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-native-system-proof"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-provider-native-system-proof-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-native-system-proof-v0.toml"
MANIFEST="docs/development/current/main/design/allocator-provider-manifest-v0.toml"
GUARDED_SSOT="docs/development/current/main/design/allocator-provider-debug-guarded-proof-ssot.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-126-M74-NATIVE-SYSTEM-PROVIDER-PROOF-BOUNDARY.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
FUTURE_REGISTRY_FILE="src/runtime/allocator_provider_registry.rs"

echo "[$TAG] checking M74 native system provider proof boundary"

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
require_file "$MANIFEST"
require_file "$GUARDED_SSOT"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Native System Proof (SSOT)"
require_text "$SSOT" "allocator-provider-native-system-proof-v0.toml"
require_text "$SSOT" "[allocator-provider/native-system-proof-missing]"
require_text "$SSOT" "would_select_provider = false"
require_text "$SSOT" "would_activate = false"
require_text "$SSOT" '#[global_allocator]'
require_text "$MANIFEST" 'provider_id = "native_system_malloc"'
require_text "$MANIFEST" 'provider_kind = "native_system_allocator"'
require_text "$GUARDED_SSOT" "debug_guarded_allocator"
require_text "$TASK_BREAKDOWN" "M64-M74"
require_text "$TASK_BREAKDOWN" "M75 may add a native mimalloc provider proof boundary"
require_text "$TASKBOARD" '| `M74 native system provider proof boundary` | `live-docs` |'
require_text "$TASKBOARD" '97. `M74 native system provider proof boundary`'
require_text "$PHASE_README" '`293x-126`'
require_text "$REAL_APP_TASKBOARD" '[x] `293x-126` M74 native system provider proof boundary'
require_text "$CURRENT_STATE" 'latest_card = "293x-126-M74-NATIVE-SYSTEM-PROVIDER-PROOF-BOUNDARY"'
require_text "$CURRENT_STATE" 'latest_card_path = "docs/development/current/main/phases/phase-293x/293x-126-M74-NATIVE-SYSTEM-PROVIDER-PROOF-BOUNDARY.md"'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_native_system_proof_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_native_system_proof_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_native_system_proof_guard.sh"

python3 - <<'PY' "$FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(f"[k2-wide-allocator-provider-native-system-proof][fail] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("schema_version") != "allocator_provider_native_system_proof_v0":
    fail("schema_version must be allocator_provider_native_system_proof_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")
if data.get("provider_id") != "native_system_malloc":
    fail("unexpected provider_id")
if data.get("provider_kind") != "native_system_allocator":
    fail("unexpected provider_kind")
if data.get("manifest_schema") != "allocator_provider_manifest_v0":
    fail("manifest_schema must be allocator_provider_manifest_v0")
if data.get("provider_selection") != "inactive":
    fail("provider_selection must be inactive")
if data.get("diagnostic") != "[allocator-provider/native-system-proof-missing]":
    fail("unexpected diagnostic")
if data.get("would_select_provider") is not False:
    fail("would_select_provider must be false")
if data.get("would_activate") is not False:
    fail("would_activate must be false")
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")

expected_operations = ["alloc", "realloc", "free"]
if data.get("operations") != expected_operations:
    fail("operations must match native system provider operations")

required = [
    "explicit_provider_manifest_fact",
    "provider_readiness_preflight_ready",
    "combined_dry_run_ready",
    "system_allocator_abi_surface_named",
    "malloc_realloc_free_contract_named",
    "oom_failure_policy_named",
    "bootstrap_allocation_path_named",
    "no_global_allocator_attribute",
    "no_process_allocator_replacement",
    "no_runtime_hook_activation",
    "no_hidden_environment_toggle",
    "no_app_or_facade_name_matching",
    "fail_fast_diagnostic_named",
]
proofs = data.get("required_native_system_proofs")
if not isinstance(proofs, list):
    fail("required_native_system_proofs must be a list")
for proof in required:
    if proof not in proofs:
        fail(f"missing required native system proof: {proof}")
PY

if [[ -e "$FUTURE_REGISTRY_FILE" ]]; then
  fail "future registry owner file must remain absent in M74: $FUTURE_REGISTRY_FILE"
fi

if rg -n 'AllocatorProviderRegistry|allocator_provider_registry|select_allocator_provider|allocator_provider_select|allocator_provider_selection_env|NYASH_ALLOCATOR_PROVIDER|ProviderRegistryEntry|ProviderRegistrySnapshot|ProviderRegistryBuildInput|ProviderSelectionRequest|ProviderSelectionDecision' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".provider_registry 2>&1; then
  cat /tmp/"$TAG".provider_registry >&2
  rm -f /tmp/"$TAG".provider_registry
  fail "provider registry/selection implementation must stay absent in M74"
fi
rm -f /tmp/"$TAG".provider_registry

if rg -n 'NativeSystemAllocatorProvider|allocator_provider_native_system|native_system_provider|select_native_system_malloc|allocator_provider_native_system_proof' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".native_system_code 2>&1; then
  cat /tmp/"$TAG".native_system_code >&2
  rm -f /tmp/"$TAG".native_system_code
  fail "native system provider proof must stay docs/fixture-only in M74"
fi
rm -f /tmp/"$TAG".native_system_code

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M74"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider proof behavior"
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
