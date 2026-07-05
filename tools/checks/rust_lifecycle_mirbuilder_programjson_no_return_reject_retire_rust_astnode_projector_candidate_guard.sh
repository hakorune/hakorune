#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-no-return-reject-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

DECISION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-no-return-reject-retire-rust-astnode-projector-candidate-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_loop_cond_continue_with_return_no_return_reject_snapshot_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$DECISION" "$PARITY_GATE"

python3 - "$DECISION" <<'PY'
import hashlib
import json
import sys
from pathlib import Path

decision = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))

def need(condition, message):
    if not condition:
        raise SystemExit(message)

def sha256(path):
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()

need(decision.get("schema_version") == 0, "bad schema_version")
need(
    decision.get("kind")
    == "MirBuilderProgramJsonNoReturnRejectRustAstNodeProjectorRetireCandidateV1",
    "bad kind",
)
need(
    decision.get("token")
    == "MIRBUILDER-PROGRAMJSON-NO-RETURN-REJECT-RETIRE-RUST-ASTNODE-PROJECTOR-CANDIDATE-001",
    "bad token",
)

state = decision.get("input_state") or {}
for key in ["hako_snapshot_source", "parity_fixture", "parity_gate"]:
    path = Path(state.get(key) or "")
    need(path.exists(), f"missing input path: {key}")
    need(sha256(path) == state.get(f"{key}_hash"), f"hash drift: {key}")

scope = decision.get("retire_candidate_scope") or {}
need(scope.get("retire_candidate") == "LoopCondContinueNoReturnRejectTokenSnapshotV1", "bad retire candidate")
need(scope.get("shape_scope") == "LoopCondContinueNoReturnRejectV1", "bad shape scope")
need(scope.get("programjson_snapshot_owner") == "ProgramJsonLoopCondContinueWithReturnSnapshotV1", "bad owner")
need(scope.get("rust_projector_runtime_dependency_removed") == 0, "runtime dependency removal must remain 0")
need(scope.get("rust_projector_oracle_only") == 1, "oracle-only candidate must be 1")
need(scope.get("full_astnode_projector_retired") == 0, "full projector retired must remain 0")

criteria = decision.get("criteria") or {}
need(criteria.get("programjson_snapshot_parity_gate") == "Green", "parity gate must be Green")
need(criteria.get("programjson_route_traverses_programjson") == 1, "ProgramJSON traversal proof missing")
need(criteria.get("runtime_path_uses_programjson_snapshot") == 1, "runtime snapshot proof missing")
need(criteria.get("rust_astnode_projector_runtime_dependency_removed") == 0, "runtime removal must not be claimed")
need(criteria.get("rust_astnode_projector_kept_for_oracle_generation") == 1, "oracle retention missing")

row = decision.get("decision") or {}
need(row.get("kind") == "RetireCandidateScoped", "bad decision kind")
need(
    row.get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-NEXT-SHAPE-SELECTION-002",
    "bad next card",
)

claims = decision.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "hako_adopted_decision",
    "rust_astnode_projector_retired",
    "rust_astnode_projector_fully_retired",
    "full_astnode_projector_retired",
    "programjson_full_parser_claim",
    "programjson_all_shapes_supported",
    "recipe_matching_migrated",
    "route_selection_migration",
    "backend_lowering_migration",
    "mir_mutation_migration",
    "id_allocation_migration",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")
PY

bash "$PARITY_GATE" >/dev/null

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-no-return-reject-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-NO-RETURN-REJECT-RETIRE-RUST-ASTNODE-PROJECTOR-CANDIDATE-001
retire_candidate=LoopCondContinueNoReturnRejectTokenSnapshotV1
shape_scope=LoopCondContinueNoReturnRejectV1
decision=RetireCandidateScoped
parity_gate=green
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
source_selfhost_claim=0
hako_adopted_decision=0
programjson_full_parser_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-NEXT-SHAPE-SELECTION-002
summary=ok
REPORT
