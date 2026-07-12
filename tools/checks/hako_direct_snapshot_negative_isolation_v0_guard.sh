#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-direct-snapshot-negative-isolation-v0"
DIR="$ROOT/lang/src/compiler/analysis/bounded_body_snapshot"
DIRECT_FIXTURE="$ROOT/tools/checks/fixtures/bounded_body_snapshot_direct_reader_v0.hako"
SESSION="$ROOT/src/backend/mir_interpreter/strict_json_session.rs"
SNAPSHOT_TESTS="$ROOT/src/backend/mir_interpreter/strict_json_session/tests/tests_snapshot"

cd "$ROOT"
bash tools/checks/hako_direct_snapshot_parity_v0_guard.sh
cargo test -q --lib analysis::bounded_body_snapshot_v0::tests::witness
cargo test -q --lib mir::strict_json_tree_backend_capability

python3 - "$DIR" "$DIRECT_FIXTURE" "$SESSION" "$SNAPSHOT_TESTS" <<'PY'
import sys
from pathlib import Path

reader_dir, fixture_path, session_path, tests_dir = map(Path, sys.argv[1:])
reader_sources = sorted(reader_dir.glob("*.hako"))
test_sources = sorted(tests_dir.glob("*.rs"))
direct_sources = [
    reader_dir / "reader_v0.hako",
    reader_dir / "reader_root_v0.hako",
    reader_dir / "reader_stmt_v0.hako",
    reader_dir / "reader_expr_child_v0.hako",
    reader_dir / "reader_expr_leaf_v0.hako",
    reader_dir / "flat_publisher_v0.hako",
    reader_dir / "snapshot_sealer_v0.hako",
    fixture_path,
]
for path in [*reader_sources, *test_sources, session_path, fixture_path]:
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")

direct = "\n".join(path.read_text(encoding="utf-8") for path in direct_sources)
for forbidden in (
    "MapBox", "replay_only", "ValidatedProgramV0BodyView", "ValidatedNodeV0",
    "SnapshotBuilderV0", "RustSnapshot", "byte_count_sidecar", "indexOf",
    "token offset", "raw JSON scanner", "MIRBuilder", "planner", "route selection",
):
    if forbidden in direct:
        raise SystemExit(f"forbidden direct-reader dependency: {forbidden}")

negative = (tests_dir / "negative.rs").read_text(encoding="utf-8")
for needle in (
    '"cond":{"type":"ArrayLiteral"', '"then":[{"type":"Extern"',
    '"body":[{"type":"Throw"', '"rhs":{"type":"ArrayLiteral"',
    '"args":[{"type":"Int","value":1},{"type":"New"',
    '"recv":{"type":"BlockExpr"', "strict_syntax_failures_precede_hako_reader",
    r'\u0062ody',
    "unsupported_backend_fails_before_parse_session_and_hako_effects",
):
    if needle not in negative:
        raise SystemExit(f"missing direct negative proof: {needle}")

limits = (tests_dir / "limits.rs").read_text(encoding="utf-8")
for needle in (
    "BudgetLimitV0::Depth", "BudgetLimitV0::ChildrenPerBody",
    "BudgetLimitV0::Arguments", "BudgetLimitV0::AtomBytes",
    "BudgetLimitV0::LiteralBytes", "decoded_text_with_bytes",
    "limit.max_depth", "limit.max_children_per_body", "limit.max_arguments",
    "limit.max_atom_bytes", "limit.max_literal_bytes",
):
    if needle not in limits:
        raise SystemExit(f"missing direct limit proof: {needle}")

session = session_path.read_text(encoding="utf-8")
preflight = session.index("enforce_mir_backend_supported(module, backend)")
session_open = session.index("let guard = self.open_strict_json_session(input)?", preflight)
execute = session.index("execute_function_with_args", session_open)
if not preflight < session_open < execute:
    raise SystemExit("backend preflight must precede parse/session and Hako execution")

print("nested_unsupported_positions=cond,then,body,rhs,args,recv")
print("nested_invalid_input_parity=green")
print("strict_syntax_before_hhako=green")
print("direct_inclusive_limit_boundaries=green")
print("large_budget_component_boundaries=green")
print("nyash_str_cp_modes=identical")
print("direct_replay_dependency=0")
print("raw_scanner_dependency=0")
print("backend_preflight_before_parse_session_hhako=green")
print("unsupported_backend_fallback=0")
print("partial_snapshot_publication=0")
print("summary=ok")
PY

echo "[$TAG] ok"
