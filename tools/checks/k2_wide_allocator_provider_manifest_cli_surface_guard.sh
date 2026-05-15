#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-manifest-cli-surface"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

CLI_FILE="src/cli/allocator_provider_manifest.rs"
CLI_ARGS="src/cli/args.rs"
CLI_MOD="src/cli/mod.rs"
CLI_DIAGNOSTIC_OUTPUT="src/cli/diagnostic_output.rs"
MAIN_FILE="src/main.rs"
RUNTIME_FILE="src/runtime/allocator_provider_manifest.rs"
SSOT="docs/development/current/main/design/allocator-provider-manifest-cli-surface-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-120-M68-ALLOCATOR-PROVIDER-MANIFEST-CLI-SURFACE.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M68 allocator provider manifest CLI surface"

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
require_file "$CLI_DIAGNOSTIC_OUTPUT"
require_file "$MAIN_FILE"
require_file "$RUNTIME_FILE"
require_file "$SSOT"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$CLI_ARGS" "allocator-provider-manifest"
require_text "$CLI_FILE" "maybe_run_allocator_provider_manifest_diagnostic"
require_text "$CLI_FILE" "build_allocator_provider_manifest_output"
require_text "$CLI_FILE" "read_labeled_file"
require_text "$CLI_FILE" "would_select_provider=false"
require_text "$CLI_MOD" "maybe_run_allocator_provider_manifest_diagnostic"
require_text "$CLI_MOD" "mod diagnostic_output"
require_text "$CLI_DIAGNOSTIC_OUTPUT" "std::fs::read_to_string"
require_text "$CLI_DIAGNOSTIC_OUTPUT" "finish_result"
require_text "$MAIN_FILE" "maybe_run_allocator_provider_manifest_diagnostic"
require_text "$SSOT" "Allocator Provider Manifest CLI Surface (SSOT)"
require_text "$SSOT" "hakorune --allocator-provider-manifest <PROVIDER_MANIFEST_TOML>"
require_text "$TASKBOARD" '| `M68 allocator provider manifest CLI surface` | `live-narrow` |'
require_text "$TASKBOARD" '91. `M68 allocator provider manifest CLI surface`'
require_text "$PHASE_README" '`293x-120`'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_manifest_cli_surface_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_manifest_cli_surface_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_manifest_cli_surface_guard.sh"

cargo test -q allocator_provider_manifest

if rg -n 'std::env|set_var|var_os|env_bool|env_string' "$CLI_FILE" >/tmp/"$TAG".env 2>&1; then
  cat /tmp/"$TAG".env >&2
  rm -f /tmp/"$TAG".env
  fail "provider manifest CLI surface must not add hidden environment toggles"
fi
rm -f /tmp/"$TAG".env

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider manifest CLI behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
