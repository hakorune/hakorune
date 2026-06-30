#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-generic-loop-body-check-tail-control-flow-probe-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_generic_loop_body_check_tail_control_flow_probe_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-tail-control-flow-probe-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1914-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TAIL-CONTROL-FLOW-PROBE-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-tail-control-flow-probe-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1914-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TAIL-CONTROL-FLOW-PROBE-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TAIL-CONTROL-FLOW-PROBE-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderGenericLoopBodyCheckTailControlFlowProbeProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["selected_subcluster_id"] != "TailControlFlowProbe":
    raise SystemExit("selected subcluster drift")
if state["source_count"] != 1:
    raise SystemExit("source count drift")

surface = fixture["source_surfaces"][0]
if surface["symbol"] != "has_control_flow_after_step":
    raise SystemExit("source surface drift")
if surface["probe_role"] != "tail_control_flow_scan":
    raise SystemExit("probe role drift")
if surface["return_type"] != "bool":
    raise SystemExit("return type drift")

descriptor = fixture["probe_descriptor"]
if descriptor["descriptor_id"] != "generic_loop_body_check_tail_control_flow_probe_v1":
    raise SystemExit("descriptor id drift")
if descriptor["source_extraction"] != "rust_tail_statement_scan":
    raise SystemExit("source extraction drift")
if descriptor["scan_range"] != "body[(step_index + 1)..]":
    raise SystemExit("scan range drift")
if descriptor["returns"] != "bool":
    raise SystemExit("return descriptor drift")
if descriptor["control_flow_predicates"] != [
    "is_exit_if(stmt)",
    "ASTNode::Break",
    "ASTNode::Continue",
    "ASTNode::Return",
]:
    raise SystemExit("control-flow predicates drift")

policy = fixture["selected_policy"]
if policy["policy"] != "SourceExtractedTailControlFlowProbeDescriptor":
    raise SystemExit("selected policy drift")
if policy["probe_descriptor_selected"] is not True:
    raise SystemExit("probe descriptor must be selected")
if policy["strict_reject_semantics_selected"] is not False:
    raise SystemExit("strict reject semantics must not be selected")
if policy["hako_projection_selected"] is not False:
    raise SystemExit("Hako projection must not be selected")

decision = fixture["decision"]
if decision["kind"] != "SelectProbeDescriptorPolicy":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-IN-BODY-STEP-VALIDATION-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
if claims.get("probe_descriptor_selected") != 1:
    raise SystemExit("probe descriptor selected claim must be 1")
for key in [
    "manual_family_selection",
    "strict_reject_semantics_selected",
    "hako_projection_selected",
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

provenance = fixture["provenance"]
if provenance["tool_role"] != "FactsAdapterGuardOrchestrator":
    raise SystemExit("tool role drift")
if provenance["semantic_projection_inference"] != 0:
    raise SystemExit("tool must not infer semantic projection")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-generic-loop-body-check-tail-control-flow-probe-projection-policy-v0
subcluster=TailControlFlowProbe
source_count=1
policy=SourceExtractedTailControlFlowProbeDescriptor
probe_descriptor_selected=1
strict_reject_semantics_selected=0
hako_projection_selected=0
selected_next_card=MIRBUILDER-GENERIC-LOOP-BODY-CHECK-IN-BODY-STEP-VALIDATION-PROJECTION-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
