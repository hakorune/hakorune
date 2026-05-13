#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-metadata-verifier-invariants"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-230-C194-VERIFIER-OWNED-ALLOCATION-INVARIANTS.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
VERIFICATION_ROOT="src/mir/verification.rs"
VERIFICATION_TYPES="src/mir/verification_types.rs"
VERIFIER="src/mir/verification/hako_alloc_metadata.rs"
ALIGNED_PLANNER="src/mir/hako_alloc_aligned_small_packed_store_pilot.rs"
HUGE_PLANNER="src/mir/hako_alloc_huge_page_packed_store_pilot.rs"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_metadata_verifier_invariants_guard.sh"

echo "[$TAG] checking C194 hako_alloc metadata verifier invariants"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$PHASE_README" \
  "$TASKBOARD" \
  "$INDEX" \
  "$VERIFICATION_ROOT" \
  "$VERIFICATION_TYPES" \
  "$VERIFIER" \
  "$ALIGNED_PLANNER" \
  "$HUGE_PLANNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C194 card must be complete"
guard_expect_in_file "$TAG" 'C194 status:' "$PLAN" "mimalloc plan must record C194 status"
guard_expect_in_file "$TAG" '`C194` is complete as' "$RECORD_SSOT" "record SSOT must mark C194 complete"
guard_expect_in_file "$TAG" '`293x-230`' "$PHASE_README" "phase README must list C194 row"
guard_expect_in_file "$TAG" '\[x\] `293x-230`' "$TASKBOARD" "taskboard must mark C194 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C194 guard"

guard_expect_in_file "$TAG" 'mod hako_alloc_metadata' "$VERIFICATION_ROOT" "MIR verifier must own C194 module"
guard_expect_in_file "$TAG" 'check_hako_alloc_metadata_invariants' "$VERIFICATION_ROOT" "MIR verifier must run C194 module check"
guard_expect_in_file "$TAG" 'HakoAllocMetadataInvariantViolation' "$VERIFICATION_TYPES" "verification error vocabulary must include C194 violation"
guard_expect_in_file "$TAG" 'check_hako_alloc_metadata_invariants' "$VERIFIER" "C194 verifier function must exist"
guard_expect_in_file "$TAG" 'missing source C209 packed ArrayBox pilot' "$VERIFIER" "C194 must require source packed pilot"
guard_expect_in_file "$TAG" 'aligned-small columns must be ptr=0, alignment=1, padded_size=2' "$VERIFIER" "C194 must lock aligned column order"
guard_expect_in_file "$TAG" 'huge-page columns must be page_id=0, ptr=1, requested_size=2, committed_size=3, live=4' "$VERIFIER" "C194 must lock huge column order"
guard_expect_in_file "$TAG" 'huge-page released_page_id_sentinel must stay -1' "$VERIFIER" "C194 must lock huge page-id sentinel"
guard_expect_in_file "$TAG" 'public_array_get_materialization_enabled' "$VERIFIER" "C194 must keep materialization closed"
guard_expect_in_file "$TAG" 'backend_lowering_enabled' "$VERIFIER" "C194 must keep backend lowering closed"
guard_expect_in_file "$TAG" 'verifier_rejects_missing_source_pilot' "$VERIFIER" "C194 tests must cover missing source pilot"
guard_expect_in_file "$TAG" 'verifier_rejects_bad_huge_released_sentinel' "$VERIFIER" "C194 tests must cover sentinel rejection"

cargo test -q hako_alloc_metadata
cargo test -q mir::hako_alloc_aligned_small_packed_store_pilot
cargo test -q mir::hako_alloc_huge_page_packed_store_pilot
cargo test -q mir::array_record_backend_capability

if rg -n 'hako_alloc_metadata|HakoAllocMetadataInvariantViolation|mir/verify:hako_alloc_metadata' \
  lang/src/hako_alloc lang/c-abi/shims src/llvm_py/instructions \
  >/tmp/"$TAG".leak 2>&1; then
  echo "[$TAG] ERROR: C194 verifier vocabulary leaked into hako_alloc/backend shim surfaces" >&2
  cat /tmp/"$TAG".leak >&2
  rm -f /tmp/"$TAG".leak
  exit 1
fi
rm -f /tmp/"$TAG".leak

if rg -n 'k2_wide_hako_alloc_metadata_verifier_invariants_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: C194 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

echo "[$TAG] ok"
