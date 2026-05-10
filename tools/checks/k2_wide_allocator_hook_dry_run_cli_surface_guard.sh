#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-hook-dry-run-cli-surface"
cd "$ROOT_DIR"

CLI_FILE="src/cli/allocator_hook_dry_run.rs"
CLI_ARGS="src/cli/args.rs"
CLI_MOD="src/cli/mod.rs"
MAIN_FILE="src/main.rs"
RUNTIME_FILE="src/runtime/allocator_hook_dry_run.rs"
SSOT="docs/development/current/main/design/allocator-hook-dry-run-cli-surface-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-113-M61-ALLOCATOR-HOOK-DRY-RUN-CLI-SURFACE.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M61 allocator hook dry-run CLI surface"

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

require_file "$CLI_FILE"
require_file "$CLI_ARGS"
require_file "$CLI_MOD"
require_file "$MAIN_FILE"
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

require_text "$CLI_ARGS" "allocator-hook-dry-run"
require_text "$CLI_ARGS" "allocator-hook-plan"
require_text "$CLI_ARGS" "allocator-hook-proof"
require_text "$CLI_FILE" "maybe_run_allocator_hook_dry_run"
require_text "$CLI_FILE" "build_allocator_hook_dry_run_output"
require_text "$CLI_FILE" "std::fs::read_to_string"
require_text "$CLI_FILE" "would_activate=false"
require_text "$CLI_MOD" "maybe_run_allocator_hook_dry_run"
require_text "$MAIN_FILE" "maybe_run_allocator_hook_dry_run"
require_text "$SSOT" "Allocator Hook Dry-Run CLI Surface (SSOT)"
require_text "$TASKBOARD" '| `M61 allocator hook dry-run CLI surface` | `live-narrow` |'
require_text "$TASKBOARD" '84. `M61 allocator hook dry-run CLI surface`'
require_text "$PHASE_README" '`293x-113`'
require_text "$REAL_APP_TASKBOARD" '`293x-113` M61 allocator hook dry-run CLI surface'
require_text "$INDEX" "tools/checks/k2_wide_allocator_hook_dry_run_cli_surface_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_hook_dry_run_cli_surface_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_hook_dry_run_cli_surface_guard.sh"

cargo test -q allocator_hook_dry_run

if rg -n 'std::env|set_var|var_os|env_bool|env_string' "$CLI_FILE" >/tmp/"$TAG".env 2>&1; then
  cat /tmp/"$TAG".env >&2
  rm -f /tmp/"$TAG".env
  fail "CLI surface must not add hidden environment toggles"
fi
rm -f /tmp/"$TAG".env

if rg -n 'std::alloc|GlobalAlloc|#\[global_allocator\]|malloc|realloc|free\(' \
  "$CLI_FILE" "$CLI_ARGS" "$MAIN_FILE" "$RUNTIME_FILE" >/tmp/"$TAG".allocator 2>&1; then
  cat /tmp/"$TAG".allocator >&2
  rm -f /tmp/"$TAG".allocator
  fail "CLI dry-run must not add process allocator replacement behavior"
fi
rm -f /tmp/"$TAG".allocator

if rg -n 'allocator-hook|allocator_hook|allocator.*hook|hook.*allocator' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator hook dry-run CLI behavior"
fi
rm -f /tmp/"$TAG".runner

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan|allocator_hook_activate|activate_allocator' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  fail "allocator hook/facade/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc

echo "[$TAG] ok"
