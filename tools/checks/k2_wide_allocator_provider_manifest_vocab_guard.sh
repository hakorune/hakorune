#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-manifest-vocab"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-provider-manifest-v0-ssot.md"
MANIFEST="docs/development/current/main/design/allocator-provider-manifest-v0.toml"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-117-M65-ALLOCATOR-PROVIDER-MANIFEST-VOCAB.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M65 allocator provider manifest vocabulary"

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
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Manifest v0 (SSOT)"
require_text "$MANIFEST" 'schema_version = "allocator_provider_manifest_v0"'
require_text "$MANIFEST" 'active = false'
require_text "$MANIFEST" 'provider_selection = "inactive"'
require_text "$MANIFEST" 'provider_id = "native_system_malloc"'
require_text "$MANIFEST" 'provider_id = "native_mimalloc"'
require_text "$MANIFEST" 'provider_id = "hako_model_allocator"'
require_text "$MANIFEST" 'provider_id = "debug_guarded_allocator"'
require_text "$TASKBOARD" '| `M65 allocator provider manifest vocabulary` | `live-docs` |'
require_text "$TASKBOARD" '88. `M65 allocator provider manifest vocabulary`'
require_text "$PHASE_README" '`293x-117`'
require_text "$REAL_APP_TASKBOARD" '`293x-117` M65 allocator provider manifest vocabulary'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_manifest_vocab_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_manifest_vocab_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_manifest_vocab_guard.sh"

python3 - "$MANIFEST" <<'PY'
import sys
import tomllib

path = sys.argv[1]
with open(path, "rb") as f:
    data = tomllib.load(f)

def fail(msg):
    raise SystemExit(msg)

if data.get("schema_version") != "allocator_provider_manifest_v0":
    fail("bad schema_version")
if data.get("status") != "reserved":
    fail("manifest must stay reserved")
if data.get("active") is not False:
    fail("manifest must stay inactive")
if data.get("provider_selection") != "inactive":
    fail("provider selection must stay inactive")
if data.get("activation") != "future_row_required":
    fail("activation must stay future-only")

providers = data.get("providers")
if not isinstance(providers, list):
    fail("providers must be a list")

expected = {
    "native_system_malloc",
    "native_mimalloc",
    "hako_model_allocator",
    "debug_guarded_allocator",
}
seen = set()
for provider in providers:
    pid = provider.get("provider_id")
    seen.add(pid)
    if provider.get("state") != "reserved":
        fail(f"{pid}: state must be reserved")
    if provider.get("activation") != "future_row_required":
        fail(f"{pid}: activation must stay future-only")
    if not provider.get("provider_kind"):
        fail(f"{pid}: missing provider_kind")
    if not provider.get("role"):
        fail(f"{pid}: missing role")
    operations = provider.get("operations")
    if not isinstance(operations, list) or not operations:
        fail(f"{pid}: missing operations")

if seen != expected:
    fail(f"provider ids mismatch: {sorted(seen)}")
PY

if rg -n 'hako_alloc_(install|replace)_allocator|allocator_replacement_hook|allocator_hook_activate|activate_allocator|HakoAllocatorReplacementHook|AllocatorReplacementHookBox|AllocatorHookPlan|HookPlan' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".activation_symbols 2>&1; then
  cat /tmp/"$TAG".activation_symbols >&2
  rm -f /tmp/"$TAG".activation_symbols
  fail "activation implementation symbols must stay absent in M65"
fi
rm -f /tmp/"$TAG".activation_symbols

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M65"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'select_allocator_provider|allocator_provider_select|allocator_provider_selection_env|NYASH_ALLOCATOR_PROVIDER' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".provider_selection 2>&1; then
  cat /tmp/"$TAG".provider_selection >&2
  rm -f /tmp/"$TAG".provider_selection
  fail "provider selection implementation/env toggle must stay absent in M65"
fi
rm -f /tmp/"$TAG".provider_selection

echo "[$TAG] ok"
