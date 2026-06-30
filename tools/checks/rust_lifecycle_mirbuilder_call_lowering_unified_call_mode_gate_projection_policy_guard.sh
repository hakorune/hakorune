#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-call-lowering-unified-call-mode-gate-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_call_lowering_unified_call_mode_gate_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-unified-call-mode-gate-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1902-MIRBUILDER-CALL-LOWERING-UNIFIED-CALL-MODE-GATE-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-unified-call-mode-gate-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1902-MIRBUILDER-CALL-LOWERING-UNIFIED-CALL-MODE-GATE-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-CALL-LOWERING-UNIFIED-CALL-MODE-GATE-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderCallLoweringUnifiedCallModeGateProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["selected_feature_subcluster_id"] != "UnifiedCallModeGate":
    raise SystemExit("selected feature subcluster drift")
if fixture["input_state"]["source_count"] != 1:
    raise SystemExit("source count drift")

surface = fixture["source_surface"]
if surface["symbol"] != "is_unified_call_enabled":
    raise SystemExit("source symbol drift")
if surface["config_accessor"] != "builder_unified_call_mode":
    raise SystemExit("config accessor drift")
if surface["env_var"] != "NYASH_MIR_UNIFIED_CALL":
    raise SystemExit("env var drift")
for marker in [
    "builder_unified_call_mode",
    "default ON during development; explicit opt-out supported",
    '"0"',
    '"false"',
    '"off"',
    "NYASH_MIR_UNIFIED_CALL",
    'env_string("NYASH_MIR_UNIFIED_CALL")',
]:
    if marker not in surface["source_markers"]:
        raise SystemExit(f"source marker missing: {marker}")

axes = fixture["selection_axes"]
expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.config_gate",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if axes != expected_axes:
    raise SystemExit(f"selection axes drift: {axes}")

policy = fixture["selected_policy"]
if policy["policy"] != "KeepParentConfigGate":
    raise SystemExit("policy drift")
if policy["owner_edge"] != "mirbuilder::call_lowering_unified_call_mode_gate":
    raise SystemExit("owner edge drift")
if policy["config_authority"] != "src/config/env/builder_flags.rs::builder_unified_call_mode":
    raise SystemExit("config authority drift")
if policy["projection_surface_selected"] is not False:
    raise SystemExit("projection surface must not be selected")
if policy["hako_config_gate_selected"] is not False:
    raise SystemExit("Hako config gate must not be selected")
if policy["new_env_flag_selected"] is not False:
    raise SystemExit("new env flag must not be selected")

decision = fixture["decision"]
if decision["kind"] != "KeepParentOwner":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-CALL-LOWERING-PURE-METHOD-CATALOG-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "manual_family_selection",
    "projection_surface_selected",
    "hako_config_gate_selected",
    "new_env_flag",
    "runtime_or_projection_policy_by_name",
    "hako_generation",
    "hako_adopted_decision",
    "native_seed_materialization",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-call-lowering-unified-call-mode-gate-projection-policy-v0
subcluster=UnifiedCallModeGate
policy=KeepParentConfigGate
config_authority=src/config/env/builder_flags.rs::builder_unified_call_mode
projection_surface_selected=0
hako_config_gate_selected=0
new_env_flag=0
selected_next_card=MIRBUILDER-CALL-LOWERING-PURE-METHOD-CATALOG-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
