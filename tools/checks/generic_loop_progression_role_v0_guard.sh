#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="generic-loop-progression-role-v0"
ROLE_TEST="progression_role"
CONTRACT_PIN="tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh"
REPORT_FIXTURE="tools/checks/fixtures/generic_loop_a2_c0_record_candidate_report.txt"
PROOF_FIXTURE="tools/checks/fixtures/generic_loop_a2_c1_record_proof_report.txt"
LOG="/tmp/${TAG}.contract-pin.log"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" timeout
guard_require_files "$TAG" \
  "$ROOT/src/mir/builder/control_flow/plan/generic_loop/facts/progression_role/observation.rs" \
  "$ROOT/src/mir/builder/control_flow/plan/generic_loop/facts/progression_role/report.rs" \
  "$ROOT/src/mir/builder/control_flow/plan/generic_loop/facts/progression_role/branch_control.rs" \
  "$ROOT/src/mir/builder/control_flow/plan/generic_loop/facts/progression_role/policy.rs" \
  "$ROOT/src/mir/builder/control_flow/plan/generic_loop/facts/progression_role/coverage_inventory.rs" \
  "$ROOT/$REPORT_FIXTURE" \
  "$ROOT/$PROOF_FIXTURE" \
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
for forbidden in (
    "observe_candidate_control_anchors_v0",
    "prove_candidate_progression_v0",
    "LoopProgressionProofV0",
):
    if forbidden in v1:
        raise SystemExit(f"C1 changed product acceptance path: {forbidden}")

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

proof_path = root / "tools/checks/fixtures/generic_loop_a2_c1_record_proof_report.txt"
proof = proof_path.read_text(encoding="utf-8").strip()
for needle in (
    "label=j|",
    "CurrentLoopExitGuard",
    "proof=Proven(ControlAnchoredBodyManaged",
    "label=fields|anchors=CandidateControlAnchorsV0 { anchors: [] }",
    'proof=Unproven("candidate.control_anchor_missing")',
    "label=field_count|anchors=CandidateControlAnchorsV0 { anchors: [] }",
):
    if needle not in proof:
        raise SystemExit(f"C1 fixture missing proof evidence: {needle}")
if proof.count("proof=Proven(") != 1 or proof.count("proof=Unproven(") != 2:
    raise SystemExit("C1 fixture must preserve exactly one Proven and two Unproven rows")

policy = (base / "facts/progression_role/policy.rs").read_text(encoding="utf-8")
branch_control = (base / "facts/progression_role/branch_control.rs").read_text(encoding="utf-8")
coverage_inventory = (base / "facts/progression_role/coverage_inventory.rs").read_text(encoding="utf-8")
neutral = policy + "\n" + branch_control
for forbidden in (
    "ParserRecordDeclarationBox",
    "label=j",
    "label=fields",
    "ShapeId",
    "std::env",
):
    if forbidden in neutral:
        raise SystemExit(f"forbidden C1 neutral-proof dependency: {forbidden}")
for forbidden_import in (
    r"^use .*recipes",
    r"^use .*lower",
    r"^use .*backend",
    r"^use .*runtime",
):
    if re.search(forbidden_import, neutral, flags=re.MULTILINE | re.IGNORECASE):
        raise SystemExit(f"forbidden C1 neutral-proof import: {forbidden_import}")

for needle in (
    "existing_verifier_does_not_reject_an_omitted_statement",
    "existing_verifier_does_not_reject_a_duplicate_statement_reference",
    "existing_verifier_still_rejects_an_out_of_bounds_reference",
):
    if needle not in coverage_inventory:
        raise SystemExit(f"missing C2-P0 coverage evidence: {needle}")

print("c0_candidate_capture=green")
print("c0_acceptance_widening=0")
print("c0_product_path_connection=0")
print("c0_preexisting_terminal=preserved")
print("c0_record_inventory=exact_fixture")
print("c1_closed_anchor_policy=green")
print("c1_record_proof_outcome=unique")
print("c1_product_path_connection=0")
print("c2_p0_existing_exact_coverage_owner=missing")
print("c2_p0_recipe_local_coverage=representable")
print("c2_p0_source_recipe_identity=design_stop")
print("parser_source_change=0")
print("summary=ok")
PY

echo "[$TAG] ok"
