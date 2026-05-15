#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-parser-birth-diagnostic-hint"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/checks/lib/phase_card_paths.sh"

CARD="$(guard_require_phase293x_card "$TAG" "293x-402-PARSER-BIRTH-002-DIRECT-BIRTH-DIAGNOSTIC-HINT.md")"
NEXT_CARD="$(guard_require_phase293x_card "$TAG" "293x-403-REUSE-LIFECYCLE-001-EXPLICIT-REUSE-METHODS.md")"
SSOT="docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md"
LIFECYCLE_REF="docs/reference/language/lifecycle.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PARSER_POLICY="src/parser/lifecycle.rs"
PARSER_CALL="src/parser/expr/call.rs"
PARSER_CURSOR="src/parser/expr_cursor.rs"
TEST_FILE="src/tests/parser_direct_birth_call.rs"
SELF_SCRIPT="tools/checks/k2_wide_parser_birth_diagnostic_hint_guard.sh"

echo "[$TAG] running PARSER-BIRTH-002 diagnostic hint guard"

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
  "$TEST_FILE" \
  "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'PARSER-BIRTH-002' "$CARD" "diagnostic hint card must exist"
guard_expect_in_file "$TAG" 'use new Box\(\.\.\.\)' "$CARD" "card must require the new-expression hint"
guard_expect_in_file "$TAG" 'REUSE-LIFECYCLE-001' "$NEXT_CARD" "next reuse lifecycle card must exist"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list the diagnostic hint guard"

guard_expect_in_file "$TAG" 'direct source birth calls are forbidden' "$PARSER_POLICY" "policy helper must keep the direct call rejection"
guard_expect_in_file "$TAG" 'use new Box\(\.\.\.\) for construction' "$PARSER_POLICY" "policy helper must own the new-expression hint"
guard_expect_in_file "$TAG" 'direct_birth_call_error' "$PARSER_CALL" "legacy parser must keep using the lifecycle policy helper"
guard_expect_in_file "$TAG" 'direct_birth_call_error' "$PARSER_CURSOR" "TokenCursor parser must keep using the lifecycle policy helper"

guard_expect_in_file "$TAG" 'parser_birth_direct_call_diagnostic_points_to_new_expression' "$TEST_FILE" "diagnostic hint fixture missing"
guard_expect_in_file "$TAG" 'use new Box\(\.\.\.\) for construction' "$TEST_FILE" "test must assert the new-expression hint"

cargo test -q --lib parser_birth_

echo "[$TAG] ok"
