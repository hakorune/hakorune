#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

files=(
  src/mir/builder/normal_script_direct_static_join_handoff.rs
  src/mir/builder/normal_script_direct_static_join_handoff/physical_input.rs
  src/mir/builder/normal_script_direct_static_join_handoff/scalar_operand_recipe.rs
  src/mir/builder/script_physical_exit/direct_static_entry_kernel.rs
  src/mir/builder/script_physical_exit/entry_session.rs
  src/mir/resolved_semantics/expression_source.rs
  src/mir/resolved_semantics/product.rs
)

for file in "${files[@]}"; do
  [[ -f "$file" ]] || { echo "missing canonical physical-input file: $file" >&2; exit 1; }
  lines="$(wc -l < "$file")"
  (( lines < 760 )) || { echo "760-line split trigger exceeded: $file ($lines)" >&2; exit 1; }
done

kernel=src/mir/builder/script_physical_exit/direct_static_entry_kernel.rs
if rg -n 'ASTNode|RawScriptBodyRecipe|lower_and_complete|ScriptPhysicalExitCommitV1|ScriptPhysicalExitCommit' "$kernel"; then
  echo "detached direct-static kernel reached a forbidden source/exit owner" >&2
  exit 1
fi

receipt_count="$(rg -o 'emit_static_global_value_terminal_with_receipt_v1' "$kernel" | wc -l)"
[[ "$receipt_count" -eq 2 ]] || {
  echo "detached kernel must import and invoke exactly one generic Call receipt emitter (count=$receipt_count)" >&2
  exit 1
}

if rg -n 'raw|compat|fallback|retry|ASTNode|ValueId.*MirType|MirType.*ValueId' \
  src/mir/builder/normal_script_direct_static_join_handoff/physical_input.rs \
  src/mir/builder/normal_script_direct_static_join_handoff/scalar_operand_recipe.rs; then
  echo "canonical physical input contains forbidden authority/fallback vocabulary" >&2
  exit 1
fi

echo "script direct-static canonical physical input guard: PASS"
