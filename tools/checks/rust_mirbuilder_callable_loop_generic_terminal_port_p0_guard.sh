#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-callable-loop-generic-terminal-port-p0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ISSUER="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts.rs"
TESTS="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts_tests.rs"
SELECTION="$ROOT_DIR/src/mir/builder/control_flow/joinir/route_entry/registry/selection.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-callable-loop-source-facts-issuer-d0-2026-08-22.md"
README="$ROOT_DIR/src/mir/builder/README.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_callable_loop_generic_terminal_port_p0_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$ISSUER" "$TESTS" "$SELECTION" "$CARD" "$README" "$INDEX"

guard_expect_fixed_in_file "$TAG" "CallableGenericLoopSourceFactsTerminalConsumerV1" "$ISSUER" \
  "terminal-only P0 must have one named consumer"
guard_expect_fixed_in_file "$TAG" "CallableGenericLoopSourceFactsConsumedV1" "$ISSUER" \
  "terminal-only P0 must have a move-only consumed state"
guard_expect_fixed_in_file "$TAG" "schedule: VerifiedCallableSemanticLoopBindingScheduleV1" "$ISSUER" \
  "consumed state must retain the existing source schedule"
guard_expect_fixed_in_file "$TAG" "selected: VerifiedLocatedGenericLoopV1SelectionV1" "$ISSUER" \
  "consumed state must retain the existing exact route seal"
guard_expect_fixed_in_file "$TAG" "terminal_consumer_moves_ready_into_one_no_effect_consumed_state" "$TESTS" \
  "focused test must prove the Ready move"
guard_expect_fixed_in_file "$TAG" "verify_located_generic_loop_v1" "$SELECTION" \
  "terminal route must originate in the existing exact selection proof"
guard_expect_fixed_in_file "$TAG" "Terminal-only Ready consumption P0" "$CARD" \
  "active card must describe the bounded terminal port"
guard_expect_fixed_in_file "$TAG" "Callable Loop source-aware Facts terminal-only P0" "$README" \
  "builder README must document terminal-only ownership"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "check index must list the terminal-port guard"

if rg -n -- 'crate::config::env|from_environment\(' "$ISSUER"; then
  guard_fail "$TAG" "terminal consumer must not reread policy"
fi
if rg -n -- 'RouteExecutionWitnessV1|PostEffectRetryDebt|lower_loop_or_freeze_v1|MirBuilder|ValueId' "$ISSUER"; then
  guard_fail "$TAG" "terminal consumer must not gain registry/lowering/physical authority"
fi

production_terminal_calls="$(
  (rg -F -o \
    --glob '!normal_callable_loop_source_facts.rs' \
    --glob '!normal_callable_loop_source_facts_tests.rs' \
    'CallableGenericLoopSourceFactsTerminalConsumerV1::consume(' \
    "$ROOT_DIR/src/mir/builder" || true) | wc -l | tr -d '[:space:]'
)"
if [[ "$production_terminal_calls" -ne 0 ]]; then
  guard_fail "$TAG" "terminal consumer must remain caller-zero; found $production_terminal_calls production calls"
fi

if rg -n -- 'derive\([^)]*Clone[^)]*\).*CallableGenericLoopSourceFactsConsumedV1|derive\([^)]*Copy[^)]*\).*CallableGenericLoopSourceFactsConsumedV1' "$ISSUER"; then
  guard_fail "$TAG" "consumed terminal state must remain move-only"
fi

for file in "$ISSUER" "$TESTS" "$SELECTION"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "terminal P0 source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (one move-only Ready->Consumed terminal seam, no production caller/effect/retry)"
