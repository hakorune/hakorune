#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-strict-program-v0-body-view"
source "$ROOT/tools/checks/lib/guard_common.sh"

VIEW="$ROOT/src/analysis/bounded_body_snapshot_v0/program_v0_body_view.rs"
JSON="$ROOT/src/analysis/bounded_body_snapshot_v0/strict_json.rs"
TESTS="$ROOT/src/analysis/bounded_body_snapshot_v0/tests.rs"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_files "$TAG" "$VIEW" "$JSON" "$TESTS"

cargo test -q --lib analysis::bounded_body_snapshot_v0::tests

python3 - "$VIEW" "$JSON" "$TESTS" <<'PY'
import sys
from pathlib import Path

view_path, json_path, tests_path = map(Path, sys.argv[1:])
view = view_path.read_text(encoding="utf-8")
strict_json = json_path.read_text(encoding="utf-8")
tests = tests_path.read_text(encoding="utf-8")
joined = view + "\n" + strict_json

required = (
    "deserializer.end()",
    "duplicate key:",
    "object.forbidden_unknown_field",
    "unsupported.wire_kind",
    "transport.schema_mismatch_stop",
    "int.not_canonical_i64",
    "operator.invalid_for_kind",
    "ValidatedProgramV0BodyView",
)
for needle in required:
    if needle not in joined:
        raise SystemExit(f"missing strict boundary token: {needle}")

test_contracts = (
    "rejects_duplicate_keys_and_trailing_input",
    "rejects_unknown_fields_and_tags",
    "preserves_known_unsupported_boundary",
    "validates_required_children_and_scalars",
    "duplicate_detection_uses_decoded_unicode_keys",
)
for needle in test_contracts:
    if needle not in tests:
        raise SystemExit(f"missing strict boundary test: {needle}")

for forbidden in (
    "crate::mir",
    "crate::runner",
    "crate::stage1",
    "MIRBuilder",
    "indexOf",
    "substring(",
    "fallback",
):
    if forbidden in joined:
        raise SystemExit(f"forbidden strict body-view dependency: {forbidden}")

for path in (view_path, json_path):
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")

print("output_contract=StrictStructuredProgramV0BodyViewV0")
print("full_input_validation=1")
print("duplicate_key_validation=1")
print("known_unsupported_distinction=1")
print("partial_snapshot_publication=0")
print("mir_or_planner_dependency=0")
print("source_files_under_800=1")
print("summary=ok")
PY
