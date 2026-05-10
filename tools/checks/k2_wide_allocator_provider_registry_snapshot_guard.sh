#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-registry-snapshot"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-registry-snapshot-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml"
ACTIVATION_ENTRY_SSOT="docs/development/current/main/design/allocator-provider-activation-entry-contract-ssot.md"
MANIFEST="docs/development/current/main/design/allocator-provider-manifest-v0.toml"
TASK_BREAKDOWN="docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-129-M77-ALLOCATOR-PROVIDER-REGISTRY-SNAPSHOT.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M77 allocator provider registry snapshot"

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
require_file "$ACTIVATION_ENTRY_SSOT"
require_file "$MANIFEST"
require_file "$TASK_BREAKDOWN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Registry Snapshot (SSOT)"
require_text "$SSOT" "allocator-provider-registry-snapshot-v0.toml"
require_text "$SSOT" "[allocator-provider/registry-snapshot-missing]"
require_text "$SSOT" "[allocator-provider/registry-provider-missing]"
require_text "$SSOT" "[allocator-provider/registry-capability-missing]"
require_text "$SSOT" "would_build_registry = false"
require_text "$SSOT" "would_select_provider = false"
require_text "$SSOT" "would_activate = false"
require_text "$ACTIVATION_ENTRY_SSOT" "M77 | registry snapshot diagnostic shape"
require_text "$MANIFEST" 'provider_id = "native_mimalloc"'
require_text "$TASK_BREAKDOWN" "M77 | registry snapshot diagnostic shape"
require_text "$TASKBOARD" '| `M77 allocator provider registry snapshot` | `live-docs` |'
require_text "$TASKBOARD" '100. `M77 allocator provider registry snapshot`'
require_text "$PHASE_README" '`293x-129`'
require_text "$REAL_APP_TASKBOARD" '[x] `293x-129` M77 allocator provider registry snapshot'
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_registry_snapshot_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_registry_snapshot_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_registry_snapshot_guard.sh"

python3 - <<'PY' "$FIXTURE"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(f"[k2-wide-allocator-provider-registry-snapshot][fail] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("schema_version") != "allocator_provider_registry_snapshot_v0":
    fail("schema_version must be allocator_provider_registry_snapshot_v0")
if data.get("status") != "reserved":
    fail("status must be reserved")
if data.get("active") is not False:
    fail("active must be false")
if data.get("registry_owner") != "src/runtime/allocator_provider_registry.rs":
    fail("registry_owner must name the future runtime owner")
if data.get("provider_manifest_input") != "allocator_provider_manifest_report":
    fail("provider_manifest_input must be allocator_provider_manifest_report")
if data.get("provider_readiness_input") != "allocator_provider_readiness_preflight_report":
    fail("provider_readiness_input must be allocator_provider_readiness_preflight_report")
if data.get("provider_selection") != "inactive":
    fail("provider_selection must be inactive")
if data.get("would_build_registry") is not False:
    fail("would_build_registry must be false")
if data.get("would_select_provider") is not False:
    fail("would_select_provider must be false")
if data.get("would_activate") is not False:
    fail("would_activate must be false")
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")
if data.get("diagnostic") != "[allocator-provider/registry-snapshot-missing]":
    fail("unexpected snapshot diagnostic")
if data.get("missing_provider_diagnostic") != "[allocator-provider/registry-provider-missing]":
    fail("unexpected missing provider diagnostic")
if data.get("missing_capability_diagnostic") != "[allocator-provider/registry-capability-missing]":
    fail("unexpected missing capability diagnostic")

required = [
    "provider_manifest_ready",
    "provider_readiness_preflight_ready",
    "provider_entries_nonempty",
    "provider_ids_reserved_set",
    "provider_operations_nonempty",
    "registry_owner_named",
    "no_hidden_environment_toggle",
    "no_implicit_manifest_discovery",
    "no_app_or_facade_name_matching",
]
facts = data.get("required_registry_snapshot_facts")
if not isinstance(facts, list):
    fail("required_registry_snapshot_facts must be a list")
for fact in required:
    if fact not in facts:
        fail(f"missing registry snapshot fact: {fact}")

expected_ids = [
    "native_system_malloc",
    "native_mimalloc",
    "hako_model_allocator",
    "debug_guarded_allocator",
]
entries = data.get("entries")
if not isinstance(entries, list) or len(entries) != len(expected_ids):
    fail("entries must contain the reserved provider set")
ids = [entry.get("provider_id") for entry in entries if isinstance(entry, dict)]
if ids != expected_ids:
    fail("entries must preserve reserved provider order")
for entry in entries:
    if entry.get("state") != "reserved":
        fail("entry state must be reserved")
    if entry.get("activation") != "future_row_required":
        fail("entry activation must require a future row")
    operations = entry.get("operations")
    if not isinstance(operations, list) or not operations:
        fail("entry operations must be nonempty")
PY


allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

if rg -n 'allocator-provider|allocator_provider|provider.*allocator|allocator.*provider' src/runner -g '*.rs' >/tmp/"$TAG".runner 2>&1; then
  cat /tmp/"$TAG".runner >&2
  rm -f /tmp/"$TAG".runner
  fail "runner must not own allocator provider registry behavior"
fi
rm -f /tmp/"$TAG".runner

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
