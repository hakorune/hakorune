#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="recipe-tree-carrier-dedup"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

RECIPE_TREE="$ROOT_DIR/src/mir/builder/control_flow/plan/recipe_tree"
MOD_RS="$RECIPE_TREE/mod.rs"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$MOD_RS"

if [[ "$(rg -c '^pub\(super\) struct BuiltRecipeTree' "$MOD_RS" || true)" != "1" ]]; then
  guard_fail "$TAG" "BuiltRecipeTree must have exactly one builder-local definition"
fi

for type_name in \
  AccumConstLoopRecipe ArrayJoinRecipe BoolPredicateScanRecipe CharMapRecipe \
  IfPhiJoinRecipe LoopBreakRecipe LoopContinueOnlyRecipe LoopSimpleWhileRecipe \
  LoopTrueEarlyExitRecipe ScanWithInitRecipe SplitScanRecipe; do
  if rg -q "^pub\\(super\\) struct ${type_name}" "$RECIPE_TREE"; then
    guard_fail "$TAG" "duplicate route-specific Recipe shell remains: $type_name"
  fi
done

while IFS= read -r file; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "RecipeTree source reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done < <(rg --files "$RECIPE_TREE" -g '*.rs')

echo "[$TAG] ok"
