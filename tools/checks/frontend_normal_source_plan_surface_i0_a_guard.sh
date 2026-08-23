#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-normal-source-plan-surface-i0-a"
source "$ROOT/tools/checks/lib/guard_common.sh"

SURFACE="$ROOT/src/parser/callable_parameter_source/normal_source_plan_surface.rs"
SURFACE_TESTS="$ROOT/src/parser/callable_parameter_source/normal_source_plan_surface_tests.rs"
SEED="$ROOT/src/parser/callable_parameter_source/normal_source_plan_seed.rs"
STATIC="$ROOT/src/parser/callable_parameter_source/static_box_source.rs"
PRODUCT="$ROOT/src/parser/callable_parameter_source/product.rs"
POSTPASS="$ROOT/src/parser/postpass_envelope.rs"
FINALIZE="$ROOT/src/parser/source_seal/finalize.rs"
SLOTS="$ROOT/src/parser/build_cfg/program_item_slots.rs"
CARD="$ROOT/docs/development/current/main/investigations/normal-root-source-plan-surface-i0-a-2026-08-23.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$SURFACE" "$SURFACE_TESTS" "$SEED" "$STATIC" \
  "$PRODUCT" "$POSTPASS" "$FINALIZE" "$SLOTS" "$CARD" "$INDEX"

[[ "$(rg -c 'pub\(in crate::parser\) struct ParserNormalSourcePlanSurfaceIssuerV1' "$SURFACE")" == 1 ]]
[[ "$(rg -c 'ParserNormalSourcePlanSurfaceIssuerV1::issue_once' "$PRODUCT")" == 1 ]]
[[ "$(rg -c 'consume_normal_source_plan_seed\(' "$POSTPASS")" == 1 ]]
[[ "$(rg -c 'let source_plan_seed = completed\.consume_normal_source_plan_seed\(\);' "$PRODUCT")" == 1 ]]
[[ "$(rg -c 'normal_source_plan_surface: ParserNormalSourcePlanSurfaceDispositionV1' "$PRODUCT")" == 1 ]]
[[ "$(rg -c 'pub\(in crate::parser\) fn into_rows\(' "$SLOTS")" == 1 ]]
[[ "$(rg -c 'ParserNormalSourcePlanSeedDispositionV1::Ready\(seed\)' "$FINALIZE")" == 1 ]]

if rg -n 'NormalSourceSurfaceInventoryV1|VerifiedRawRootExpansionV1|MirBuilder|NormalCompileRequest|ValueId|BasicBlockId|fallback|retry|from_program' "$SURFACE"; then
  guard_fail "$TAG" "parser source surface leaked policy, Builder, physical, or AST-reconstruction authority"
fi

if rg -n 'prepared: PreparedParserStaticBoxParentSourceV1' "$STATIC"; then
  guard_fail "$TAG" "narrow static seal retained the full prepared parent payload"
fi

if rg -n 'normal_source_plan_surface[^\n]*Option|normal_source_plan_seed[^\n]*Option' "$PRODUCT" "$POSTPASS" "$SEED"; then
  guard_fail "$TAG" "source surface/seed became optional or default-merged"
fi

if rg -n 'ParserNormalSourcePlanSurfaceIssuerV1|normal_source_plan_surface' "$ROOT/src/mir/compiler"; then
  guard_fail "$TAG" "compiler reached the parser-only I0-A surface before the named consumer cell"
fi

for needle in \
  'executable_script_surface_is_issued_once_as_one_complete_row' \
  'ordinary_box_surface_keeps_the_parser_owned_non_static_observation' \
  'top_level_callable_and_executable_rows_share_one_surface_order' \
  'static_main_surface_keeps_the_nested_parent_relation' \
  'compatibility_postpass_cannot_emit_a_source_plan_bound'; do
  rg -q "$needle" "$SURFACE_TESTS" || guard_fail "$TAG" "missing focused evidence: $needle"
done

rg -q 'NORMAL-ROOT-SOURCE-PLAN-SURFACE-I0-A' "$CARD"
rg -q 'frontend_normal_source_plan_surface_i0_a_guard.sh' "$INDEX"

for file in "$SURFACE" "$SURFACE_TESTS" "$SEED" "$STATIC" "$PRODUCT" "$POSTPASS" "$FINALIZE" "$SLOTS"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  (( lines < 760 )) || guard_fail "$TAG" "source crossed split trigger: $file ($lines)"
done

echo "[$TAG] one parser surface issuer=1"
echo "[$TAG] seed consumed at product boundary=1"
echo "[$TAG] narrow static seal retains full rows=0"
echo "[$TAG] compiler/policy/physical caller=0"
echo "[$TAG] focused parser evidence=1"
echo "[$TAG] source-size limits=1"
echo "[$TAG] PASS"
