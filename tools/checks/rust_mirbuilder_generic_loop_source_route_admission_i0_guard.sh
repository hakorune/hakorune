#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-generic-loop-source-route-admission-i0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ADMISSION="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts/generic/source_admission.rs"
ISSUER="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts/generic/issuer.rs"
ROUTE_SELECTION="$ROOT_DIR/src/mir/builder/control_flow/joinir/route_entry/registry/selection.rs"
ROUTE="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_route.rs"
TESTS="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts/generic/source_admission_tests.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-call-parser-array-push-route-overlap-d0-2026-09-22.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_generic_loop_source_route_admission_i0_guard.sh"

 guard_require_command "$TAG" rg
 guard_require_command "$TAG" wc
 guard_require_files "$TAG" "$ADMISSION" "$ISSUER" "$ROUTE_SELECTION" "$ROUTE" "$TESTS" "$CARD" "$INDEX"

guard_expect_fixed_in_file "$TAG" "CallableGenericLoopSourceRouteAdmissionV1" "$ADMISSION" \
  "source route admission must remain the named aggregate"
guard_expect_fixed_in_file "$TAG" "[LoopRouteId::GenericLoopV1]" "$ADMISSION" \
  "exact GenericLoopV1 must remain admitted"
guard_expect_fixed_in_file "$TAG" "[LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]" "$ADMISSION" \
  "only the documented V0/V1 overlap may be source-admitted"
guard_expect_fixed_in_file "$TAG" "PreparedCallableGenericLoopSourceEvidenceV1" "$ISSUER" \
  "issuer must build source evidence before route admission"
guard_expect_fixed_in_file "$TAG" "verify_located_generic_loop_v1" "$ROUTE_SELECTION" \
  "raw registry must retain the exact V1 verifier"
guard_expect_fixed_in_file "$TAG" "source_evidence_rejects_missing_selected_target_before_disposition" "$TESTS" \
  "missing source target must have focused evidence"
guard_expect_fixed_in_file "$TAG" "source_evidence_rejects_duplicate_selected_target_before_consumption" "$TESTS" \
  "duplicate source target must have focused evidence"
guard_expect_fixed_in_file "$TAG" "source_evidence_rejects_value_demanded_target_without_scalar_contract" "$TESTS" \
  "value-demanded source target must have focused evidence"
guard_expect_fixed_in_file "$TAG" "issuer_rejects_foreign_owner_before_facts" "$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts_tests.rs" \
  "foreign owner must remain an effect-zero issuer negative"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "check index must list this reusable guard"
guard_expect_fixed_in_file "$TAG" "MIR-CALL-PARSER-ARRAY-PUSH-ROUTE-EVIDENCE-I0-FOCUSED-NEGATIVE" "$CARD" \
  "active card must name the focused negative row"

if rg -n -- 'contains\(&LoopRouteId::GenericLoopV1\)|Ok\(None\)|GenericLoopV0.*drop' "$ADMISSION"; then
    guard_fail "$TAG" "source admission must not use contains/None/drop shortcuts"
fi

for test_name in \
  source_evidence_rejects_missing_selected_target_before_disposition \
  source_evidence_rejects_duplicate_selected_target_before_consumption \
  source_evidence_rejects_value_demanded_target_without_scalar_contract; do
    count="$(rg -F -o -- "fn ${test_name}" "$TESTS" | wc -l | tr -d '[:space:]')"
    if [[ "$count" -ne 1 ]]; then
        guard_fail "$TAG" "focused negative must have one definition: ${test_name} (found ${count})"
    fi
done

printf '[%s] PASS: source admission shapes and focused negative corpus are pinned\n' "$TAG"
