#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-manifest-parser"
cd "$ROOT_DIR"

RUNTIME_FILE="src/runtime/allocator_provider_manifest.rs"
SSOT="docs/development/current/main/design/allocator-provider-manifest-diagnostic-parser-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-119-M67-ALLOCATOR-PROVIDER-MANIFEST-PARSER.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M67 allocator provider manifest parser"

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

require_file "$RUNTIME_FILE"
require_file "$SSOT"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$RUNTIME_FILE" "AllocatorProviderManifestStatus"
require_text "$RUNTIME_FILE" "AllocatorProviderManifestReport"
require_text "$RUNTIME_FILE" "parse_allocator_provider_manifest_text"
require_text "$RUNTIME_FILE" "DIAG_PROVIDER_MANIFEST_READY"
require_text "$RUNTIME_FILE" "DIAG_PROVIDER_MANIFEST_MISSING"
require_text "$RUNTIME_FILE" "missing_facts"
require_text "$RUNTIME_FILE" "would_select_provider: false"
require_text "$RUNTIME_FILE" "provider_manifest_reserved_fixture_reports_ready_without_selection"
require_text "$SSOT" "Allocator Provider Manifest Diagnostic Parser (SSOT)"
require_text "$SSOT" "would_select_provider = false"
require_text "$TASKBOARD" '| `M67 allocator provider manifest parser` | `live-narrow` |'
require_text "$TASKBOARD" '90. `M67 allocator provider manifest parser`'
require_text "$PHASE_README" '`293x-119`'
require_text "$REAL_APP_TASKBOARD" '`293x-119` M67 allocator provider manifest parser'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_manifest_parser_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_manifest_parser_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_manifest_parser_guard.sh"

cargo test -q allocator_provider_manifest

if rg -n 'std::env|std::fs|read_to_string|var_os|std::alloc|GlobalAlloc|#\[global_allocator\]' \
  "$RUNTIME_FILE" >/tmp/"$TAG".forbidden_runtime 2>&1; then
  cat /tmp/"$TAG".forbidden_runtime >&2
  rm -f /tmp/"$TAG".forbidden_runtime
  fail "provider parser must not add env/fs/allocator replacement behavior"
fi
rm -f /tmp/"$TAG".forbidden_runtime

if rg -n 'AllocatorProviderRegistry|allocator_provider_registry|select_allocator_provider|allocator_provider_select|allocator_provider_selection_env|NYASH_ALLOCATOR_PROVIDER' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".provider_selection 2>&1; then
  cat /tmp/"$TAG".provider_selection >&2
  rm -f /tmp/"$TAG".provider_selection
  fail "provider registry/selection implementation must stay absent in M67"
fi
rm -f /tmp/"$TAG".provider_selection

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M67"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan|allocator_hook_activate|activate_allocator|native_mimalloc|native_system_malloc' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  fail "allocator provider/hook/facade/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc

echo "[$TAG] ok"
