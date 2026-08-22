#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-callable-loop-generic-facts-policy-p0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FACTS_BUILDER="$ROOT_DIR/src/mir/builder/control_flow/plan/facts/loop_builder.rs"
POLICY="$ROOT_DIR/src/mir/builder/control_flow/plan/generic_loop/facts/policy.rs"
EXTRACTOR="$ROOT_DIR/src/mir/builder/control_flow/plan/generic_loop/facts/extract/v1.rs"
CONTEXT="$ROOT_DIR/src/mir/builder/control_flow/plan/planner/context.rs"
README="$ROOT_DIR/src/mir/builder/control_flow/plan/generic_loop/README.md"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-callable-physical-header-completion-value-d0-2026-08-22.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_callable_loop_generic_facts_policy_p0_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$FACTS_BUILDER" "$POLICY" "$EXTRACTOR" "$CONTEXT" \
  "$README" "$CARD" "$INDEX"

guard_expect_fixed_in_file "$TAG" "GenericLoopFactsPolicyFrameV1" "$POLICY" \
  "Facts policy frame must be an explicit product"
guard_expect_fixed_in_file "$TAG" "from_generic_loop_policy" "$CONTEXT" \
  "PlannerContext must provide the source-aware policy transport seam"
guard_expect_fixed_in_file "$TAG" "try_extract_generic_loop_v1_with_policy" "$EXTRACTOR" \
  "GenericLoop extractor must accept an explicit policy frame"
guard_expect_fixed_in_file "$TAG" "generic_loop_v1_probe" "$FACTS_BUILDER" \
  "Facts builder must retain the one-shot probe result"
guard_expect_fixed_in_file "$TAG" "probe.map(|extraction| extraction.into_facts())" "$FACTS_BUILDER" \
  "final GenericLoop Facts must consume the retained extraction"
guard_expect_fixed_in_file "$TAG" "GenericLoopFactsPolicyFrameV1 per Facts build" "$CARD" \
  "active card must record the one-frame acceptance"
guard_expect_fixed_in_file "$TAG" "Policy frame boundary" "$README" \
  "GenericLoop README must document the policy boundary"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "check index must list the reusable P0 guard"

# The hint path must not use a compatibility facade and then ask the extractor
# for final Facts again. The two explicit calls below are mutually exclusive:
# one is the conditional probe, the other is the no-probe fallback.
if rg -n -- 'has_generic_loop_v1_recipe_hint\(|try_extract_generic_loop_v1_facts\(' \
  "$FACTS_BUILDER"; then
  guard_fail "$TAG" "Facts builder still calls a duplicate/hint extractor facade"
fi
call_count="$(rg -F -o -- 'try_extract_generic_loop_v1_with_policy(' "$FACTS_BUILDER" | wc -l | tr -d '[:space:]')"
if [[ "$call_count" -ne 2 ]]; then
  guard_fail "$TAG" "Facts builder must have exactly two mutually-exclusive policy call sites; found $call_count"
fi
guard_expect_fixed_in_file "$TAG" "else if let Some(probe) = generic_loop_v1_probe" "$FACTS_BUILDER" \
  "a successful or empty probe must be reused for final Facts"

# The selected GenericLoop policy may be read from ambient environment only at
# the outer frame constructor. The extractor and Facts summary consume the
# frame instead of re-reading environment state.
if rg -n -- 'crate::config::env|joinir_dev_enabled' "$EXTRACTOR" "$FACTS_BUILDER"; then
  guard_fail "$TAG" "selected GenericLoop Facts path still re-reads ambient policy"
fi

for file in "$FACTS_BUILDER" "$POLICY" "$EXTRACTOR" "$CONTEXT"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "P0 source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

# P0 is a caller-zero foundation. Source relation, ordinary consumer, route
# selection, fallback/retry, and production switch remain later tasks.
if rg -n -- 'CallableGenericLoopSourceFactsIssuerV1|CallableLoopOutsideReasonV1|lower_outside_callable_loop_v1' \
  "$FACTS_BUILDER" "$POLICY" "$EXTRACTOR" "$CONTEXT"; then
  guard_fail "$TAG" "P0 source/facts transport grew into the later consumer slice"
fi

echo "[$TAG] ok (one policy frame, one-shot GenericLoop extraction, source-aware caller=0)"
