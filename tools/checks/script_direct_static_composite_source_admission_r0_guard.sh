#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

issuer=src/mir/builder/normal_script_composite_partition.rs
loan=src/parser/callable_parameter_source/composite_source/loan.rs
lifecycle=src/mir/builder/normal_default_root_catalog_lifecycle.rs
work_plan=src/mir/builder/program_root_work_plan.rs
decision=src/mir/builder/normal_script_root_admission_witness.rs
window=src/mir/builder/normal_script_root_demand_window.rs
card=docs/development/current/main/investigations/script-direct-static-a-source-capability-d0-2026-08-21.md

for file in "$issuer" "$loan" "$lifecycle" "$work_plan" "$decision" "$window" "$card"; do
  [[ -f "$file" ]] || {
    echo "[script-composite-r0] missing $file" >&2
    exit 1
  }
done

for file in "$issuer" "$loan" "$lifecycle" "$work_plan" "$decision" "$window"; do
  lines="$(wc -l < "$file")"
  (( lines < 760 )) || {
    echo "[script-composite-r0] 760-line split trigger exceeded: $file ($lines)" >&2
    exit 1
  }
done

rg -q 'CanonicalScriptCompositeProgramPartitionIssuerV1::issue' "$lifecycle"
(( "$(rg -c 'CanonicalScriptCompositeProgramPartitionIssuerV1::issue' "$lifecycle")" == 1 ))
rg -q 'with_composite_source_loan' "$issuer"
rg -q "for<'source> FnOnce" "$loan"
rg -q 'StaticCallableCatalogTransfer' "$decision"
rg -q 'record_selected_work_item_with_composite_partition' "$window"
rg -q 'SourceAuthorityUnavailable' "$issuer"
rg -q 'Incomplete' "$issuer"
rg -q 'IntegrityInvalid' "$issuer"
rg -q 'SCRIPT-COMPOSITE-SOURCE-ADMIT-I0-R0' "$card"
rg -q 'resolver, lookup, A/C, physical' "$card"

issuer_impl="$(mktemp)"
product_struct="$(mktemp)"
trap 'rm -f "$issuer_impl" "$product_struct"' EXIT
sed '/^#\[cfg(test)\]/,$d' "$issuer" > "$issuer_impl"
sed -n '/struct CanonicalScriptCompositeProgramPartitionV1[[:space:]]*{/,/^}/p' \
  "$issuer_impl" > "$product_struct"

if rg -n 'ValueId|MirType|BasicBlockId|JoinSig|RecipeKey|ASTNode|&ASTNode' "$product_struct"; then
  echo "[script-composite-r0] downstream/AST authority leaked into the partition product" >&2
  exit 1
fi
if rg -n 'prepare_script_recipe|old_recipe|fallback|retry|compatibility' "$issuer_impl"; then
  echo "[script-composite-r0] old-route or fallback edge leaked into R0 issuer" >&2
  exit 1
fi

echo "script direct-static composite source admission R0 guard: PASS"
