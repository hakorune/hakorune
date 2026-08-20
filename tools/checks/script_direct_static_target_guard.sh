#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  grep -Fq "$text" "$file" || {
    echo "[script-direct-static-target] missing '$text' in $file" >&2
    exit 1
  }
}

MODULE=src/mir/source_call_target/script_direct_static.rs
TESTS=src/mir/source_call_target/script_direct_static_tests.rs
TYPEOP_POLICY=src/mir/policies/source_method_typeop_route.rs
TYPEOP_TESTS=src/mir/policies/source_method_typeop_route_tests.rs
SPECIAL_HANDLERS=src/mir/builder/calls/special_handlers.rs
CALL_BUILD=src/mir/builder/calls/build.rs
BUNDLE=src/mir/builder/normal_script_direct_static_result_bundle.rs
BUNDLE_TESTS=src/mir/builder/normal_script_direct_static_result_bundle_tests.rs
ADMISSION=src/mir/builder/normal_script_root_demand_window.rs
LIFECYCLE=src/mir/builder/normal_default_root_catalog_lifecycle.rs
SEMANTIC_SOURCE=src/mir/builder/normal_script_semantic_source.rs
CONTINUATION=src/mir/builder/normal_script_source_continuation.rs
CONTINUATION_TESTS=src/mir/builder/normal_script_source_continuation_tests.rs
LOWERING_INPUT=src/mir/builder/normal_script_semantic_lowering_input.rs
RESULT_OWNER=src/mir/builder/normal_script_direct_static_result_publication_owner.rs
RESULT_OWNER_TESTS=src/mir/builder/normal_script_direct_static_result_publication_owner_tests.rs
RECIPE=src/mir/builder/normal_script_direct_static_recipe.rs
RECIPE_TESTS=src/mir/builder/normal_script_direct_static_recipe_tests.rs
JOIN_HANDOFF=src/mir/builder/normal_script_direct_static_join_handoff.rs
JOIN_HANDOFF_TESTS=src/mir/builder/normal_script_direct_static_join_handoff_tests.rs
LOWERING_STATE=src/mir/builder/normal_script_semantic_lowering_state.rs
CLAIM_LEDGER=src/mir/builder/normal_script_direct_static_claim_ledger.rs
CLAIM_LEDGER_TESTS=src/mir/builder/normal_script_direct_static_claim_ledger_tests.rs
ROOT_TRAVERSAL=src/mir/resolved_semantics/shadow/root_traversal.rs
BUILDER_README=src/mir/builder/README.md
CARD=docs/development/current/main/investigations/script-direct-static-call-target-d0.md
SOURCE_FINALIZER=src/parser/source_seal/finalize.rs
SOURCE_TESTS=src/parser/normal_callable_program_source/tests.rs

require_text "$MODULE" "VerifiedScriptDirectStaticCallTargetInventoryV1"
require_text "$MODULE" "observe_script_method_calls_shadow_view_v0"
require_text "$MODULE" "classify_source_method_typeop_route_v1"
require_text "$TYPEOP_POLICY" "SourceMethodTypeOpDispositionV1"
require_text "$TYPEOP_POLICY" "classify_source_method_typeop_route_v1"
require_text "$TYPEOP_POLICY" "SourceMethodTypeOpKindV1"
require_text "$TYPEOP_TESTS" "direct_string_typeops_are_typed_non_candidates"
require_text "$SPECIAL_HANDLERS" "classify_source_method_typeop_route_v1"
require_text "$CALL_BUILD" "SourceMethodTypeOpDispositionV1"
require_text "$MODULE" "TargetOutsideCatalog"
require_text "$ADMISSION" "attach_script_direct_static_targets"
require_text "$LIFECYCLE" "VerifiedScriptDirectStaticCallTargetInventoryV1::issue"
require_text "$BUNDLE" "VerifiedScriptDirectStaticResultBundleV1"
require_text "$BUNDLE" "TargetInventoryBrandMismatch"
require_text "$SEMANTIC_SOURCE" "attach_direct_static_result_bundle"
require_text "$CONTINUATION" "VerifiedScriptSourceContinuationV1"
require_text "$CONTINUATION" "validate_statement_window"
require_text "$LOWERING_INPUT" "VerifiedScriptSemanticLoweringInputV1"
require_text "$RESULT_OWNER" "VerifiedScriptDirectStaticResultPublicationOwnerV1"
require_text "$RESULT_OWNER" "BundleSourceMismatch"
require_text "$RESULT_OWNER" "ContinuationMissing"
require_text "$RESULT_OWNER_TESTS" "owner_accepts_a_complete_script_source_bundle"
require_text "$RESULT_OWNER_TESTS" "owner_rejects_a_bundle_from_a_foreign_source"
require_text "$RECIPE" "VerifiedScriptDirectStaticRecipeV1"
require_text "$RECIPE" "FinalSequence"
require_text "$RECIPE" "RootReturn"
require_text "$RECIPE" "MissingFinalValueRelation"
require_text "$RECIPE" "TerminalRelationMismatch"
require_text "$RECIPE_TESTS" "complete_empty_owner_emits_a_valid_empty_recipe"
require_text "$RECIPE_TESTS" "terminal_shape_accepts_bare_final_sequence_call"
require_text "$RECIPE_TESTS" "terminal_shape_rejects_final_local_value_as_sequence_result"
require_text "$RECIPE_TESTS" "terminal_shape_accepts_direct_root_return_value"
require_text "$RECIPE_TESTS" "terminal_shape_rejects_nested_return_call"
require_text "$JOIN_HANDOFF" "VerifiedScriptDirectStaticJoinHandoffV1"
require_text "$JOIN_HANDOFF" "SourceIdentityMismatch"
require_text "$JOIN_HANDOFF" "PublicationRowMissing"
require_text "$JOIN_HANDOFF" "RecipeRowMissing"
require_text "$JOIN_HANDOFF_TESTS" "empty_recipe_emits_empty_join_handoff"
require_text "$JOIN_HANDOFF_TESTS" "join_handoff_rejects_a_foreign_source_owner"
require_text "$JOIN_HANDOFF_TESTS" "non_empty_recipe_row_is_carried_by_recipe_key"
require_text "$LOWERING_STATE" "direct_static_recipe"
require_text "$LOWERING_STATE" "direct_static_claim_ledger"
require_text "$LOWERING_STATE" "take_direct_static_claim"
require_text "$LOWERING_STATE" "complete_direct_static_claim"
require_text "$LOWERING_STATE" "finish_direct_static_claims"
require_text "$CLAIM_LEDGER" "ScriptDirectStaticClaimLedgerV1"
require_text "$CLAIM_LEDGER" "PartialSourceProducts"
require_text "$CLAIM_LEDGER" "DuplicateClaim"
require_text "$CLAIM_LEDGER" "PendingRows"
require_text "$CLAIM_LEDGER_TESTS" "complete_pair_is_claimed_once_and_finishes_exhausted"
require_text "$CLAIM_LEDGER_TESTS" "partial_source_products_are_rejected_before_claiming"
require_text "$CLAIM_LEDGER_TESTS" "finish_rejects_unclaimed_rows_without_mutating_the_source_products"
require_text "$ROOT_TRAVERSAL" "record_statement_shape"
require_text "$BUILDER_README" "VerifiedScriptSourceContinuationV1"
require_text "$BUILDER_README" "source/Facts-only"
require_text "$BUILDER_README" "ScriptDirectStaticClaimLedgerV1"
require_text "$BUILDER_README" "there is no rollback"
require_text "$CARD" "SCRIPT-DIRECT-STATIC-CALL-SOURCE-CONTINUATION-I0"
require_text "$CARD" "source-only continuation rows"
require_text "$CARD" "result publication, and physical lowering"
require_text "$SOURCE_FINALIZER" "finalize_compatibility_source"
require_text "$SOURCE_FINALIZER" "attach_constructor_source"
require_text "$SOURCE_TESTS" "exact_static_callable_set_survives_one_transform"
require_text "$SOURCE_TESTS" "ordinary_constructor_source_catalog_survives_normal_source_transform"
require_text "$SOURCE_TESTS" "unsupported_compatibility_cohorts_do_not_enter_initial_source_lane"

for file in "$MODULE" "$TESTS" "$TYPEOP_POLICY" "$TYPEOP_TESTS" "$SPECIAL_HANDLERS" "$CALL_BUILD" "$BUNDLE" "$BUNDLE_TESTS" "$ADMISSION" "$LIFECYCLE" "$SEMANTIC_SOURCE" "$CONTINUATION" "$CONTINUATION_TESTS" "$LOWERING_INPUT" "$LOWERING_STATE" "$CLAIM_LEDGER" "$CLAIM_LEDGER_TESTS" "$RESULT_OWNER" "$RESULT_OWNER_TESTS" "$RECIPE" "$RECIPE_TESTS" "$JOIN_HANDOFF" "$JOIN_HANDOFF_TESTS" "$ROOT_TRAVERSAL" "$BUILDER_README" "$SOURCE_FINALIZER" "$SOURCE_TESTS"; do
  lines="$(wc -l < "$file")"
  if (( lines >= 760 )); then
    echo "[script-direct-static-target] source split required: $file has $lines lines" >&2
    exit 1
  fi
done

TOKEN_HEADER="$(rg -B 2 -n "struct ScriptDirectStaticClaimedRowV1" "$CLAIM_LEDGER" || true)"
if printf '%s\n' "$TOKEN_HEADER" | rg -n "Clone"; then
  echo "[script-direct-static-target] claim token became Clone" >&2
  exit 1
fi

if rg -n "rollback|reinsert|put_back|emit_.*call|lower_.*physical|MirType|ValueId|ScriptPhysicalExit" "$CLAIM_LEDGER"; then
  echo "[script-direct-static-target] claim ledger crossed the operational-only boundary" >&2
  exit 1
fi

if rg -n "emit_.*call|MirType|ValueId|ScriptPhysicalExit|raw_invocation_source_transport" "$CLAIM_LEDGER_TESTS"; then
  echo "[script-direct-static-target] claim ledger tests crossed the physical boundary" >&2
  exit 1
fi

if rg -n "raw_root_body_recipe|JoinSig|lower_.*physical|emit_.*call" "$MODULE"; then
  echo "[script-direct-static-target] observation module crossed the Recipe/physical boundary" >&2
  exit 1
fi

if rg -n 'method == "is"|method == "as"' "$SPECIAL_HANDLERS" "$CALL_BUILD" "$MODULE"; then
  echo "[script-direct-static-target] typeop method spelling was duplicated outside the shared policy" >&2
  exit 1
fi

if rg -n "raw_root_body_recipe|normal_source_plan|JoinSig|ValueId|MirType|lower_.*physical|emit_.*call" "$RECIPE"; then
  echo "[script-direct-static-target] dedicated Recipe crossed the scalar/physical boundary" >&2
  exit 1
fi

if rg -n "crate::.*(JoinSig|ValueId|MirType)|raw_root_body_recipe::|fn lower_.*physical|fn emit_.*call" "$RESULT_OWNER"; then
  echo "[script-direct-static-target] result owner crossed the source/Facts boundary" >&2
  exit 1
fi

if rg -n "raw_root_body_recipe|JoinSig|ValueId|MirType|lower_.*physical|emit_.*call" "$JOIN_HANDOFF"; then
  echo "[script-direct-static-target] Join handoff crossed the source/Facts boundary" >&2
  exit 1
fi

if rg -n "raw_root_body_recipe|JoinSig|lower_.*physical|emit_.*call" "$CONTINUATION" "$LOWERING_INPUT"; then
  echo "[script-direct-static-target] continuation crossed the Recipe/physical boundary" >&2
  exit 1
fi

CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  mir::source_call_target::script_direct_static_tests --lib
CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  mir::policies::source_method_typeop_route_tests --lib
CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  mir::builder::normal_script_direct_static_result_bundle --lib
CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  mir::builder::normal_script_direct_static_recipe --lib
CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  mir::builder::normal_script_direct_static_join_handoff --lib
CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  mir::builder::normal_script_semantic_lowering_state::direct_static_claim_ledger::tests --lib
CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  normal_script_source_continuation_tests --lib

echo "[script-direct-static-target] OK"
