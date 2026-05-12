#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-guard-else-surface"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

APP="apps/guard-else-surface-proof/main.hako"
APP_TEST="apps/guard-else-surface-proof/test.sh"
APP_README="apps/guard-else-surface-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-206-C200-GUARD-ELSE-SURFACE.md"
ORDER_CARD="docs/development/current/main/phases/phase-293x/293x-202-C197-C200-PROOF-APPLICATION-SURFACE-ORDER.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
EBNF="docs/reference/language/EBNF.md"
TOKEN_KINDS="src/tokenizer/kinds.rs"
TOKEN_IDENT="src/tokenizer/lex_ident.rs"
PARSER_CONTROL="src/parser/statements/control_flow.rs"
PARSER_MOD="src/parser/statements/mod.rs"
PARSER_TEST="src/tests/parser_guard_else_surface.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_guard_else_surface_guard.sh"

echo "[$TAG] checking C200 guard else surface"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$ORDER_CARD" \
  "$PLAN" \
  "$EBNF" \
  "$TOKEN_KINDS" \
  "$TOKEN_IDENT" \
  "$PARSER_CONTROL" \
  "$PARSER_MOD" \
  "$PARSER_TEST" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" '### C200 Guard Else Surface' "$EBNF" "EBNF must record the C200 decision"
guard_expect_in_file "$TAG" 'Decision: accepted' "$EBNF" "EBNF must mark C200 accepted"
guard_expect_in_file "$TAG" "guard_stmt := 'guard' expr 'else' block" "$EBNF" "EBNF must define guard_stmt"
guard_expect_in_file "$TAG" 'C200 guard else surface` \| Complete' "$PLAN" "plan must mark C200 complete"
guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C200 card must be complete"
guard_expect_in_file "$TAG" 'GUARD' "$TOKEN_KINDS" "tokenizer must define GUARD token"
guard_expect_in_file "$TAG" '"guard" => TokenType::GUARD' "$TOKEN_IDENT" "tokenizer must recognize guard keyword"
guard_expect_in_file "$TAG" 'TokenType::GUARD' "$PARSER_MOD" "statement parser must dispatch guard"
guard_expect_in_file "$TAG" 'parse_guard_else' "$PARSER_CONTROL" "control-flow parser must parse guard else"
guard_expect_in_file "$TAG" 'UnaryOperator::Not' "$PARSER_CONTROL" "guard must lower to negative If condition"
guard_expect_in_file "$TAG" 'guard ready == 1 else' "$APP" "proof app must cover passing guard"
guard_expect_in_file "$TAG" 'guard allowed == 1 else' "$APP" "proof app must cover early-exit guard"
guard_expect_in_file "$TAG" 'summary=ok' "$APP" "proof app must expose stable summary"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C200 guard"

if rg -n 'guard-else-surface|parse_guard_else|TokenType::GUARD|guard expr else' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: C200 syntax/proof matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

cargo test -q parser_guard_else_surface

"$APP_TEST"

echo "[$TAG] ok"
