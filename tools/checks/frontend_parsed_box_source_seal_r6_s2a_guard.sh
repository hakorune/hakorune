#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s2a"
PARSER_MOD="$ROOT/src/parser/mod.rs"
SESSION_TESTS="$ROOT/src/parser/source_session_tests.rs"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$PARSER_MOD" "$SESSION_TESTS"

python3 - "$ROOT" "$PARSER_MOD" "$SESSION_TESTS" <<'PY'
import sys
from pathlib import Path

root, parser_mod_path, session_tests_path = map(Path, sys.argv[1:])
parser_mod = parser_mod_path.read_text(encoding="utf-8")
session_tests = session_tests_path.read_text(encoding="utf-8")

for needle in (
    "source_invocation_brand",
    "next_source_statement_ordinal",
    "active_source_statement_ordinal",
    "ParserInvocationBrandV1::issue()",
    "active_source_statement_ordinal = Some(statement_ordinal)",
    "active_source_statement_ordinal = None",
):
    if needle not in parser_mod:
        raise SystemExit(f"missing R6-S2a parser session contract: {needle}")

for needle in (
    "parser_session_owns_fresh_brand_and_top_level_cursor",
    "parser_session_advances_top_level_cursor_once_per_statement",
    "box First {}",
    "box Second {}",
):
    if needle not in session_tests:
        raise SystemExit(f"missing R6-S2a session test: {needle}")

for path in (parser_mod_path, session_tests_path):
    if len(path.read_text(encoding="utf-8").splitlines()) >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path}")

print("parser_invocation_brand_owner=1")
print("top_level_statement_cursor=1")
print("session_test_coverage=1")
print("producer_transaction_connection=historical-later-rows-allowed")
print("rich_parse_output_connection=historical-later-rows-allowed")
print("resolver_connection=0")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"
