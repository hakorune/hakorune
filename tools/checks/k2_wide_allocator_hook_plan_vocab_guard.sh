#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-hook-plan-vocab"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-hook-plan-v0-ssot.md"
MANIFEST="docs/development/current/main/design/allocator-hook-plan-v0.toml"
BOUNDARY="docs/development/current/main/design/allocator-replacement-hook-boundary-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-105-M53-ALLOCATOR-HOOK-PLAN-VOCAB-LOCK.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
M52_GUARD="tools/checks/k2_wide_allocator_replacement_hook_boundary_guard.sh"

echo "[$TAG] checking M53 allocator HookPlan vocabulary"

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
require_file "$MANIFEST"
require_file "$BOUNDARY"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$M52_GUARD"

require_text "$SSOT" "Allocator HookPlan v0 (SSOT)"
require_text "$SSOT" "docs/development/current/main/design/allocator-hook-plan-v0.toml"
require_text "$SSOT" "No active HookPlan row exists yet."
require_text "$BOUNDARY" "M53 allocator HookPlan vocabulary lock"
require_text "$CARD" "M53 Allocator HookPlan Vocabulary Lock"
require_text "$TASKBOARD" '| `M53 allocator HookPlan vocabulary lock` | `live-docs` |'
require_text "$TASKBOARD" '76. `M53 allocator HookPlan vocabulary lock`'
require_text "$PHASE_README" '`293x-105`'
require_text "$REAL_APP_TASKBOARD" '`293x-105` M53 allocator HookPlan vocabulary lock'
require_text "$INDEX" "tools/checks/k2_wide_allocator_hook_plan_vocab_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_hook_plan_vocab_guard.sh"

python3 - <<'PY' "$MANIFEST"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(f"[k2-wide-allocator-hook-plan-vocab][fail] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("schema_version") != "allocator_hook_plan_v0":
    fail("schema_version must be allocator_hook_plan_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")

plans = data.get("plans")
if not isinstance(plans, list) or len(plans) != 1:
    fail("expected exactly one reserved plan fixture")

required = {
    "hook_id",
    "state",
    "entrypoints",
    "policy_owner",
    "substrate_routes",
    "reentrancy_mode",
    "requirements",
    "fail_fast_diagnostic",
    "activation",
}

plan = plans[0]
missing = sorted(required.difference(plan))
if missing:
    fail(f"missing plan keys: {missing}")
if plan["state"] != "reserved":
    fail("plan state must stay reserved")
if plan["reentrancy_mode"] != "not_active":
    fail("reentrancy_mode must stay not_active")
if plan["activation"] != "future_row_required":
    fail("activation must require a future row")
if plan["fail_fast_diagnostic"] != "[allocator-hook/plan-missing]":
    fail("unexpected fail-fast diagnostic tag")
if plan["entrypoints"] != ["alloc", "realloc", "free"]:
    fail("entrypoints must be alloc/realloc/free")
for req in [
    "manifest_fact_required",
    "no_app_name_matching",
    "no_facade_name_matching",
    "no_hidden_env_toggle",
    "fail_fast_diagnostic_required",
]:
    if req not in plan["requirements"]:
        fail(f"missing requirement: {req}")
PY

if rg -n 'hako_alloc_(install|replace)_allocator|allocator_replacement_hook|HakoAllocatorReplacementHook|AllocatorReplacementHookBox|AllocatorHookPlan|HookPlan' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".implementation 2>&1; then
  cat /tmp/"$TAG".implementation >&2
  rm -f /tmp/"$TAG".implementation
  fail "allocator HookPlan implementation symbols must stay absent in M53"
fi
rm -f /tmp/"$TAG".implementation

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M53"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'NYASH_.*ALLOC.*(HOOK|REPLACE)|HAKO_.*ALLOC.*HOOK|HAKORUNE_.*ALLOC.*HOOK' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".env 2>&1; then
  cat /tmp/"$TAG".env >&2
  rm -f /tmp/"$TAG".env
  fail "allocator hook environment toggles must not be introduced in M53"
fi
rm -f /tmp/"$TAG".env

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan' \
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
  fail "inactive allocator-adjacent rows must stay inactive in M53"
fi
rm -f /tmp/"$TAG".inactive_rows

echo "[$TAG] ok"
