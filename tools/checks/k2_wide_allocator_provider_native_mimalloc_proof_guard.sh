#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-native-mimalloc-proof"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-native-mimalloc-proof-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-native-mimalloc-proof-v0.toml"
MANIFEST="docs/development/current/main/design/allocator-provider-manifest-v0.toml"
SYSTEM_SSOT="docs/development/current/main/design/allocator-provider-native-system-proof-ssot.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-127-M75-NATIVE-MIMALLOC-PROVIDER-PROOF-BOUNDARY.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M75 native mimalloc provider proof boundary"

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
require_file "$SYSTEM_SSOT"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Native Mimalloc Proof (SSOT)"
require_text "$SSOT" "allocator-provider-native-mimalloc-proof-v0.toml"
require_text "$SSOT" "[allocator-provider/native-mimalloc-proof-missing]"
require_text "$SSOT" "would_select_provider = false"
require_text "$SSOT" "would_activate = false"
require_text "$SSOT" "production mimalloc activation"
require_text "$MANIFEST" 'provider_id = "native_mimalloc"'
require_text "$MANIFEST" 'provider_kind = "native_mimalloc_allocator"'
require_text "$SYSTEM_SSOT" "native_system_malloc"
require_text "$TASK_BREAKDOWN" "M64-M75"
require_text "$TASK_BREAKDOWN" "Provider proof boundary ladder is now closed"
require_text "$TASKBOARD" '| `M75 native mimalloc provider proof boundary` | `live-docs` |'
require_text "$TASKBOARD" '98. `M75 native mimalloc provider proof boundary`'
require_text "$PHASE_README" '`293x-127`'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_native_mimalloc_proof_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_native_mimalloc_proof_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_native_mimalloc_proof_guard.sh"

python3 - <<'PY' "$FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(f"[k2-wide-allocator-provider-native-mimalloc-proof][fail] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("schema_version") != "allocator_provider_native_mimalloc_proof_v0":
    fail("schema_version must be allocator_provider_native_mimalloc_proof_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")
if data.get("provider_id") != "native_mimalloc":
    fail("unexpected provider_id")
if data.get("provider_kind") != "native_mimalloc_allocator":
    fail("unexpected provider_kind")
if data.get("manifest_schema") != "allocator_provider_manifest_v0":
    fail("manifest_schema must be allocator_provider_manifest_v0")
if data.get("provider_selection") != "inactive":
    fail("provider_selection must be inactive")
if data.get("diagnostic") != "[allocator-provider/native-mimalloc-proof-missing]":
    fail("unexpected diagnostic")
if data.get("would_select_provider") is not False:
    fail("would_select_provider must be false")
if data.get("would_activate") is not False:
    fail("would_activate must be false")
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")

expected_operations = ["alloc", "realloc", "free", "page_reserve", "page_commit", "page_decommit"]
if data.get("operations") != expected_operations:
    fail("operations must match native mimalloc provider operations")

required = [
    "explicit_provider_manifest_fact",
    "provider_readiness_preflight_ready",
    "combined_dry_run_ready",
    "mimalloc_allocator_abi_surface_named",
    "mimalloc_page_lifecycle_contract_named",
    "mimalloc_size_class_policy_named",
    "mimalloc_remote_free_policy_named",
    "mimalloc_tls_cache_policy_named",
    "no_production_activation_without_later_row",
    "no_global_allocator_attribute",
    "no_process_allocator_replacement",
    "no_runtime_hook_activation",
    "no_hidden_environment_toggle",
    "no_app_or_facade_name_matching",
    "fail_fast_diagnostic_named",
]
proofs = data.get("required_native_mimalloc_proofs")
if not isinstance(proofs, list):
    fail("required_native_mimalloc_proofs must be a list")
for proof in required:
    if proof not in proofs:
        fail(f"missing required native mimalloc proof: {proof}")
PY


allocator_provider_forbid_selection "$TAG"

if rg -n 'NativeMimallocProvider|allocator_provider_native_mimalloc|native_mimalloc_provider|select_native_mimalloc|allocator_provider_native_mimalloc_proof' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".native_mimalloc_code 2>&1; then
  cat /tmp/"$TAG".native_mimalloc_code >&2
  rm -f /tmp/"$TAG".native_mimalloc_code
  fail "native mimalloc provider proof must stay docs/fixture-only in M75"
fi
rm -f /tmp/"$TAG".native_mimalloc_code

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider proof behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
