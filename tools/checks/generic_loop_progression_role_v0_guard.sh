#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="generic-loop-progression-role-v0"
ROLE_TEST="progression_role"
CONTRACT_PIN="tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh"
REPORT_FIXTURE="tools/checks/fixtures/generic_loop_a2_c0_record_candidate_report.txt"
PROOF_FIXTURE="tools/checks/fixtures/generic_loop_a2_c1_record_proof_report.txt"
TYPE0_V0_FIXTURE="apps/tests/generic_loop_carrier_type_v0_numeric_min.hako"
TYPE0_V1_FIXTURE="apps/tests/phase29cb_generic_loop_in_body_step_min.hako"
LOG="/tmp/${TAG}.contract-pin.log"
TYPE0_V0_LOG="/tmp/${TAG}.type0-v0.log"
TYPE0_V1_LOG="/tmp/${TAG}.type0-v1.log"
TYPE0_V0_DEBUG_LOG="/tmp/${TAG}.type0-v0-debug.log"
TYPE0_V1_DEBUG_LOG="/tmp/${TAG}.type0-v1-debug.log"
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
  "$ROOT/tools/checks/fixtures/generic_loop_carrier_type_m0_inventory_v1.json" \
  "$ROOT/tools/checks/lib/generic_loop_carrier_type_inventory.py" \
  "$ROOT/$TYPE0_V0_FIXTURE" \
  "$ROOT/$TYPE0_V1_FIXTURE" \
  "$ROOT/$CONTRACT_PIN"

cd "$ROOT"
cargo test -q "$ROLE_TEST" --lib
python3 "$ROOT/tools/checks/lib/generic_loop_carrier_type_inventory.py" \
  "$ROOT" --check-reference
cargo build -q --features vm-reference --bin hakorune
cargo build -q --release --features vm-reference --bin hakorune

set +e
HAKO_EMIT_EXE_CACHE=0 HAKO_JOINIR_DEBUG=1 \
  "$ROOT/target/debug/hakorune" --backend mir "$ROOT/$TYPE0_V0_FIXTURE" \
  >"$TYPE0_V0_DEBUG_LOG" 2>&1
type0_v0_debug_rc=$?
HAKO_EMIT_EXE_CACHE=0 HAKO_JOINIR_DEBUG=1 \
  "$ROOT/target/debug/hakorune" --backend mir "$ROOT/$TYPE0_V1_FIXTURE" \
  >"$TYPE0_V1_DEBUG_LOG" 2>&1
type0_v1_debug_rc=$?
HAKO_EMIT_EXE_CACHE=0 HAKO_JOINIR_DEBUG=1 \
  "$ROOT/target/release/hakorune" --backend mir "$ROOT/$TYPE0_V0_FIXTURE" \
  >"$TYPE0_V0_LOG" 2>&1
type0_v0_rc=$?
HAKO_EMIT_EXE_CACHE=0 HAKO_JOINIR_DEBUG=1 \
  "$ROOT/target/release/hakorune" --backend mir "$ROOT/$TYPE0_V1_FIXTURE" \
  >"$TYPE0_V1_LOG" 2>&1
type0_v1_rc=$?
set -e

[[ "$type0_v0_debug_rc" -eq 4 ]] \
  || guard_fail "$TAG" "TYPE0 numeric V0 debug result drift: rc=$type0_v0_debug_rc"
[[ "$type0_v1_debug_rc" -eq 3 ]] \
  || guard_fail "$TAG" "TYPE0 numeric V1 debug result drift: rc=$type0_v1_debug_rc"
[[ "$type0_v0_rc" -eq 4 ]] \
  || guard_fail "$TAG" "TYPE0 numeric V0 result drift: rc=$type0_v0_rc"
[[ "$type0_v1_rc" -eq 3 ]] \
  || guard_fail "$TAG" "TYPE0 numeric V1 result drift: rc=$type0_v1_rc"
rg -q "route=generic_loop_v0" "$TYPE0_V0_LOG" \
  || guard_fail "$TAG" "TYPE0 numeric V0 route drift"
rg -q "route=generic_loop_v1" "$TYPE0_V1_LOG" \
  || guard_fail "$TAG" "TYPE0 numeric V1 route drift"
rg -q "route=generic_loop_v0" "$TYPE0_V0_DEBUG_LOG" \
  || guard_fail "$TAG" "TYPE0 numeric V0 debug route drift"
rg -q "route=generic_loop_v1" "$TYPE0_V1_DEBUG_LOG" \
  || guard_fail "$TAG" "TYPE0 numeric V1 debug route drift"

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
rg -q "ParserBox\.esc_json/1" "$LOG" \
  || guard_fail "$TAG" "pre-TYPE0 terminal owner changed"
if rg -q "phi_type_publication/concrete_fact_conflict.*first_type=Integer.*second_type=String" "$LOG"; then
  guard_fail "$TAG" "retired CorePlan String Add conflict reappeared"
fi
rg -q "ParserBox\.static_const_parse_add/2" "$LOG" \
  || guard_fail "$TAG" "post-REP0 terminal function changed"
rg -qF "GenericLoop carrier representation failed: MissingTransientType { init:" "$LOG" \
  || guard_fail "$TAG" "post-REP0 GenericLoop transient-type frontier changed"

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
