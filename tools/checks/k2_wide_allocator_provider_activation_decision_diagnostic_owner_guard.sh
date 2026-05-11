#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-provider-activation-decision-diagnostic-owner"
cd "$ROOT_DIR"
source tools/checks/lib/allocator_provider_forbidden_patterns.sh

SSOT="docs/development/current/main/design/allocator-provider-activation-decision-diagnostic-owner-ssot.md"
FIXTURE="docs/development/current/main/design/allocator-provider-activation-decision-v0.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-141-M88-ALLOCATOR-PROVIDER-ACTIVATION-DECISION-DIAGNOSTIC-OWNER.md"
M87_GUARD="tools/checks/k2_wide_allocator_provider_activation_decision_fixture_contract_guard.sh"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

OWNER_PATH="src/runtime/allocator_provider_activation_decision.rs"

echo "[$TAG] checking M88 allocator provider activation decision diagnostic owner"

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
require_file "$CARD"
require_file "$M87_GUARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Provider Activation Decision Diagnostic Owner (SSOT)"
require_text "$SSOT" "$OWNER_PATH"
require_text "$SSOT" "validate_allocator_provider_activation_decision(decision_bundle)"
require_text "$SSOT" "activation_decision_allowed = false"
require_text "$SSOT" "would_select_provider = false"
require_text "$SSOT" "would_consume_proof = false"
require_text "$SSOT" "would_prepare_rollback = false"
require_text "$SSOT" "would_open_activation_gate = false"
require_text "$SSOT" "would_install_hook = false"
require_text "$SSOT" "would_replace_process_allocator = false"
require_text "$SSOT" "would_activate = false"
require_text "$SSOT" 'M87 must not block future diagnostic owner/type names in `src/`'
require_text "$FIXTURE" "decision_surface_owner = \"$OWNER_PATH\""
require_text "$FIXTURE" 'activation_decision_allowed = false'
require_text "$FIXTURE" 'would_activate = false'
require_text "$CARD" "293x-141 M88 Allocator Provider Activation Decision Diagnostic Owner"
require_text "$CARD" "$OWNER_PATH"
require_text "$CARD" "This is docs/fixture/guard only."
require_text "$INDEX" "tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_owner_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_owner_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_owner_guard.sh"

python3 - <<'PY' "$FIXTURE" "$OWNER_PATH"
import sys
import tomllib
from pathlib import Path

path = Path(sys.argv[1])
owner_path = sys.argv[2]
data = tomllib.loads(path.read_text())

def fail(message: str) -> None:
    print(f"[k2-wide-allocator-provider-activation-decision-diagnostic-owner][fail] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("decision_surface_owner") != owner_path:
    fail("decision_surface_owner must name the future runtime diagnostic owner")
if data.get("activation_decision_allowed") is not False:
    fail("activation_decision_allowed must remain false")
for key in [
    "would_select_provider",
    "would_consume_proof",
    "would_prepare_rollback",
    "would_open_activation_gate",
    "would_install_hook",
    "would_replace_process_allocator",
    "would_activate",
]:
    if data.get(key) is not False:
        fail(f"{key} must remain false")
if data.get("activation") != "future_row_required":
    fail("activation must require a future row")
PY

if rg -n "decision_surface_owner.*future_row_required|M87 fixture contract must not add activation decision runtime or CLI code|--allocator-provider-activation-decision\\|activation_decision\\|ActivationDecision" "$M87_GUARD" >/tmp/"$TAG".m87_pin 2>&1; then
  cat /tmp/"$TAG".m87_pin >&2
  rm -f /tmp/"$TAG".m87_pin
  fail "M87 guard must not pin the pre-owner state or block future activation decision diagnostics"
fi
rm -f /tmp/"$TAG".m87_pin

allocator_provider_forbid_activation_gate_open "$TAG"

allocator_provider_forbid_selection "$TAG"

allocator_provider_forbid_proof_consumption "$TAG"

allocator_provider_forbid_rollback_preparation "$TAG"

allocator_provider_forbid_hook_activation "$TAG"

allocator_provider_forbid_global_allocator "$TAG"

allocator_provider_forbid_inc_matchers "$TAG"

echo "[$TAG] ok"
