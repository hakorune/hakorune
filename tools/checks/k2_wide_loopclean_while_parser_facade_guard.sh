#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-loopclean-while-parser-facade"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-406-LOOPCLEAN-006-WHILE-PARSER-FACADE-MERGE.md"
SIDE_BAND="docs/development/current/main/design/compiler-cleanup-sidecar-task-breakdown-ssot.md"
LOOP_SSOT="docs/development/current/main/design/loop-cleanup-before-packedarray-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PARSER="src/parser/statements/control_flow.rs"
TEST_FILE="src/tests/parser_loop_cleanup_surface.rs"
SELF_SCRIPT="tools/checks/k2_wide_loopclean_while_parser_facade_guard.sh"

echo "[$TAG] running LOOPCLEAN-006 while parser facade guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$SIDE_BAND" \
  "$LOOP_SSOT" \
  "$TASKBOARD" \
  "$INDEX" \
  "$PARSER" \
  "$TEST_FILE" \
  "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'LOOPCLEAN-006' "$CARD" "cleanup card must exist"
guard_expect_in_file "$TAG" 'same canonical loop parser entry' "$CARD" "card must state the facade merge"
guard_expect_in_file "$TAG" 'TokenType::WHILE if stage3 => self\.parse_loop\(\)' "$PARSER" "while compatibility must route through parse_loop"
guard_expect_in_file "$TAG" 'is_while_compat' "$PARSER" "parse_loop must own the while compatibility branch"
guard_expect_in_file "$TAG" 'while cond \{ \.\.\. \}` parses to the same `ASTNode::Loop` output' "$PARSER" "parse_loop docs must state while compatibility output"
guard_expect_in_file "$TAG" 'parser_loopclean_while_stage3_normalizes_to_loop_ast' "$TEST_FILE" "while normalization fixture missing"
guard_expect_in_file "$TAG" 'LOOPCLEAN-006' "$SIDE_BAND" "cleanup sidecar SSOT must list LOOPCLEAN-006"
guard_expect_in_file "$TAG" 'LOOPCLEAN-006 while parser facade merge' "$LOOP_SSOT" "loop cleanup SSOT must list LOOPCLEAN-006"
guard_expect_in_file "$TAG" 'LOOPCLEAN-006' "$TASKBOARD" "taskboard must list LOOPCLEAN-006"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

if hits="$(rg -n 'parse_while_stage3' src || true)"; then
  if [[ -n "$hits" ]]; then
    echo "[$TAG] ERROR: stale parse_while_stage3 facade found" >&2
    printf '%s\n' "$hits" >&2
    exit 1
  fi
fi

cargo test -q --lib parser_loopclean_while_stage3_normalizes_to_loop_ast

echo "[$TAG] ok"
