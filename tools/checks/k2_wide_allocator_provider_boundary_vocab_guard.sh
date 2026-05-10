#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-boundary-vocab"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-provider-boundary-v0-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-116-M64-ALLOCATOR-PROVIDER-BOUNDARY-VOCAB.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M64 allocator provider boundary vocabulary"

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
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Boundary v0 (SSOT)"
require_text "$SSOT" "native_system_malloc"
require_text "$SSOT" "native_mimalloc"
require_text "$SSOT" "hako_model_allocator"
require_text "$SSOT" "debug_guarded_allocator"
require_text "$SSOT" "M64 keeps these inactive"
require_text "$CARD" "M64 Allocator Provider Boundary Vocabulary"
require_text "$TASKBOARD" '| `M64 allocator provider boundary vocabulary` | `live-docs` |'
require_text "$TASKBOARD" '87. `M64 allocator provider boundary vocabulary`'
require_text "$PHASE_README" '`293x-116`'
require_text "$REAL_APP_TASKBOARD" '`293x-116` M64 allocator provider boundary vocabulary'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_boundary_vocab_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_boundary_vocab_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_boundary_vocab_guard.sh"

if rg -n 'hako_alloc_(install|replace)_allocator|allocator_replacement_hook|allocator_hook_activate|activate_allocator|HakoAllocatorReplacementHook|AllocatorReplacementHookBox|AllocatorHookPlan|HookPlan' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".activation_symbols 2>&1; then
  cat /tmp/"$TAG".activation_symbols >&2
  rm -f /tmp/"$TAG".activation_symbols
  fail "activation implementation symbols must stay absent in M64"
fi
rm -f /tmp/"$TAG".activation_symbols

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M64"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'select_allocator_provider|allocator_provider_select|allocator_provider_selection_env|NYASH_ALLOCATOR_PROVIDER' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".provider_selection 2>&1; then
  cat /tmp/"$TAG".provider_selection >&2
  rm -f /tmp/"$TAG".provider_selection
  fail "provider selection implementation/env toggle must stay absent in M64"
fi
rm -f /tmp/"$TAG".provider_selection

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan|allocator_hook_activate|activate_allocator|native_mimalloc|native_system_malloc' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  fail "allocator provider/hook/facade/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc

echo "[$TAG] ok"
