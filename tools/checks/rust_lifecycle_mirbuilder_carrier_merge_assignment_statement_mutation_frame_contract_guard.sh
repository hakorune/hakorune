#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-carrier-merge-assignment-statement-mutation-frame-contract-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-merge-assignment-statement-mutation-frame-contract-v0.json"
INPUT_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-merge-assignment-statement-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1868-MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-MUTATION-FRAME-CONTRACT-001.md"
SOURCE="$ROOT_DIR/src/mir/builder/control_flow/plan/features/carrier_merge.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$INPUT_FIXTURE" "$CARD" "$SOURCE"

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-merge-assignment-statement-mutation-frame-contract-v0.json").read_text())
input_fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-merge-assignment-statement-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1868-MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-MUTATION-FRAME-CONTRACT-001.md").read_text()
source = Path("src/mir/builder/control_flow/plan/features/carrier_merge.rs").read_text()

token = "MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-MUTATION-FRAME-CONTRACT-001"
if fixture.get("kind") != "MirBuilderCarrierMergeAssignmentStatementMutationFrameContractV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token or token not in card:
    raise SystemExit("token mismatch")
if input_fixture.get("decision", {}).get("selected_next_card") != token:
    raise SystemExit("input projection policy does not point to this contract")
if "pub(in crate::mir::builder) fn lower_assignment_stmt" not in source:
    raise SystemExit("source signature missing")

contract = fixture.get("mutation_frame_contract") or {}
if set(contract.get("state_outputs") or []) != {
    "current_bindings",
    "carrier_updates",
    "builder.variable_ctx.variable_map",
}:
    raise SystemExit("state outputs drift")
if contract.get("read_only_inputs") != ["carrier_phis"]:
    raise SystemExit("read-only inputs drift")
if len(contract.get("mutation_order") or []) != 6:
    raise SystemExit("mutation order must have 6 steps")

last = -1
for marker in fixture.get("source_order_markers") or []:
    index = source.find(marker)
    if index < 0:
        raise SystemExit(f"source order marker missing: {marker}")
    if index <= last:
        raise SystemExit(f"source order marker out of order: {marker}")
    last = index

if source.count("builder.variable_ctx.variable_map.insert") < 2:
    raise SystemExit("expected reseal and publish variable_map inserts")
for forbidden in [
    "carrier_phis.insert",
    "carrier_phis.remove",
    "carrier_phis.clear",
]:
    if forbidden in source:
        raise SystemExit(f"carrier_phis must be read-only: {forbidden}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectHakoShadowParity":
    raise SystemExit("decision kind drift")
if decision.get("selected_next_card") != "MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-SHADOW-PARITY-001":
    raise SystemExit("selected next card drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "hako_generation",
    "hako_shadow_projector_selected",
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
output_contract=rust-lifecycle-mirbuilder-carrier-merge-assignment-statement-mutation-frame-contract-v0
mutation_frame_contract_ready=1
decision=SelectHakoShadowParity
selected_next_card=MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-SHADOW-PARITY-001
source_selfhost_claim=0
runtime_fallback=0
summary=ok
REPORT
