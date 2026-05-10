#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-hook-activation-proof"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-hook-activation-proof-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-hook-activation-proof-v0.toml"
PLAN_MANIFEST="docs/development/current/main/design/allocator-hook-plan-v0.toml"
DRY_RUN_SSOT="docs/development/current/main/design/allocator-hook-runtime-dry-run-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-107-M55-ALLOCATOR-HOOK-ACTIVATION-PROOF.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
M54_GUARD="tools/checks/k2_wide_allocator_hook_runtime_dry_run_guard.sh"

echo "[$TAG] checking M55 allocator hook activation proof vocabulary"

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
require_file "$PLAN_MANIFEST"
require_file "$DRY_RUN_SSOT"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"
require_file "$M54_GUARD"

require_text "$SSOT" "Allocator Hook Activation Proof (SSOT)"
require_text "$SSOT" "[allocator-hook/activation-proof-missing]"
require_text "$SSOT" "Runtime dry-run and"
require_text "$PLAN_MANIFEST" 'active = false'
require_text "$DRY_RUN_SSOT" "[allocator-hook/dry-run-missing-plan]"
require_text "$CARD" "M55 Allocator Hook Activation Proof"
require_text "$TASKBOARD" '| `M55 allocator hook activation proof` | `live-docs` |'
require_text "$TASKBOARD" '78. `M55 allocator hook activation proof`'
require_text "$PHASE_README" '`293x-107`'
require_text "$REAL_APP_TASKBOARD" '`293x-107` M55 allocator hook activation proof'
require_text "$INDEX" "tools/checks/k2_wide_allocator_hook_activation_proof_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_hook_activation_proof_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_hook_activation_proof_guard.sh"

python3 - <<'PY' "$FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(f"[k2-wide-allocator-hook-activation-proof][fail] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("schema_version") != "allocator_hook_activation_proof_v0":
    fail("schema_version must be allocator_hook_activation_proof_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")
if data.get("hook_id") != "hako_alloc.production.v0":
    fail("unexpected hook_id")
if data.get("diagnostic") != "[allocator-hook/activation-proof-missing]":
    fail("unexpected diagnostic")
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")

required = [
    "explicit_hook_plan_fact",
    "runtime_dry_run_validated",
    "default_inactive",
    "no_app_or_facade_name_matching",
    "no_hidden_environment_toggle",
    "no_process_allocator_replacement_without_activation_row",
    "reentrancy_guard_named",
    "bootstrap_allocation_path_named",
    "no_alloc_no_safepoint_contract_named",
    "rollback_condition_named",
    "fail_fast_diagnostic_named",
]
proofs = data.get("required_proofs")
if not isinstance(proofs, list):
    fail("required_proofs must be a list")
for proof in required:
    if proof not in proofs:
        fail(f"missing required proof: {proof}")
PY

if rg -n 'hako_alloc_(install|replace)_allocator|allocator_replacement_hook|allocator_hook_(dry_run|activate)|HakoAllocatorReplacementHook|AllocatorReplacementHookBox|AllocatorHookPlan|HookPlan' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".implementation 2>&1; then
  cat /tmp/"$TAG".implementation >&2
  rm -f /tmp/"$TAG".implementation
  fail "allocator hook implementation symbols must stay absent in M55"
fi
rm -f /tmp/"$TAG".implementation

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M55"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'NYASH_.*ALLOC.*(HOOK|REPLACE|DRY|ACTIVATE)|HAKO_.*ALLOC.*(HOOK|DRY|ACTIVATE)|HAKORUNE_.*ALLOC.*(HOOK|DRY|ACTIVATE)' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".env 2>&1; then
  cat /tmp/"$TAG".env >&2
  rm -f /tmp/"$TAG".env
  fail "allocator hook environment toggles must not be introduced in M55"
fi
rm -f /tmp/"$TAG".env

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan|allocator_hook_dry_run|allocator_hook_activate|activate_allocator' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  fail "allocator hook/facade/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc

if rg -n 'hako_atomic_ptr_fetch_add|ptr_fetch_add|hako_osvm_(unreserve|release)|unreserve_bytes|release_bytes' \
  src lang/c-abi/shims crates/nyash_kernel lang/src -g '!**/*.md' >/tmp/"$TAG".inactive_rows 2>&1; then
  cat /tmp/"$TAG".inactive_rows >&2
  rm -f /tmp/"$TAG".inactive_rows
  fail "inactive allocator-adjacent rows must stay inactive in M55"
fi
rm -f /tmp/"$TAG".inactive_rows

echo "[$TAG] ok"
