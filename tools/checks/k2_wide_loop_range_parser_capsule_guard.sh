#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-loop-range-parser-capsule"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-272-LOOP-002-STAGE0-LOOPRANGE-PARSER-CAPSULE.md"
DESIGN="docs/development/current/main/design/loop-range-parser-capsule-ssot.md"
TASKS="docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md"
EBNF="docs/reference/language/EBNF.md"
PARSER="src/parser/statements/control_flow.rs"
AST_JSON="src/macro/ast_json/joinir_compat.rs"
ROUNDTRIP="src/macro/ast_json/roundtrip.rs"
PROGRAM_JSON="src/stage1/program_json_v0/lowering.rs"
PARSER_TEST="src/tests/parser_loop_scan_range_shape.rs"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
SELF_SCRIPT="tools/checks/k2_wide_loop_range_parser_capsule_guard.sh"

echo "[$TAG] checking LOOP-002 LoopRange parser capsule"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$DESIGN" \
  "$TASKS" \
  "$EBNF" \
  "$PARSER" \
  "$AST_JSON" \
  "$ROUNDTRIP" \
  "$PROGRAM_JSON" \
  "$PARSER_TEST" \
  "$INDEX" \
  "$CURRENT_STATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "LOOP-002 card must be complete"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "LOOP-002 design must be accepted"
guard_expect_in_file "$TAG" 'LOOP-002 status' "$TASKS" "language task breakdown must record LOOP-002 status"
guard_expect_in_file "$TAG" "loop_range_head := IDENT 'in' expr '..' expr" "$EBNF" "EBNF must describe loop range header"
guard_expect_in_file "$TAG" 'current_loop_range_header_starts' "$PARSER" "parser must detect loop range header"
guard_expect_in_file "$TAG" 'parse_loop_range_header' "$PARSER" "parser must parse loop range header"
guard_expect_in_file "$TAG" 'ASTNode::ForRange' "$PARSER" "parser must transport LoopRange through ForRange AST metadata"
guard_expect_in_file "$TAG" '"kind": "LoopRange"' "$AST_JSON" "AST JSON must emit LoopRange kind"
guard_expect_in_file "$TAG" '"LoopRange" | "ForRange"' "$ROUNDTRIP" "roundtrip JSON must decode LoopRange metadata"
guard_expect_in_file "$TAG" '"type": "LoopRange"' "$PROGRAM_JSON" "Program JSON v0 must transport LoopRange metadata"
guard_expect_in_file "$TAG" 'parser_loop_range_surface_parses_parenless_loop_header' "$PARSER_TEST" "parser test must cover paren-less LoopRange"
guard_expect_in_file "$TAG" 'parser_loop_range_surface_parses_parenthesized_loop_header' "$PARSER_TEST" "parser test must cover parenthesized LoopRange"
guard_expect_in_file "$TAG" 'parser_loop_condition_surface_accepts_parenless_loop_condition' "$PARSER_TEST" "parser test must cover paren-less condition loop"
guard_expect_in_file "$TAG" '293x-272 LOOP-002 loop range parser capsule landed' "$CURRENT_STATE" "current state landed tail must retain LOOP-002"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list LOOP-002 guard"

if rg -n 'while keyword|for keyword as canonical|lower_for_range|LoopRange lowering|continue-safe|readonly index|read-only index|bounds facts' \
  "$PARSER" "$AST_JSON" "$PROGRAM_JSON" >/tmp/"$TAG".semantic_leak 2>&1; then
  echo "[$TAG] ERROR: LOOP-002 must remain parser/metadata only" >&2
  cat /tmp/"$TAG".semantic_leak >&2
  rm -f /tmp/"$TAG".semantic_leak
  exit 1
fi
rm -f /tmp/"$TAG".semantic_leak

cargo test -q parser_loop_range_surface

echo "[$TAG] ok"
