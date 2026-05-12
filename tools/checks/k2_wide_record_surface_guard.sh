#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-record-surface"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-208-C202-RECORD-SURFACE-SEMANTICS.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
EBNF="docs/reference/language/EBNF.md"
TOKEN_KINDS="src/tokenizer/kinds.rs"
TOKEN_IDENT="src/tokenizer/lex_ident.rs"
PARSER="src/parser/declarations/box_def/mod.rs"
PARSER_TEST="src/tests/parser_record_surface.rs"
DECL_INDEXER="src/mir/builder/declaration_indexer.rs"
FACTORY="src/runner/modes/common_util/user_box_factory.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_record_surface_guard.sh"

echo "[$TAG] checking C202 record surface"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$EBNF" \
  "$TOKEN_KINDS" \
  "$TOKEN_IDENT" \
  "$PARSER" \
  "$PARSER_TEST" \
  "$DECL_INDEXER" \
  "$FACTORY" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" '### C202 Record Surface And Semantics' "$EBNF" "EBNF must record C202 decision"
guard_expect_in_file "$TAG" 'Decision: accepted' "$EBNF" "EBNF must mark C202 accepted"
guard_expect_in_file "$TAG" "record_decl := 'record' IDENT" "$EBNF" "EBNF must define record_decl"
guard_expect_in_file "$TAG" 'C202 status:' "$PLAN" "mimalloc plan must record C202 status"
guard_expect_in_file "$TAG" '`C202` is complete as `293x-208`' "$RECORD_SSOT" "record SSOT must mark C202 complete"
guard_expect_in_file "$TAG" 'RECORD' "$TOKEN_KINDS" "tokenizer must define RECORD token"
guard_expect_in_file "$TAG" '"record" => TokenType::RECORD' "$TOKEN_IDENT" "tokenizer must recognize record keyword"
guard_expect_in_file "$TAG" 'parse_record_declaration' "$PARSER" "parser must expose record declaration parser"
guard_expect_in_file "$TAG" 'is_record: true' "$PARSER" "record AST must be marked distinct from ordinary box"
guard_expect_in_file "$TAG" 'if \*is_record' "$DECL_INDEXER" "MIR declaration indexer must not treat record as ordinary box"
guard_expect_in_file "$TAG" 'if \*is_record' "$FACTORY" "runtime factory must not treat record as ordinary box"
guard_expect_in_file "$TAG" 'c202_record_declaration_parses_typed_fields' "$PARSER_TEST" "parser test must cover accepted records"
guard_expect_in_file "$TAG" 'c202_record_rejects_weak_untyped_and_method_bodies' "$PARSER_TEST" "parser test must cover rejected records"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C202 guard"

if rg -n 'record-surface|parse_record_declaration|TokenType::RECORD|is_record' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: C202 record matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

cargo test -q parser_record_surface

echo "[$TAG] ok"
