#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-hako-model-proof"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-hako-model-proof-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-hako-model-proof-v0.toml"
MANIFEST="docs/development/current/main/design/allocator-provider-manifest-v0.toml"
REGISTRY_SSOT="docs/development/current/main/design/allocator-provider-registry-boundary-ssot.md"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-124-M72-HAKO-MODEL-PROVIDER-PROOF-FIXTURE.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M72 hako model provider proof fixture"

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
require_file "$REGISTRY_SSOT"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Hako Model Proof (SSOT)"
require_text "$SSOT" "allocator-provider-hako-model-proof-v0.toml"
require_text "$SSOT" "[allocator-provider/hako-model-proof-missing]"
require_text "$SSOT" "would_select_provider = false"
require_text "$SSOT" "would_activate = false"
require_text "$MANIFEST" 'provider_id = "hako_model_allocator"'
require_text "$MANIFEST" 'provider_kind = "hako_policy_model"'
require_text "$REGISTRY_SSOT" "ProviderSelectionDecision"
require_text "$TASK_BREAKDOWN" "M72 | hako model provider proof fixture"
require_text "$TASK_BREAKDOWN" "M73 | debug guarded provider proof fixture"
require_text "$TASKBOARD" '| `M72 hako model provider proof fixture` | `live-docs` |'
require_text "$TASKBOARD" '95. `M72 hako model provider proof fixture`'
require_text "$PHASE_README" '`293x-124`'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_hako_model_proof_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_hako_model_proof_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_hako_model_proof_guard.sh"

python3 - <<'PY' "$FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(f"[k2-wide-allocator-provider-hako-model-proof][fail] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("schema_version") != "allocator_provider_hako_model_proof_v0":
    fail("schema_version must be allocator_provider_hako_model_proof_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")
if data.get("provider_id") != "hako_model_allocator":
    fail("unexpected provider_id")
if data.get("provider_kind") != "hako_policy_model":
    fail("unexpected provider_kind")
if data.get("manifest_schema") != "allocator_provider_manifest_v0":
    fail("manifest_schema must be allocator_provider_manifest_v0")
if data.get("provider_selection") != "inactive":
    fail("provider_selection must be inactive")
if data.get("diagnostic") != "[allocator-provider/hako-model-proof-missing]":
    fail("unexpected diagnostic")
if data.get("would_select_provider") is not False:
    fail("would_select_provider must be false")
if data.get("would_activate") is not False:
    fail("would_activate must be false")
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")

expected_operations = ["model_alloc", "model_free", "stats", "stress_validate"]
if data.get("operations") != expected_operations:
    fail("operations must match hako policy model provider operations")

required = [
    "explicit_provider_manifest_fact",
    "provider_readiness_preflight_ready",
    "combined_dry_run_ready",
    "hako_alloc_policy_state_named",
    "model_alloc_free_state_transition_named",
    "model_stats_observation_named",
    "stress_validate_fixture_named",
    "no_native_pointer_or_metal_activation",
    "no_process_allocator_replacement",
    "no_hidden_environment_toggle",
    "no_app_or_facade_name_matching",
    "fail_fast_diagnostic_named",
]
proofs = data.get("required_model_proofs")
if not isinstance(proofs, list):
    fail("required_model_proofs must be a list")
for proof in required:
    if proof not in proofs:
        fail(f"missing required model proof: {proof}")
PY


allocator_provider_forbid_selection "$TAG"

if rg -n 'HakoModelAllocatorProvider|allocator_provider_hako_model|hako_model_provider|select_hako_model_allocator|allocator_provider_hako_model_proof' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".model_provider_code 2>&1; then
  cat /tmp/"$TAG".model_provider_code >&2
  rm -f /tmp/"$TAG".model_provider_code
  fail "hako model provider proof must stay docs/fixture-only in M72"
fi
rm -f /tmp/"$TAG".model_provider_code

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider proof behavior"
fi
rm -f /tmp/"$TAG".runner

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan|allocator_hook_activate|activate_allocator|hako_model_allocator|native_mimalloc|native_system_malloc' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  fail "allocator provider/hook/facade/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc

echo "[$TAG] ok"
