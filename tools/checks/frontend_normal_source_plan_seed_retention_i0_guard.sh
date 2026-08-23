#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-normal-source-plan-seed-retention-i0"
source "$ROOT/tools/checks/lib/guard_common.sh"

SEED="$ROOT/src/parser/callable_parameter_source/normal_source_plan_seed.rs"
CALLABLE_MOD="$ROOT/src/parser/callable_parameter_source/mod.rs"
INITIAL_ISSUE="$ROOT/src/parser/initial_callable_program_source/issue.rs"
FINALIZER="$ROOT/src/parser/source_seal/finalize.rs"
SOURCE_MODEL="$ROOT/src/parser/source_seal/model.rs"
POSTPASS="$ROOT/src/parser/postpass_envelope.rs"
BODY_SOURCE="$ROOT/src/parser/body_source.rs"
RESOLVER_HANDOFF="$ROOT/src/parser/source_resolver_handoff.rs"
SEAL_TESTS="$ROOT/src/parser/source_seal_finalizer_tests.rs"
README="$ROOT/src/parser/callable_parameter_source/README.md"
CARD="$ROOT/docs/development/current/main/investigations/normal-root-source-plan-seed-retention-i0-2026-08-23.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$SEED" "$CALLABLE_MOD" "$INITIAL_ISSUE" \
  "$FINALIZER" "$SOURCE_MODEL" "$POSTPASS" "$BODY_SOURCE" "$RESOLVER_HANDOFF" \
  "$SEAL_TESTS" "$README" "$CARD" "$INDEX"

[[ "$(rg -c 'pub\(in crate::parser\) struct ParserNormalSourcePlanSeedV1' "$SEED")" == 1 ]]
[[ "$(rg -c 'pub\(in crate::parser\) fn issue\(' "$SEED")" == 1 ]]
[[ "$(rg -c 'ParserNormalSourcePlanSeedV1::issue\(' "$FINALIZER")" == 1 ]]
[[ "$(rg -c 'normal_source_plan_seed: ParserNormalSourcePlanSeedV1' "$SOURCE_MODEL")" == 1 ]]
[[ "$(rg -c 'normal_source_plan_seed: ParserNormalSourcePlanSeedDispositionV1' "$POSTPASS")" == 4 ]]
[[ "$(rg -c 'program_slots: &ProjectedProgramItemSlotSetV1' "$INITIAL_ISSUE")" == 2 ]]
if rg -n 'program_slots: Option<ProjectedProgramItemSlotSetV1>' "$INITIAL_ISSUE"; then
  guard_fail "$TAG" "initial source issuer still accepts an optional slot relation"
fi

rg -q 'mod normal_source_plan_seed;' "$CALLABLE_MOD"
rg -q 'normal_source_plan_seed\.projected_program_slots\(\)' "$FINALIZER"
rg -q 'normal_source_plan_seed,' "$SOURCE_MODEL"
rg -q 'normal_source_plan_seed\.discard_unconnected\(\);' "$POSTPASS"
rg -q 'seed\.discard_unconnected\(\);' "$BODY_SOURCE"
rg -q 'seed\.discard_unconnected\(\);' "$RESOLVER_HANDOFF"
rg -q 'normal_source_plan_seed' "$SEAL_TESTS"

if rg -n '_prepared_static_box_sources' "$FINALIZER"; then
  guard_fail "$TAG" "ordinary finalizer still discards prepared static-parent rows"
fi

if rg -n -B1 'pub\(in crate::parser\) struct ParserNormalSourcePlanSeedV1' "$SEED" \
  | rg -n 'Clone|Copy'; then
  guard_fail "$TAG" "source-plan seed became cloneable"
fi

if rg -n 'ASTNode|NormalCompileRequest|MirBuilder|ValueId|BasicBlockId|MirType|Recipe|Join|fallback|retry' "$SEED"; then
  guard_fail "$TAG" "seed leaked semantic, physical, or fallback authority"
fi

if rg -n 'ParserNormalSourcePlanSeedV1' "$ROOT/src/parser/initial_callable_program_source"; then
  guard_fail "$TAG" "initial callable source stores or consumes the seed"
fi

for file in "$SEED" "$CALLABLE_MOD" "$INITIAL_ISSUE" "$FINALIZER" "$SOURCE_MODEL" \
  "$POSTPASS" "$BODY_SOURCE" "$RESOLVER_HANDOFF" "$SEAL_TESTS"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  (( lines < 760 )) || guard_fail "$TAG" "source crossed split trigger: $file ($lines)"
done

for needle in \
  'missing_projected_slots_are_not_an_empty_seed' \
  'normal_source_plan_seed' \
  'Normal source-plan seed retention I0'; do
  rg -q "$needle" "$SEED" "$SEAL_TESTS" "$README" || \
    guard_fail "$TAG" "missing focused evidence or owner documentation: $needle"
done

rg -q 'frontend_normal_source_plan_seed_retention_i0_guard.sh' "$INDEX" || \
  guard_fail "$TAG" "check index does not list the seed-retention guard"

echo "[$TAG] sole seed issuer=1"
echo "[$TAG] projected slots borrowed, initial rows owned"
echo "[$TAG] prepared static rows are retained, not discarded"
echo "[$TAG] seed semantic/physical/fallback authority=0"
echo "[$TAG] touched source-size limits=1"
echo "[$TAG] PASS"
