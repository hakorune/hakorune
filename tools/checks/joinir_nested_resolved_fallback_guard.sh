#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="joinir-nested-resolved-fallback-guard"
COMPILER_DIR="$ROOT_DIR/src/mir/compiler"
COMPILER_MOD="$COMPILER_DIR/mod.rs"
SOURCE_BOUND="$COMPILER_DIR/source_bound_package.rs"
CUTOVER="$COMPILER_DIR/resolved_nested_predicate_cutover.rs"

fail() {
  printf '[%s] ERROR: %s\n' "$TAG" "$1" >&2
  exit 1
}

require_file() {
  [[ -f "$1" ]] || fail "missing file: ${1#"$ROOT_DIR/"}"
}

count_fixed() {
  local pattern="$1"
  local path="$2"
  local expected="$3"
  local count
  count="$(rg -n -F "$pattern" "$path" 2>/dev/null | wc -l | tr -d '[:space:]')"
  [[ "$count" == "$expected" ]] || \
    fail "anchor count drift: ${pattern@Q} in ${path#"$ROOT_DIR/"}: $count != $expected"
}

require_file "$COMPILER_MOD"
require_file "$SOURCE_BOUND"
require_file "$CUTOVER"

# The resolved first-family ingress has one named Nested consumer and one
# source-bound physical lowerer. This is a structural caller census, not a
# runtime claim and not permission to delete the still-live normal/raw route.
count_fixed \
  'resolved_nested_predicate_cutover::compile_nested_predicate_source_bound(' \
  "$COMPILER_MOD" 1
count_fixed \
  'pub(super) fn compile_nested_predicate_source_bound(' \
  "$CUTOVER" 1
count_fixed \
  'builder.lower_resolved_nested_predicate_function_draft(plan)' \
  "$SOURCE_BOUND" 1

for forbidden in \
  'route_loop(' \
  'try_cf_loop_joinir(' \
  'lower_loop_or_freeze_v1(' \
  'route_nested_loop_minimal'
do
  if rg -n -F "$forbidden" "$COMPILER_DIR" >/dev/null 2>&1; then
    fail "resolved compiler ingress still references legacy Loop edge: $forbidden"
  fi
done

for required in \
  'CanonicalFirstFamilyPlanV1::Loop(CanonicalLoopFamilyPlanV1::NestedPredicate(plan))' \
  'ExactCanonicalPreflightPlanV1::Loop(CanonicalLoopFamilyPlanV1::NestedPredicate(' \
  'CanonicalLoopFamilyPlanV1::NestedPredicate('
do
  rg -n -F "$required" "$COMPILER_MOD" "$SOURCE_BOUND" >/dev/null || \
    fail "Nested resolved-family anchor missing: $required"
done

for path in "$COMPILER_MOD" "$SOURCE_BOUND" "$CUTOVER"; do
  lines="$(wc -l < "$path" | tr -d '[:space:]')"
  (( lines < 800 )) || fail "800-line boundary exceeded: ${path#"$ROOT_DIR/"}=$lines"
done

printf '[%s] ok: resolved Nested ingress has one canonical caller, no legacy Loop edge, and stays below 800 lines\n' "$TAG"
