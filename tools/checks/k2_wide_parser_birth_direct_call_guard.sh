#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-parser-birth-direct-call"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-401-PARSER-BIRTH-001-DIRECT-BIRTH-NEGATIVE-FIXTURE.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-402-PARSER-BIRTH-002-DIRECT-BIRTH-DIAGNOSTIC-HINT.md"
SSOT="docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md"
LIFECYCLE_REF="docs/reference/language/lifecycle.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PARSER_POLICY="src/parser/lifecycle.rs"
PARSER_CALL="src/parser/expr/call.rs"
PARSER_CURSOR="src/parser/expr_cursor.rs"
FROM_CALL_A="src/parser/expressions.rs"
FROM_CALL_B="src/parser/expr/primary.rs"
TEST_FILE="src/tests/parser_direct_birth_call.rs"
TEST_MOD="src/tests/parser/mod.rs"
SELF_SCRIPT="tools/checks/k2_wide_parser_birth_direct_call_guard.sh"

echo "[$TAG] running PARSER-BIRTH-001 direct birth call guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$NEXT_CARD" \
  "$SSOT" \
  "$LIFECYCLE_REF" \
  "$TASKBOARD" \
  "$INDEX" \
  "$PARSER_POLICY" \
  "$PARSER_CALL" \
  "$PARSER_CURSOR" \
  "$FROM_CALL_A" \
  "$FROM_CALL_B" \
  "$TEST_FILE" \
  "$TEST_MOD" \
  "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

for path in "$CARD" "$SSOT" "$LIFECYCLE_REF"; do
  guard_expect_in_file "$TAG" '[Dd]irect source' "$path" "$path must mention direct source birth calls"
  guard_expect_in_file "$TAG" 'constructor hook' "$path" "$path must define birth as a constructor hook"
done

guard_expect_in_file "$TAG" 'PARSER-BIRTH-001' "$TASKBOARD" "taskboard must track PARSER-BIRTH-001"
guard_expect_in_file "$TAG" 'PARSER-BIRTH-002' "$TASKBOARD" "taskboard must keep the diagnostic follow-up visible"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list the parser birth direct-call guard"

guard_expect_in_file "$TAG" 'DIRECT_BIRTH_CALL_EXPECTED' "$PARSER_POLICY" "parser lifecycle policy must own the direct birth call diagnostic contract"
guard_expect_in_file "$TAG" 'direct source birth calls are forbidden' "$PARSER_POLICY" "parser lifecycle policy must forbid direct birth calls"
guard_expect_in_file "$TAG" 'direct_birth_call_error' "$PARSER_CALL" "legacy expression parser must use the lifecycle rejection helper"
guard_expect_in_file "$TAG" 'TokenType::BIRTH' "$PARSER_CALL" "legacy expression parser must inspect birth after receiver dot"
guard_expect_in_file "$TAG" 'direct_birth_call_error' "$PARSER_CURSOR" "TokenCursor expression parser must use the lifecycle rejection helper"
guard_expect_in_file "$TAG" 'TokenType::BIRTH' "$PARSER_CURSOR" "TokenCursor expression parser must inspect birth after receiver dot"

guard_expect_in_file "$TAG" 'parser_birth_rejects_direct_receiver_birth_call_legacy_expr_parser' "$TEST_FILE" "legacy parser negative fixture missing"
guard_expect_in_file "$TAG" 'parser_birth_rejects_direct_receiver_birth_call_token_cursor' "$TEST_FILE" "TokenCursor parser negative fixture missing"
guard_expect_in_file "$TAG" 'parser_birth_accepts_constructor_declaration' "$TEST_FILE" "constructor declaration fixture missing"
guard_expect_in_file "$TAG" 'parser_birth_keeps_parent_constructor_delegation' "$TEST_FILE" "parent constructor delegation fixture missing"
guard_expect_in_file "$TAG" 'parser_direct_birth_call' "$TEST_MOD" "parser test module must include direct birth fixture"

guard_expect_in_file "$TAG" 'TokenType::BIRTH' "$FROM_CALL_A" "from-call parser must keep parent birth delegation support"
guard_expect_in_file "$TAG" 'TokenType::BIRTH' "$FROM_CALL_B" "primary parser must keep parent birth delegation support"

cargo test -q --lib parser_birth_

echo "[$TAG] ok"
