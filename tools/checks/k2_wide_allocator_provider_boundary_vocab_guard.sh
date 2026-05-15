#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-boundary-vocab"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh
source tools/checks/lib/phase_card_paths.sh
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-boundary-v0-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="$(guard_require_phase293x_card "$TAG" "293x-116-M64-ALLOCATOR-PROVIDER-BOUNDARY-VOCAB.md")"
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

allocator_provider_forbid_global_allocator "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
