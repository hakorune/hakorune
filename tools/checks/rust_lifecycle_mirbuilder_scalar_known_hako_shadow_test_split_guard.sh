#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-hako-shadow-test-split"
source "$ROOT/tools/checks/lib/guard_common.sh"

SOURCE="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
TESTS="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow/tests.rs"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3425-MIRBUILDER-SCALAR-KNOWN-HAKO-SHADOW-TEST-SPLIT-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_files "$TAG" "$SOURCE" "$TESTS" "$CARD" "$TASK_ORDER" "$MANIFEST"

SOURCE_LINES="$(wc -l < "$SOURCE")"
TEST_LINES="$(wc -l < "$TESTS")"
if [ "$SOURCE_LINES" -ge 800 ] || [ "$TEST_LINES" -ge 800 ]; then
  echo "[$TAG] source/test file must remain below 800 lines" >&2
  exit 1
fi

python3 - "$SOURCE" "$TESTS" "$CARD" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

source = Path(sys.argv[1]).read_text(encoding="utf-8")
tests = Path(sys.argv[2]).read_text(encoding="utf-8")
card = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[5]).read_text(encoding="utf-8"))


def need(condition, message):
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-SCALAR-KNOWN-HAKO-SHADOW-TEST-SPLIT-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-POLICY-ROW-IDENTITY-TRANSPORT-001"
need(token in card, "card token missing")
need(token in {row.get("token") for row in manifest.get("rows", [])}, "manifest token missing")
need(next_card in card and next_card in task_order, "next task pointer missing")
need('#[path = "scalar_known_hako_shadow/tests.rs"]' in source, "test path module missing")
need("mod tests;" in source, "test module declaration missing")
need("#[cfg(test)]\nmod tests" not in source, "test body remains in production source")
need("use super::*;" in tests, "test parent import missing")
for name in [
    "mapload_scalar_i64_shadow_artifact_matches_rust_fastpath_policy",
    "string_scalar_i64_shadow_artifact_matches_rust_fastpath_policy",
    "collection_scalar_i64_shadow_artifact_matches_rust_fastpath_policy",
    "mapstore_i64_shadow_artifact_matches_rust_fastpath_policy",
    "write_push_shadow_artifact_matches_rust_fastpath_policy",
    "mapstore_any_shadow_artifact_matches_rust_fastpath_policy",
]:
    need(tests.count(f"fn {name}(") == 1, f"test missing or duplicated: {name}")
need(tests.count("#[should_panic") == 13, "shadow rejection test count drift")
for forbidden in [
    "caller_orientation_runtime_path",
    "route_selection_authority_switch",
    "backend_lowering_authority",
    "source_selfhost_claim",
]:
    need(forbidden not in source and forbidden not in tests, f"3425 opened forbidden claim: {forbidden}")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-hako-shadow-test-split")
print(f"production_source_lines={len(source.splitlines())}")
print(f"test_source_lines={len(tests.splitlines())}")
print("shadow_test_boxshape_split=1")
print("production_behavior_changed=0")
print("caller_orientation_runtime_path=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY

cargo test -q scalar_known_hako_shadow
