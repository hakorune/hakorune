#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="generic-loop-progression-role-v0"
ROLE_TEST="progression_role"
CONTRACT_PIN="tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh"
REPORT_FIXTURE="tools/checks/fixtures/generic_loop_a2_c0_record_candidate_report.txt"
LOG="/tmp/${TAG}.contract-pin.log"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" timeout
guard_require_files "$TAG" \
  "$ROOT/src/mir/builder/control_flow/plan/generic_loop/facts/progression_role/observation.rs" \
  "$ROOT/src/mir/builder/control_flow/plan/generic_loop/facts/progression_role/report.rs" \
  "$ROOT/$REPORT_FIXTURE" \
  "$ROOT/$CONTRACT_PIN"

cd "$ROOT"
cargo test -q "$ROLE_TEST" --lib
cargo build -q --release --features vm-reference --bin hakorune

set +e
timeout 240s bash "$CONTRACT_PIN" >"$LOG" 2>&1
pin_rc=$?
set -e

if [ "$pin_rc" -eq 0 ]; then
  guard_fail "$TAG" "C0 must not widen acceptance or make the contract pin green"
fi
if [ "$pin_rc" -eq 124 ]; then
  tail -n 120 "$LOG" >&2
  guard_fail "$TAG" "contract pin exceeded 240 seconds"
fi
rg -q "ParserDelegateExposesBox\._parse_delegate/3|FuncScannerBox\._scan_methods/4" "$LOG" \
  || guard_fail "$TAG" "pre-C0 terminal owner changed"
rg -q "loop var used after in-body step|no valid loop_var candidates found" "$LOG" \
  || guard_fail "$TAG" "pre-C0 terminal reason changed"

python3 - "$ROOT" <<'PY'
import pathlib
import re
import sys

root = pathlib.Path(sys.argv[1])
base = root / "src/mir/builder/control_flow/plan/generic_loop"
for path in base.rglob("*.rs"):
    lines = len(path.read_text(encoding="utf-8").splitlines())
    if lines >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path} has {lines}")

report = (base / "facts/progression_role/report.rs").read_text(encoding="utf-8")
for needle in (
    "CandidateIdV0",
    "CandidateDecisionRowV0",
    "CandidateSelectionReportV0",
    "capture_multiple_candidate_report_v0",
    "diagnostic_label",
    "discovery_source",
    "provisional_rank",
    "comparison",
):
    if needle not in report:
        raise SystemExit(f"missing C0 report contract: {needle}")
for forbidden in (
    r"^use .*facts_types",
    r"^use .*recipes",
    r"^use .*lower",
    r"^use .*backend",
    r"^use .*runtime",
    r"std::env",
    r"ShapeId",
):
    if re.search(forbidden, report, flags=re.MULTILINE | re.IGNORECASE):
        raise SystemExit(f"forbidden C0 report dependency: {forbidden}")

v1 = (base / "facts/extract/v1.rs").read_text(encoding="utf-8")
for forbidden in (
    "observe_candidate_progression_v0",
    "capture_multiple_candidate_report_v0",
    "CandidateSelectionReportV0",
):
    if forbidden in v1:
        raise SystemExit(f"C0 changed product acceptance path: {forbidden}")

fixture_path = root / "tools/checks/fixtures/generic_loop_a2_c0_record_candidate_report.txt"
fixture = fixture_path.read_text(encoding="utf-8").strip()
if not fixture.startswith(
    "[plan/trace:loop_var_candidates] ctx=generic_loop_v1 outcome=ambiguous "
):
    raise SystemExit("C0 fixture missing stable trace prefix")
for label in ("label=j|", "label=fields|", "label=field_count|"):
    if fixture.count(label) != 1:
        raise SystemExit(f"C0 fixture candidate row mismatch: {label}")
for needle in (
    "outcome=Multiple",
    "source=ExistingTrueLoopIncrement",
    "cond=false",
    "true_inc=true",
    "writes=",
    "canon=",
    "nonstep=",
    "post=",
    "conditional=",
    "role=",
    "rank=",
    "comparison=Tied",
):
    if needle not in fixture:
        raise SystemExit(f"C0 fixture missing captured field: {needle}")

print("c0_candidate_capture=green")
print("c0_acceptance_widening=0")
print("c0_product_path_connection=0")
print("c0_preexisting_terminal=preserved")
print("c0_record_inventory=exact_fixture")
print("parser_source_change=0")
print("summary=ok")
PY

echo "[$TAG] ok"
