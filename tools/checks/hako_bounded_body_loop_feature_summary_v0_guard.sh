#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-bounded-body-loop-feature-summary-v0"
CONSUMER="$ROOT/lang/src/compiler/analysis/bounded_body_snapshot/loop_feature_summary_v0.hako"
FIXTURE="$ROOT/tools/checks/fixtures/bounded_body_snapshot_direct_reader_v0.hako"
TEST="$ROOT/src/backend/mir_interpreter/strict_json_session/tests/tests_snapshot/summary.rs"
OLD_SOURCE="$ROOT/lang/src/compiler/lib/loop_feature_facts.hako"
OLD_PARITY="$ROOT/tools/checks/rust_lifecycle_mirbuilder_loop_feature_facts_parity_gate.sh"
OLD_DECISION_GUARD="$ROOT/tools/checks/rust_lifecycle_mirbuilder_loop_feature_facts_token_snapshot_hako_adoption_decision_guard.sh"

cd "$ROOT"
for mode in unset 1; do
  if [ "$mode" = unset ]; then
    env -u NYASH_STR_CP cargo test -q --features vm-reference --lib \
      backend::mir_interpreter::strict_json_session::tests::tests_snapshot::summary
  else
    NYASH_STR_CP=1 cargo test -q --features vm-reference --lib \
      backend::mir_interpreter::strict_json_session::tests::tests_snapshot::summary
  fi
done

python3 - "$ROOT" "$CONSUMER" "$FIXTURE" "$TEST" "$OLD_SOURCE" "$OLD_PARITY" "$OLD_DECISION_GUARD" <<'PY'
import sys
from pathlib import Path

root, consumer_path, fixture_path, test_path, old_source, old_parity, old_decision_guard = map(Path, sys.argv[1:])
consumer = consumer_path.read_text(encoding="utf-8")
fixture = fixture_path.read_text(encoding="utf-8")
tests = test_path.read_text(encoding="utf-8")
for path in (consumer_path, fixture_path, test_path):
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")
for path in (old_source, old_parity, old_decision_guard):
    if path.exists():
        raise SystemExit(f"retired token artifact still exists: {path}")

for needle in (
    "LoopFeatureSummaryV0", "LoopFeatureSummaryOutcomeV0", "consume(snapshot_outcome)",
    'snapshot_outcome.status() != "Ready"', "source.path()", "source.node_kind()",
    "source.reason()", 'kind == "Loop" || kind == "LoopRange"',
    'edge.role() == "then" || edge.role() == "else"',
):
    if needle not in consumer:
        raise SystemExit(f"missing snapshot-only summary contract: {needle}")
for forbidden in (
    "LoopFeatureFacts", "try_extract_loop_feature_facts", "MapBox", "indexOf",
    "substring(", "MIRBuilder", "planner.", "route.", "ValueId", "BlockId",
    "MirInstruction", "PlanBox",
):
    if forbidden in consumer:
        raise SystemExit(f"forbidden summary dependency: {forbidden}")

for needle in (
    "loop_feature_summary(session, root)", "LoopFeatureSummaryV0Box.consume",
    "hako_loop_feature_summary_reads_only_the_sealed_snapshot",
    "hako_loop_feature_summary_preserves_non_ready_outcomes",
    "Return,Break,Continue", "Return Continue Loop",
):
    if needle not in fixture + tests:
        raise SystemExit(f"missing summary fixture proof: {needle}")

allowed = {
    consumer_path.resolve(), fixture_path.resolve(), test_path.resolve(),
    (root / "tools/checks/hako_bounded_body_loop_feature_summary_v0_guard.sh").resolve(),
    (root / "docs/development/current/main/CURRENT_STATE.toml").resolve(),
    (root / "docs/development/current/main/investigations/mirbuilder-hako-bounded-body-analysis-snapshot-v0-2026-07-12.md").resolve(),
    (root / "docs/development/current/main/investigations/mirbuilder-hako-loop-feature-snapshot-parity-2026-07-12.md").resolve(),
    (root / "docs/tools/check-scripts-index.md").resolve(),
}
for base in (root / "lang/src", root / "src", root / "tools/checks/fixtures"):
    for path in base.rglob("*"):
        if not path.is_file() or path.suffix not in {".hako", ".rs"}:
            continue
        if "LoopFeatureSummaryV0" in path.read_text(encoding="utf-8") and path.resolve() not in allowed:
            raise SystemExit(f"summary escaped observation-only allowlist: {path}")

old_tokens = (
    "LoopFeatureFactsBox",
    "using lang.compiler.lib.loop_feature_facts",
    "rust_lifecycle_mirbuilder_loop_feature_facts_parity_gate.sh",
    "rust_lifecycle_mirbuilder_loop_feature_facts_token_snapshot_hako_adoption_decision_guard.sh",
)
scan_roots = (root / "lang/src", root / "src", root / "tools/checks")
self_path = (root / "tools/checks/hako_bounded_body_loop_feature_summary_v0_guard.sh").resolve()
for base in scan_roots:
    for path in base.rglob("*"):
        if not path.is_file() or path.resolve() == self_path:
            continue
        text = path.read_text(encoding="utf-8", errors="ignore")
        for token in old_tokens:
            if token in text:
                raise SystemExit(f"retired token caller remains: {path}: {token}")

print("input=BoundedBodyAnalysisSnapshotV0")
print("rust_ast_fact_semantics_fixture=green")
print("ready_summary_parity=green")
print("unsupported_outcome_preserved=green")
print("invalid_input_outcome_preserved=green")
print("nested_loop_exit_skip=green")
print("raw_token_false_positive=0")
print("token_facade_removed=1")
print("token_facade_callers=0")
print("rust_planner_fact_changed=0")
print("planner_connection=0")
print("route_connection=0")
print("backend_connection=0")
print("runtime_connection=0")
print("mir_mutation=0")
print("summary=ok")
PY

echo "[$TAG] ok"
