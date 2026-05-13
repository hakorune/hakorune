#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-page-lifecycle-verifier-invariants"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-252-C194B-VERIFIER-OWNED-PAGE-LIFECYCLE-INVARIANTS.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
VERIFICATION_ROOT="src/mir/verification.rs"
VERIFICATION_TYPES="src/mir/verification_types.rs"
VERIFIER="src/mir/verification/hako_alloc_page_lifecycle.rs"
M207_GUARD="tools/checks/k2_wide_hako_alloc_page_lifecycle_invariant_guard.sh"
PROOF_RUNNER="tools/checks/run_proof_app.sh"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_page_lifecycle_verifier_invariants_guard.sh"

echo "[$TAG] checking C194b hako_alloc page lifecycle verifier invariants"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$PHASE_README" \
  "$TASKBOARD" \
  "$INDEX" \
  "$VERIFICATION_ROOT" \
  "$VERIFICATION_TYPES" \
  "$VERIFIER" \
  "$M207_GUARD" \
  "$PROOF_RUNNER" \
  "$PROOF_MANIFEST" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$M207_GUARD" "$PROOF_RUNNER" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C194b card must be complete"
guard_expect_in_file "$TAG" 'C194b status:' "$PLAN" "mimalloc plan must record C194b status"
guard_expect_in_file "$TAG" '`293x-252`' "$PHASE_README" "phase README must list C194b row"
guard_expect_in_file "$TAG" '\[x\] `293x-252`' "$TASKBOARD" "taskboard must mark C194b complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C194b guard"
guard_expect_in_file "$TAG" 'mod hako_alloc_page_lifecycle' "$VERIFICATION_ROOT" "MIR verifier must own C194b module"
guard_expect_in_file "$TAG" 'check_hako_alloc_page_lifecycle_invariants' "$VERIFICATION_ROOT" "MIR verifier must run C194b module check"
guard_expect_in_file "$TAG" 'HakoAllocPageLifecycleInvariantViolation' "$VERIFICATION_TYPES" "verification error vocabulary must include C194b violation"
guard_expect_in_file "$TAG" 'check_hako_alloc_page_lifecycle_invariants' "$VERIFIER" "C194b verifier function must exist"
guard_expect_in_file "$TAG" 'missing required lifecycle function' "$VERIFIER" "C194b must require lifecycle functions"
guard_expect_in_file "$TAG" 'missing lifecycle typed object plan' "$VERIFIER" "C194b must require lifecycle report plan"
guard_expect_in_file "$TAG" 'duplicate_decommit_blocked' "$VERIFIER" "C194b must lock duplicate decommit fact"
guard_expect_in_file "$TAG" 'must declare `i64`' "$VERIFIER" "C194b must lock lifecycle field declared type"
guard_expect_in_file "$TAG" 'must use i64 storage' "$VERIFIER" "C194b must lock lifecycle field storage"
guard_expect_in_file "$TAG" 'verifier_rejects_missing_required_function' "$VERIFIER" "C194b tests must cover missing function"
guard_expect_in_file "$TAG" 'verifier_rejects_bad_report_field_shape' "$VERIFIER" "C194b tests must cover field shape rejection"
guard_expect_in_file "$TAG" 'id = "M207"' "$PROOF_MANIFEST" "proof app manifest must keep M207 proof available"

cargo test -q hako_alloc_page_lifecycle
bash "$M207_GUARD"
bash "$PROOF_RUNNER" M207

if rg -n 'check_hako_alloc_page_lifecycle_invariants|HakoAllocPageLifecycleInvariantViolation|mir/verify:hako_alloc_page_lifecycle' \
  lang/src/hako_alloc lang/c-abi/shims src/llvm_py/instructions \
  >/tmp/"$TAG".leak 2>&1; then
  echo "[$TAG] ERROR: C194b verifier vocabulary leaked into hako_alloc/backend shim surfaces" >&2
  cat /tmp/"$TAG".leak >&2
  rm -f /tmp/"$TAG".leak
  exit 1
fi
rm -f /tmp/"$TAG".leak

if rg -n 'k2_wide_hako_alloc_page_lifecycle_verifier_invariants_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: C194b guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

echo "[$TAG] ok"
