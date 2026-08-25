#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-d1b-selected-normal-duplicate-projection-guard"
PARENT="$ROOT_DIR/src/mir/builder/program_root_work_plan.rs"
PRODUCTION="$ROOT_DIR/src/mir/builder/program_root_work_plan_production.rs"
VALIDATOR="$ROOT_DIR/src/mir/builder/program_root_work_plan/selected_projection_validator.rs"
TESTS="$ROOT_DIR/src/mir/builder/program_root_work_plan_tests.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-call-d1b-selected-normal-duplicate-projection-i0-2026-08-26.toml"

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

for file in "$PARENT" "$PRODUCTION" "$VALIDATOR" "$TESTS" "$CARD"; do
  [[ -f "$file" ]] || fail "missing owner ${file#$ROOT_DIR/}"
done

while IFS= read -r file; do
  lines="$(wc -l < "$file")"
  (( lines < 760 )) || fail "760-line split trigger exceeded: ${file#$ROOT_DIR/} ($lines)"
done < <(printf '%s\n' "$PARENT" "$PRODUCTION" "$VALIDATOR" "$TESTS")

python3 - "$PARENT" "$PRODUCTION" "$VALIDATOR" "$TESTS" "$CARD" <<'PY'
from pathlib import Path
import sys

parent, production, validator, tests, card = map(Path, sys.argv[1:])
parent_text = parent.read_text()
production_text = production.read_text()
validator_text = validator.read_text()
tests_text = tests.read_text()
card_text = card.read_text()

if parent_text.count("mod selected_projection_validator;") != 1:
    raise SystemExit("selected projection validator module declaration drifted")
if production_text.count("validate_selected_normal_top_level_projections(") != 1:
    raise SystemExit("production validator call count drifted")
if parent_text.count("validate_selected_normal_top_level_projections(") != 1:
    raise SystemExit("test-only validator call count drifted")

for token in (
    "duplicate-physical-projection",
    "source-projection-mismatch",
    "source-missing",
    "top-level-site-kind-mismatch",
    "source-statement-missing",
    "source-row-mismatch",
):
    if token not in validator_text:
        raise SystemExit(f"validator lost rejection state: {token}")

if "MirInstruction::call" in validator_text or "Callee" in validator_text:
    raise SystemExit("validator issued a target or canonical Call")
if "LegacyReplaceWholePair" in production_text:
    raise SystemExit("SelectedNormal work-plan owner re-entered RawCompatibility replacement")

selected_start = production_text.index(
    "if work_plan_admission == ProgramRootWorkPlanAdmissionV1::SelectedNormal"
)
loop_start = production_text.index("let mut immediate = Vec::new();", selected_start)
if production_text.index("validate_selected_normal_top_level_projections(", selected_start, loop_start) > loop_start:
    raise SystemExit("SelectedNormal validation moved after work-plan mutation")

for token in (
    "selected_top_level_functions_reject_duplicate_physical_projection",
    "accepts_unique_selected_normal_physical_projections",
    "rejects_same_name_and_arity_selected_normal_physical_projection",
    "duplicate-physical-projection",
):
    if token not in tests_text + validator_text:
        raise SystemExit(f"missing focused duplicate projection evidence: {token}")

for token in (
    "status = \"landed_bounded_row\"",
    "implementation_permission = false",
    "no callable-index or target handoff",
    "TypedRejectBeforeBodyLowering",
):
    if token not in card_text:
        raise SystemExit(f"active card lost bounded-row contract: {token}")
PY

echo "[$TAG] ok"
