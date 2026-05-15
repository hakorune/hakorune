#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-looprange-ast-rename"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-405-LOOPCLEAN-005-FORRANGE-TO-LOOPRANGE-AST-RENAME.md"
SSOT="docs/development/current/main/design/loop-range-parser-capsule-ssot.md"
TASKS="docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
AST_MOD="src/ast/mod.rs"
NODE_TYPE="src/ast/utils/node_type.rs"
ROUNDTRIP="src/macro/ast_json/roundtrip.rs"
JOINIR_COMPAT="src/macro/ast_json/joinir_compat.rs"
LOWERING="src/stage1/program_json_v0/lowering.rs"
PARSER="src/parser/statements/control_flow.rs"
TEST_FILE="src/tests/parser_loop_scan_range_shape.rs"
SELF_SCRIPT="tools/checks/k2_wide_looprange_ast_rename_guard.sh"

echo "[$TAG] running LOOPCLEAN-005 LoopRange AST rename guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$SSOT" \
  "$TASKS" \
  "$INDEX" \
  "$AST_MOD" \
  "$NODE_TYPE" \
  "$ROUNDTRIP" \
  "$JOINIR_COMPAT" \
  "$LOWERING" \
  "$PARSER" \
  "$TEST_FILE" \
  "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'LOOPCLEAN-005' "$CARD" "cleanup card must exist"
guard_expect_in_file "$TAG" 'ASTNode::LoopRange' "$CARD" "card must name the LoopRange AST target"
guard_expect_in_file "$TAG" 'ForRange legacy compatibility input' "$CARD" "card must preserve old JSON decode compatibility"
guard_expect_in_file "$TAG" 'ASTNode::LoopRange' "$SSOT" "LoopRange SSOT must record the AST rename"
guard_expect_in_file "$TAG" 'LOOPCLEAN-005 LoopRange AST rename' "$TASKS" "task breakdown must mark LOOPCLEAN-005 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

guard_expect_in_file "$TAG" 'LoopRange \{' "$AST_MOD" "AST must define LoopRange variant"
guard_expect_in_file "$TAG" 'ASTNode::LoopRange \{ .. \} => "LoopRange"' "$NODE_TYPE" "node_type must report LoopRange"
guard_expect_in_file "$TAG" '"LoopRange" \| "ForRange" => ASTNode::LoopRange' "$ROUNDTRIP" "roundtrip reader must decode old ForRange into LoopRange"
guard_expect_in_file "$TAG" '"LoopRange" \| "ForRange" => ASTNode::LoopRange' "$JOINIR_COMPAT" "JoinIR compat reader must decode old ForRange into LoopRange"
guard_expect_in_file "$TAG" '"type": "LoopRange"' "$LOWERING" "Program JSON emission must remain LoopRange"
guard_expect_in_file "$TAG" 'Ok\(ASTNode::LoopRange' "$PARSER" "parser must normalize range headers to LoopRange"
guard_expect_in_file "$TAG" 'ASTNode::LoopRange' "$TEST_FILE" "parser fixtures must assert LoopRange metadata"

if hits="$(rg -n 'ASTNode::ForRange|ForRange \{' src || true)"; then
  if [[ -n "$hits" ]]; then
    echo "[$TAG] ERROR: stale ForRange AST variant reference found" >&2
    printf '%s\n' "$hits" >&2
    exit 1
  fi
fi

cargo test -q --lib parser_loop_range_surface
cargo test -q --lib parser_legacy_for_range_surface

echo "[$TAG] ok"
