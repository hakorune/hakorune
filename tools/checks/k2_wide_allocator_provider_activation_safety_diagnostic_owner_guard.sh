#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-safety-diagnostic-owner"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-owner-ssot.md"
GATE_SSOT="docs/development/current/main/design/allocator-provider-activation-safety-gate-ssot.md"
GATE_FIXTURE="docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml"
REGISTRY_BOUNDARY_SSOT="docs/development/current/main/design/allocator-provider-registry-boundary-ssot.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-134-M82-ALLOCATOR-PROVIDER-ACTIVATION-SAFETY-DIAGNOSTIC-OWNER.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
M81_GUARD="tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh"

echo "[$TAG] checking M82 allocator provider activation safety diagnostic owner"

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
require_file "$GATE_SSOT"
require_file "$GATE_FIXTURE"
require_file "$REGISTRY_BOUNDARY_SSOT"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"
require_file "$M81_GUARD"

require_text "$SSOT" "Allocator Provider Activation Safety Diagnostic Owner (SSOT)"
require_text "$SSOT" "src/runtime/allocator_provider_registry.rs"
require_text "$SSOT" "validate_allocator_provider_activation_safety_gate(evidence)"
require_text "$SSOT" "activation_gate_open = false"
require_text "$SSOT" "would_open_activation_gate = false"
require_text "$SSOT" "would_activate_hook = false"
require_text "$SSOT" "would_activate = false"
require_text "$SSOT" "past guards must not require the future provider registry owner file to stay"
require_text "$GATE_SSOT" "Allocator Provider Activation Safety Gate (SSOT)"
require_text "$GATE_FIXTURE" 'safety_gate_owner = "src/runtime/allocator_provider_registry.rs"'
require_text "$GATE_FIXTURE" 'activation_safety_gate = "inactive"'
require_text "$GATE_FIXTURE" 'activation_gate_open = false'
require_text "$GATE_FIXTURE" 'would_open_activation_gate = false'
require_text "$GATE_FIXTURE" 'would_activate = false'
require_text "$REGISTRY_BOUNDARY_SSOT" "ProviderRegistrySnapshot"
require_text "$REGISTRY_BOUNDARY_SSOT" "ProviderSelectionDecision"
require_text "$TASK_BREAKDOWN" "M82 | activation safety diagnostic owner"
require_text "$TASKBOARD" '| `M82 allocator provider activation safety diagnostic owner` | `live-docs` |'
require_text "$TASKBOARD" '105. `M82 allocator provider activation safety diagnostic owner`'
require_text "$CARD" "293x-134 M82 Allocator Provider Activation Safety Diagnostic Owner"
require_text "$PHASE_README" '`293x-134`'
require_text "$REAL_APP_TASKBOARD" '[x] `293x-134` M82 allocator provider activation safety diagnostic owner'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_owner_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_owner_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_owner_guard.sh"

python3 - <<'PY' "$GATE_FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(f"[k2-wide-allocator-provider-activation-safety-diagnostic-owner][fail] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("safety_gate_owner") != "src/runtime/allocator_provider_registry.rs":
    fail("safety_gate_owner must name src/runtime/allocator_provider_registry.rs")
if data.get("activation_safety_gate") != "inactive":
    fail("activation_safety_gate must remain inactive")
if data.get("safety_status") != "reserved_gate_closed":
    fail("safety_status must remain reserved_gate_closed")
for key in [
    "activation_gate_open",
    "would_open_activation_gate",
    "would_activate_hook",
    "would_activate",
]:
    if data.get(key) is not False:
        fail(f"{key} must remain false")
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")
PY

past_guard_files=$(find tools/checks -maxdepth 1 -name 'k2_wide_allocator_provider_*_guard.sh' \
  ! -name 'k2_wide_allocator_provider_activation_safety_diagnostic_owner_guard.sh' \
  -print)

if rg -n 'FUTURE_REGISTRY_FILE|future registry owner file must remain absent' $past_guard_files >/tmp/"$TAG".owner_file_pin 2>&1; then
  cat /tmp/"$TAG".owner_file_pin >&2
  rm -f /tmp/"$TAG".owner_file_pin
  fail "past provider guards must not pin the future registry owner file as absent"
fi
rm -f /tmp/"$TAG".owner_file_pin

if rg -n "if rg -n '.*(AllocatorProviderRegistry|allocator_provider_registry|ProviderRegistryEntry|ProviderRegistrySnapshot|ProviderRegistryBuildInput|ProviderSelectionRequest|ProviderSelectionDecision)" \
  $past_guard_files >/tmp/"$TAG".owner_type_pin 2>&1; then
  cat /tmp/"$TAG".owner_type_pin >&2
  rm -f /tmp/"$TAG".owner_type_pin
  fail "past provider guards must not block future diagnostic owner/type names"
fi
rm -f /tmp/"$TAG".owner_type_pin

if rg -n '(^|[^A-Za-z0-9_])open_activation_gate([^A-Za-z0-9_]|$)' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".gate_open 2>&1; then
  cat /tmp/"$TAG".gate_open >&2
  rm -f /tmp/"$TAG".gate_open
  fail "activation gate opening must stay absent in M82"
fi
rm -f /tmp/"$TAG".gate_open

if rg -n '(^|[^A-Za-z0-9_])select_allocator_provider([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])allocator_provider_select([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])allocator_provider_selection_env([^A-Za-z0-9_]|$)|NYASH_ALLOCATOR_PROVIDER' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".provider_selection 2>&1; then
  cat /tmp/"$TAG".provider_selection >&2
  rm -f /tmp/"$TAG".provider_selection
  fail "provider selection implementation/env toggle must stay absent in M82"
fi
rm -f /tmp/"$TAG".provider_selection

if rg -n 'allocator_hook_activate|activate_allocator|install_allocator_hook|replace_allocator' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".hook_activation 2>&1; then
  cat /tmp/"$TAG".hook_activation >&2
  rm -f /tmp/"$TAG".hook_activation
  fail "hook activation/process allocator replacement must stay absent in M82"
fi
rm -f /tmp/"$TAG".hook_activation

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M82"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider activation safety diagnostics"
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
