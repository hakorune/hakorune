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
LOWERING_STATE=src/mir/builder/normal_script_semantic_lowering_state.rs
ROOT_TRAVERSAL=src/mir/resolved_semantics/shadow/root_traversal.rs
BUILDER_README=src/mir/builder/README.md
CARD=docs/development/current/main/investigations/script-direct-static-call-target-d0.md

require_text "$MODULE" "VerifiedScriptDirectStaticCallTargetInventoryV1"
require_text "$MODULE" "observe_script_method_calls_shadow_view_v0"
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
require_text "$RECIPE_TESTS" "complete_empty_owner_emits_a_valid_empty_recipe"
require_text "$LOWERING_STATE" "direct_static_recipe"
require_text "$ROOT_TRAVERSAL" "record_statement_shape"
require_text "$BUILDER_README" "VerifiedScriptSourceContinuationV1"
require_text "$BUILDER_README" "source/Facts-only"
require_text "$CARD" "SCRIPT-DIRECT-STATIC-CALL-SOURCE-CONTINUATION-I0"
require_text "$CARD" "source-only continuation rows"
require_text "$CARD" "result publication, and physical lowering"

for file in "$MODULE" "$TESTS" "$BUNDLE" "$BUNDLE_TESTS" "$ADMISSION" "$LIFECYCLE" "$SEMANTIC_SOURCE" "$CONTINUATION" "$CONTINUATION_TESTS" "$LOWERING_INPUT" "$LOWERING_STATE" "$RESULT_OWNER" "$RESULT_OWNER_TESTS" "$RECIPE" "$RECIPE_TESTS" "$ROOT_TRAVERSAL" "$BUILDER_README"; do
  lines="$(wc -l < "$file")"
  if (( lines >= 760 )); then
    echo "[script-direct-static-target] source split required: $file has $lines lines" >&2
    exit 1
  fi
done

if rg -n "raw_root_body_recipe|JoinSig|lower_.*physical|emit_.*call" "$MODULE"; then
  echo "[script-direct-static-target] observation module crossed the Recipe/physical boundary" >&2
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

if rg -n "raw_root_body_recipe|JoinSig|lower_.*physical|emit_.*call" "$CONTINUATION" "$LOWERING_INPUT"; then
  echo "[script-direct-static-target] continuation crossed the Recipe/physical boundary" >&2
  exit 1
fi

CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  mir::source_call_target::script_direct_static_tests --lib
CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  mir::builder::normal_script_direct_static_result_bundle --lib
CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  mir::builder::normal_script_direct_static_recipe --lib
CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  normal_script_source_continuation_tests --lib

echo "[script-direct-static-target] OK"
