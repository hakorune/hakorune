#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-dispatch-support-shape-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-dispatch-support-shape-retire-rust-astnode-projector-candidate-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_dispatch_support_shape_scan_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PARITY_GATE"

bash "$PARITY_GATE" >/dev/null

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))

if fixture.get("kind") != "MirBuilderProgramJsonDispatchSupportShapeRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-DISPATCH-SUPPORT-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

evidence = fixture.get("evidence") or {}
expected_hashes = {
    "hako_snapshot_source_hash": "sha256:17debe7c9bfe427b3921c164cf1c43a1e26a6f6780049ba5199d2e642bc13c5d",
    "parity_fixture_hash": "sha256:2f548052d8fea406b87495d6199b76e95ba5559a588e34fabd7b10e371ac8139",
    "parity_gate_hash": "sha256:dab7ec0eafc41d0881c5151b67eceded0922e734f85d8cc9bcf882b5c3b26496",
}
for key, expected in expected_hashes.items():
    if evidence.get(key) != expected:
        raise SystemExit(f"hash drift: {key}")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "DispatchSupportShapeSnapshotV1":
    raise SystemExit("bad retire candidate")
if scope.get("shape_scope") != "covered ProgramJSON statement dispatch-support rows":
    raise SystemExit("bad shape scope")
if scope.get("programjson_snapshot_owner") != "ProgramJsonDispatchSupportShapeScanV1":
    raise SystemExit("bad snapshot owner")

rows = scope.get("covered_rows") or []
expected_rows = [
    "top_local_dispatchable",
    "top_loop_dispatchable",
    "top_try_dispatchable_cleanup",
    "top_break_unsupported_exit",
    "top_context_scope_unsupported",
    "top_extern_unsupported",
    "top_function_declaration_kind_only_unsupported",
    "top_looprange_unsupported_other",
    "if_then_return_else_break",
    "if_then_task_scope_else_local",
]
if rows != expected_rows:
    raise SystemExit("covered rows drift")

if scope.get("rust_projector_runtime_dependency_removed") != 0:
    raise SystemExit("runtime dependency removal must stay unclaimed")
if scope.get("rust_projector_oracle_only") != 1:
    raise SystemExit("rust oracle marker missing")
if scope.get("full_astnode_projector_retired") != 0:
    raise SystemExit("full ASTNode projector retirement must stay unclaimed")

criteria = fixture.get("criteria") or {}
if criteria.get("programjson_snapshot_parity_gate") != "Green":
    raise SystemExit("parity gate must be green")
if criteria.get("programjson_route_traverses_programjson") != 1:
    raise SystemExit("programjson traversal marker missing")
if criteria.get("programjson_route_uses_string_only_facade") != 0:
    raise SystemExit("string-only facade must remain 0")
if criteria.get("covered_row_count") != 10:
    raise SystemExit("covered row count drift")
if criteria.get("dispatch_support_added") != 0:
    raise SystemExit("dispatch support must stay unclaimed")
if criteria.get("unsupported_stmt_resolved") != 0:
    raise SystemExit("unsupported stmt resolution must stay unclaimed")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "mir_mutation",
    "id_allocation",
    "backend_lowering",
    "route_selection",
    "full_recipe_matcher_execution",
    "dispatch_support_added",
    "unsupported_stmt_resolved",
    "hako_adopted_decision",
    "programjson_full_parser_claim",
    "programjson_all_shapes_supported",
    "rust_astnode_projector_fully_retired",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "RetireCandidateScoped":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION":
    raise SystemExit("bad selected next card")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-dispatch-support-shape-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-DISPATCH-SUPPORT-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=DispatchSupportShapeSnapshotV1
shape_scope=covered ProgramJSON statement dispatch-support rows
covered_rows=10
decision=RetireCandidateScoped
parity_gate=green
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
dispatch_support_added=0
unsupported_stmt_resolved=0
source_selfhost_claim=0
hako_adopted_decision=0
programjson_full_parser_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
summary=ok
REPORT
