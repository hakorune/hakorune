#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-ast-wire-observation-oracle-v0"
ORACLE="$ROOT/src/analysis/bounded_body_snapshot_v0/ast_wire_oracle_v0"
MOD="$ROOT/src/analysis/bounded_body_snapshot_v0/mod.rs"
DIRECT_TEST="$ROOT/src/backend/mir_interpreter/strict_json_session/tests/tests_snapshot/ast_oracle.rs"

cd "$ROOT"
cargo test -q --lib analysis::bounded_body_snapshot_v0::ast_wire_oracle_v0
cargo test -q --features vm-reference --lib \
  backend::mir_interpreter::strict_json_session::tests::tests_snapshot::ast_oracle

python3 - "$ORACLE" "$MOD" "$DIRECT_TEST" <<'PY'
import sys
from pathlib import Path

oracle_root, mod_path, direct_test_path = map(Path, sys.argv[1:])
sources = sorted(oracle_root.glob("*.rs"))
joined = "\n".join(path.read_text(encoding="utf-8") for path in sources)
mod = mod_path.read_text(encoding="utf-8")
direct = direct_test_path.read_text(encoding="utf-8")
for path in [*sources, direct_test_path]:
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")

if "#[cfg(test)]\npub(crate) mod ast_wire_oracle_v0;" not in mod:
    raise SystemExit("AST oracle must remain test-only")
for forbidden in (
    "crate::stage1", "program_json_v0::", "serde_json", "ValidatedProgramV0BodyView",
    "program_v0_snapshot_witness", "crate::runner", "crate::mir", "MIRBuilder",
    "try_extract_loop_feature_facts", "SnapshotDirectReaderFixtureV0Box",
):
    if forbidden in joined:
        raise SystemExit(f"oracle dependency boundary violated: {forbidden}")
if joined.count("_ =>") != 2 or "other =>" in joined:
    raise SystemExit("AST oracle wildcard inventory drift")

for needle in (
    "ASTNode::Local", "ASTNode::Assignment", "ASTNode::Print", "ASTNode::If",
    "ASTNode::Loop", "ASTNode::LoopRange", "ASTNode::Return", "ASTNode::Break",
    "ASTNode::Continue", "LiteralValue::TypedInteger", "ASTNode::BinaryOp",
    'name == "env.console.log"', "checked_neg", "unsupported.source_wire_projection",
    "source_loss_equivalence_classes_are_exact",
    "local_binding_expansion_matches_multiple_wire_locals",
    "all_operators_use_the_closed_wire_partition",
    "context_sensitive_and_source_only_shapes_are_explicit_unsupported",
):
    if needle not in joined:
        raise SystemExit(f"missing AST oracle contract: {needle}")
for needle in (
    "emit_program_json_v0_for_strict_authority_source",
    "observe_ast_body_v0",
    "rust_snapshot(&program_json)",
    "snapshot_signature(&oracle)",
    "SnapshotDirectReaderFixtureV0Box.snapshot_signature/2",
):
    if needle not in direct:
        raise SystemExit(f"missing independent parity proof: {needle}")

print("input=canonical_ASTNode_body")
print("output=BoundedBodyAnalysisSnapshotV0")
print("test_only_oracle=1")
print("program_json_generation=0")
print("serializer_import=0")
print("mir_planner_runtime_dependency=0")
print("loss_equivalence_fixtures=green")
print("all_18_operators=green")
print("context_sensitive_projection=explicit_unsupported")
print("authoritative_serializer_hhako_exact_parity=green")
print("source_files_under_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"
