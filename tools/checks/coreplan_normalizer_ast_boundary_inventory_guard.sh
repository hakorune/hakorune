#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="coreplan-normalizer-ast-boundary"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

TASKBOARD="docs/development/current/main/workstreams/compiler-foundation-current.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1008-COREPLAN-D1-001-NORMALIZER-AST-BOUNDARY-INVENTORY.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/coreplan_normalizer_ast_boundary_inventory_guard.sh"
NORMALIZER_DIR="src/mir/builder/control_flow/plan/normalizer"
RECIPE_TREE_DIR="src/mir/builder/control_flow/plan/recipe_tree"
NORMALIZER_README="src/mir/builder/control_flow/plan/normalizer/README.md"

echo "[$TAG] checking normalizer AST-boundary inventory"

guard_require_files \
  "$TAG" \
  "$TASKBOARD" \
  "$CARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$NORMALIZER_README"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" \
  "COREPLAN-D1-001: normalizer AST-boundary inventory" \
  "$TASKBOARD" \
  "taskboard must keep D1 row"
guard_expect_fixed_in_file "$TAG" \
  "normalizer_ast_boundary_inventory=1" \
  "$TASKBOARD" \
  "taskboard must record D1 inventory acceptance"
guard_expect_fixed_in_file "$TAG" \
  "$SELF_SCRIPT" \
  "$CARD" \
  "D1 card must name this guard"
guard_expect_fixed_in_file "$TAG" \
  "report-only" \
  "$CARD" \
  "D1 card must keep inventory guard report-only"
guard_expect_fixed_in_file "$TAG" \
  "$SELF_SCRIPT" \
  "$INDEX" \
  "check index must list this guard"
guard_expect_fixed_in_file "$TAG" \
  "Composer/entry 経路では使わない" \
  "$NORMALIZER_README" \
  "normalizer README must keep legacy/analysis boundary"

normalizer_ast_hits="$(rg -n 'ASTNode::' "$NORMALIZER_DIR" -g '*.rs' | wc -l | tr -d ' ')"
normalizer_ast_files="$(rg -l 'ASTNode::' "$NORMALIZER_DIR" -g '*.rs' | wc -l | tr -d ' ')"
synthetic_loop_hits="$(rg -n 'ASTNode::Loop' "$RECIPE_TREE_DIR" -g '*.rs' | wc -l | tr -d ' ')"

echo "[$TAG] normalizer_ast_hit_count=$normalizer_ast_hits"
echo "[$TAG] normalizer_ast_file_count=$normalizer_ast_files"
echo "[$TAG] recipe_tree_synthetic_ast_loop_count=$synthetic_loop_hits"
echo "[$TAG] ok"
