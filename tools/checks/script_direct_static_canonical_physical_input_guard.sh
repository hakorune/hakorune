#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

files=(
  src/mir/builder/normal_script_direct_static_join_handoff.rs
  src/mir/builder/normal_script_direct_static_join_handoff/physical_input.rs
  src/mir/builder/normal_script_direct_static_join_handoff/scalar_operand_recipe.rs
  src/mir/resolved_semantics/expression_source.rs
  src/mir/resolved_semantics/product.rs
)

for file in "${files[@]}"; do
  [[ -f "$file" ]] || { echo "missing canonical physical-input file: $file" >&2; exit 1; }
  lines="$(wc -l < "$file")"
  (( lines < 760 )) || { echo "760-line split trigger exceeded: $file ($lines)" >&2; exit 1; }
done



echo "script direct-static canonical physical input guard: PASS"
