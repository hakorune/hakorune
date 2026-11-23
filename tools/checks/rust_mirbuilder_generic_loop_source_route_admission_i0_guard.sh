#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-generic-loop-source-route-admission-i0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ROUTER="$ROOT_DIR/src/mir/builder/control_flow/joinir/route_entry/router.rs"
RAW_LOOP="$ROOT_DIR/src/mir/builder/raw_loop_child_entry.rs"
SPINE="$ROOT_DIR/src/mir/compiler/loop_node_winner_spine.rs"
OLD_ADMISSION="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts/generic/source_admission.rs"
OLD_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts/generic/source_admission_tests.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-call-parser-array-push-route-overlap-d0-2026-09-22.md"
MANIFEST="$ROOT_DIR/docs/development/current/main/design/fixtures/generic-m10b-deletion-manifest-v1.tsv"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$RAW_LOOP" "$CARD" "$MANIFEST"

if [[ -e "$OLD_ADMISSION" || -e "$OLD_TESTS" || -e "$ROUTER" || -e "$SPINE" ]]; then
  guard_fail "$TAG" "superseded registry-backed GenericLoop source admission was reintroduced"
fi

guard_expect_fixed_in_file "$TAG" "[raw-loop-child-entry/checked-source-route-unavailable]" "$RAW_LOOP" \
  "raw Loop entry must name the missing checked route before effect emission"
guard_expect_fixed_in_file "$TAG" "superseded_by_M10b_I0_R0" "$CARD" \
  "the previous source-admission decision must be marked superseded"
guard_expect_fixed_in_file "$TAG" "B01" "$MANIFEST" \
  "the deleted source-admission edge must remain in the finite R0 delete manifest"

if rg -n -- 'CallableGenericLoopSourceRouteAdmissionV1|verify_located_generic_loop_v1' \
  "$ROOT_DIR/src/mir/builder"; then
  guard_fail "$TAG" "retired GenericLoop source-route admission symbols remain in the builder"
fi

lines="$(wc -l < "$RAW_LOOP" | tr -d '[:space:]')"
if (( lines >= 800 )); then
  guard_fail "$TAG" "selected loop route reached 800-line boundary: raw_loop_child_entry.rs=$lines"
fi

printf '[%s] PASS: old GenericLoop source-route owner retired; checked raw Loop refusal pinned\n' "$TAG"
