#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

neutral=src/mir/builder/normal_script_neutral_window.rs
neutral_tests=src/mir/builder/normal_script_neutral_window_tests.rs
lifecycle=src/mir/builder/normal_default_root_catalog_lifecycle.rs
work_plan=src/mir/builder/program_root_work_plan.rs
work_plan_production=src/mir/builder/program_root_work_plan_production.rs
legacy_window=src/mir/builder/normal_script_root_demand_window.rs
builder_barrel=src/mir/builder.rs
instance_transfer=src/mir/builder/normal_script_instance_box_transfer.rs
constructor=src/mir/builder/normal_instance_constructor_admission.rs
card=docs/development/current/main/investigations/script-direct-static-a-source-capability-d0-2026-08-21.md

for file in "$neutral" "$neutral_tests" "$lifecycle" "$work_plan" \
  "$work_plan_production" "$legacy_window" "$builder_barrel" \
  "$instance_transfer" "$constructor" "$card"; do
  [[ -f "$file" ]] || {
    echo "[script-source-reown-r0] missing $file" >&2
    exit 1
  }
done

for file in "$neutral" "$neutral_tests" "$lifecycle" "$work_plan" \
  "$work_plan_production" "$legacy_window" "$instance_transfer" "$constructor"; do
  lines="$(wc -l < "$file")"
  (( lines < 760 )) || {
    echo "[script-source-reown-r0] 760-line split trigger exceeded: $file ($lines)" >&2
    exit 1
  }
done

rg -q 'PreparedCanonicalScriptNeutralProgramWindowV1::issue\(package\)' "$lifecycle"
(( "$(rg -c 'PreparedCanonicalScriptNeutralProgramWindowV1::issue\(package\)' "$lifecycle")" == 1 ))
rg -q 'CanonicalScriptCompositeProgramPartitionIssuerV1::issue_from_program_loan' "$neutral"
rg -q 'VerifiedScriptInstanceBoxTransferCohortV1::issue_from_program_loan' "$neutral"
rg -q 'VerifiedInstanceConstructorPhysicalSourceCohortV1::issue_from_program_loan' "$neutral"
(( "$(rg -c 'VerifiedScriptRootDemandWindowV1::seal' "$neutral")" == 1 ))
rg -q 'with_normal_program_source_loan' "$neutral"

# The old Builder/window/occurrence route remains only in test fixtures.
if rg -n \
  'ScriptRootDemandWindowBuilderV1::|ScriptRootSemanticDecisionV1::|SelectedScriptProgramOccurrenceV1::|CanonicalScriptCompositeProgramPartitionIssuerV1::issue\(|VerifiedScriptInstanceBoxTransferCohortV1::issue\(|VerifiedInstanceConstructorPhysicalSourceCohortV1::issue\(' \
  "$lifecycle" "$work_plan_production" "$neutral"; then
  echo "[script-source-reown-r0] legacy Builder/source issuer leaked into production edge" >&2
  exit 1
fi

rg -U -q '#\[cfg\(test\)\][[:space:]]*mod normal_script_root_admission_witness' "$builder_barrel"
rg -U -q '#\[cfg\(test\)\][[:space:]]*mod normal_script_selected_occurrence' "$builder_barrel"
legacy_builder_line="$(rg -n '^pub\(super\) struct ScriptRootDemandWindowBuilderV1' "$legacy_window" | cut -d: -f1)"
legacy_cfg_line="$(sed -n "1,${legacy_builder_line}p" "$legacy_window" | rg -n '^#\[cfg\(test\)\]' | tail -n 1 | cut -d: -f1)"
[[ -n "$legacy_builder_line" && -n "$legacy_cfg_line" && "$legacy_cfg_line" -lt "$legacy_builder_line" ]]

# Neutral source and lookup failure must precede target installation and
# Builder effects.
neutral_line="$(rg -n 'PreparedCanonicalScriptNeutralProgramWindowV1::issue\(package\)' "$lifecycle" | cut -d: -f1)"
lookup_line="$(rg -n 'ScriptDirectStaticCallLookupIssuerV1::issue' "$lifecycle" | cut -d: -f1)"
install_line="$(rg -n 'install_pinned_text_target_capability' "$lifecycle" | tail -n 1 | cut -d: -f1)"
effect_line="$(rg -n 'prepare_normal_default_module' "$lifecycle" | cut -d: -f1)"
(( neutral_line < lookup_line && lookup_line < install_line && install_line < effect_line )) || {
  echo "[script-source-reown-r0] neutral/lookup failure is not before target/effects" >&2
  exit 1
}

product_struct="$(sed -n '/pub(super) struct PreparedCanonicalScriptNeutralProgramWindowV1/,/^}/p' "$neutral")"
if printf '%s\n' "$product_struct" | rg -n 'ASTNode|ValueId|MirType|BasicBlockId|RecipeKey|JoinSig|\*const|as \*const'; then
  echo "[script-source-reown-r0] neutral aggregate leaked AST/downstream identity" >&2
  exit 1
fi
if rg -n 'VerifiedScriptDirectStaticCallTargetInventoryV1|prepare_script_recipe|old_recipe|fallback|retry' \
  "$neutral" "$work_plan_production"; then
  echo "[script-source-reown-r0] lookup/Recipe/fallback authority leaked into neutral transport" >&2
  exit 1
fi

rg -q 'source_row\(\)' "$instance_transfer"
if rg -n 'statement_positions|BTreeSet<u32>' "$instance_transfer"; then
  echo "[script-source-reown-r0] instance transfer retained ordinal-set authority" >&2
  exit 1
fi
if rg -n 'source_ast\(\)[[:space:]]*,[[:space:]]*package|issue\(package\.source_ast' \
  "$lifecycle" "$neutral"; then
  echo "[script-source-reown-r0] lifecycle retained a second AST source issuer" >&2
  exit 1
fi

rg -q 'SCRIPT-SOURCE-REOWN-I0-R0' "$card"
rg -q 'neutral window' "$card"
rg -q 'Builder.*caller' "$card"

echo "script direct-static source reownership window R0 guard: PASS"
