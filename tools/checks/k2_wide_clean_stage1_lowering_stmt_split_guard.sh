#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-clean-stage1-lowering-stmt-split"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-407-CLEAN-STAGE1-LOWERING-002-STMT-SPLIT.md"
SIDE_BAND="docs/development/current/main/design/compiler-cleanup-sidecar-task-breakdown-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
LOWERING="src/stage1/program_json_v0/lowering.rs"
TEST_FILE="src/stage1/program_json_v0/tests/basics_and_enums.rs"
SELF_SCRIPT="tools/checks/k2_wide_clean_stage1_lowering_stmt_split_guard.sh"

echo "[$TAG] running CLEAN-STAGE1-LOWERING-002 statement split guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$SIDE_BAND" \
  "$TASKBOARD" \
  "$INDEX" \
  "$LOWERING" \
  "$TEST_FILE" \
  "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'CLEAN-STAGE1-LOWERING-002' "$CARD" "statement split card must exist"
guard_expect_in_file "$TAG" '## TODO' "$CARD" "statement split card must keep TODO checklist"
guard_expect_in_file "$TAG" 'CLEAN-LOWER-002.*landed' "$SIDE_BAND" "cleanup sidecar SSOT must mark CLEAN-LOWER-002 landed"
guard_expect_in_file "$TAG" 'CLEAN-STAGE1-LOWERING-002' "$TASKBOARD" "taskboard must list CLEAN-STAGE1-LOWERING-002"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

guard_expect_in_file "$TAG" 'fn local_statement_to_json_v0_many' "$LOWERING" "local multi-output helper missing"
guard_expect_in_file "$TAG" 'fn assignment_statement_to_json_v0' "$LOWERING" "assignment helper missing"
guard_expect_in_file "$TAG" 'fn print_statement_to_json_v0' "$LOWERING" "print helper missing"
guard_expect_in_file "$TAG" 'fn return_statement_to_json_v0' "$LOWERING" "return helper missing"
guard_expect_in_file "$TAG" 'fn if_statement_to_json_v0' "$LOWERING" "if helper missing"
guard_expect_in_file "$TAG" 'fn loop_statement_to_json_v0' "$LOWERING" "loop helper missing"
guard_expect_in_file "$TAG" 'fn loop_range_statement_to_json_v0' "$LOWERING" "LoopRange helper missing"
guard_expect_in_file "$TAG" 'fn throw_statement_to_json_v0' "$LOWERING" "throw helper missing"
guard_expect_in_file "$TAG" 'fn try_catch_statement_to_json_v0' "$LOWERING" "try/catch helper missing"
guard_expect_in_file "$TAG" 'fn expression_statement_to_json_v0' "$LOWERING" "expression statement helper missing"
guard_expect_in_file "$TAG" 'source_to_program_json_v0_emits_statement_family_shapes' "$TEST_FILE" "statement family fixture missing"

cargo test -q --lib source_to_program_json_v0_emits_statement_family_shapes
cargo test -q --lib source_to_program_json_v0_minimal_main
cargo test -q --lib source_to_program_json_v0_rewrites_if_some_sugar_to_local_plus_if

echo "[$TAG] ok"

