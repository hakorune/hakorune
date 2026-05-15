#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-debug-guarded-proof"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-debug-guarded-proof-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-debug-guarded-proof-v0.toml"
MANIFEST="docs/development/current/main/design/allocator-provider-manifest-v0.toml"
MODEL_SSOT="docs/development/current/main/design/allocator-provider-hako-model-proof-ssot.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-125-M73-DEBUG-GUARDED-PROVIDER-PROOF-FIXTURE.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M73 debug guarded provider proof fixture"

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
require_file "$MODEL_SSOT"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Debug Guarded Proof (SSOT)"
require_text "$SSOT" "allocator-provider-debug-guarded-proof-v0.toml"
require_text "$SSOT" "[allocator-provider/debug-guarded-proof-missing]"
require_text "$SSOT" "would_select_provider = false"
require_text "$SSOT" "would_activate = false"
require_text "$MANIFEST" 'provider_id = "debug_guarded_allocator"'
require_text "$MANIFEST" 'provider_kind = "debug_guarded_provider"'
require_text "$MODEL_SSOT" "hako_model_allocator"
require_text "$TASK_BREAKDOWN" "M73 | debug guarded provider proof fixture"
require_text "$TASK_BREAKDOWN" "M74 | native system provider proof boundary"
require_text "$TASKBOARD" '| `M73 debug guarded provider proof fixture` | `live-docs` |'
require_text "$TASKBOARD" '96. `M73 debug guarded provider proof fixture`'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_debug_guarded_proof_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_debug_guarded_proof_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_debug_guarded_proof_guard.sh"

python3 - <<'PY' "$FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(f"[k2-wide-allocator-provider-debug-guarded-proof][fail] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("schema_version") != "allocator_provider_debug_guarded_proof_v0":
    fail("schema_version must be allocator_provider_debug_guarded_proof_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")
if data.get("provider_id") != "debug_guarded_allocator":
    fail("unexpected provider_id")
if data.get("provider_kind") != "debug_guarded_provider":
    fail("unexpected provider_kind")
if data.get("manifest_schema") != "allocator_provider_manifest_v0":
    fail("manifest_schema must be allocator_provider_manifest_v0")
if data.get("provider_selection") != "inactive":
    fail("provider_selection must be inactive")
if data.get("diagnostic") != "[allocator-provider/debug-guarded-proof-missing]":
    fail("unexpected diagnostic")
if data.get("would_select_provider") is not False:
    fail("would_select_provider must be false")
if data.get("would_activate") is not False:
    fail("would_activate must be false")
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")

expected_operations = ["alloc", "realloc", "free", "guard_check", "leak_check"]
if data.get("operations") != expected_operations:
    fail("operations must match debug guarded provider operations")

required = [
    "explicit_provider_manifest_fact",
    "provider_readiness_preflight_ready",
    "combined_dry_run_ready",
    "guard_check_lifecycle_bounds_named",
    "leak_check_observation_named",
    "allocation_api_guard_surface_named",
    "no_process_allocator_replacement",
    "no_native_metal_activation",
    "no_hidden_environment_toggle",
    "no_app_or_facade_name_matching",
    "fail_fast_diagnostic_named",
]
proofs = data.get("required_guarded_proofs")
if not isinstance(proofs, list):
    fail("required_guarded_proofs must be a list")
for proof in required:
    if proof not in proofs:
        fail(f"missing required guarded proof: {proof}")
PY


allocator_provider_forbid_selection "$TAG"

if rg -n 'DebugGuardedAllocatorProvider|allocator_provider_debug_guarded|debug_guarded_provider|select_debug_guarded_allocator|allocator_provider_debug_guarded_proof' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".guarded_provider_code 2>&1; then
  cat /tmp/"$TAG".guarded_provider_code >&2
  rm -f /tmp/"$TAG".guarded_provider_code
  fail "debug guarded provider proof must stay docs/fixture-only in M73"
fi
rm -f /tmp/"$TAG".guarded_provider_code

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider proof behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
