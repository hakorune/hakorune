#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="concurrency-boundary-surface-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TASKBOARD="$ROOT_DIR/docs/development/current/main/design/concurrency-boundary-migration-taskboard-ssot.md"
REFERENCE="$ROOT_DIR/docs/reference/concurrency/boundary-model.md"
AST="$ROOT_DIR/src/ast/mod.rs"
PARSER="$ROOT_DIR/src/parser/statements/task_scope.rs"
PROGRAM_JSON="$ROOT_DIR/src/stage1/program_json_v0/lowering.rs"
AST_JSON_ROUNDTRIP="$ROOT_DIR/src/macro/ast_json/roundtrip.rs"
JSON_BRIDGE="$ROOT_DIR/src/runner/json_v0_bridge/lowering/stmts.rs"

guard_require_command "$TAG" rg
guard_require_files "$TAG" \
  "$TASKBOARD" \
  "$REFERENCE" \
  "$AST" \
  "$PARSER" \
  "$PROGRAM_JSON" \
  "$AST_JSON_ROUNDTRIP" \
  "$JSON_BRIDGE"

guard_expect_in_file "$TAG" "CONC-COMPAT-001" "$TASKBOARD" "taskboard must keep legacy-surface audit row"
guard_expect_in_file "$TAG" "CONC-CO-001" "$TASKBOARD" "taskboard must keep co implementation row"
guard_expect_in_file "$TAG" "landed-parser-json" "$TASKBOARD" "taskboard must record landed parser/json state"
guard_expect_in_file "$TAG" 'co \{ \.\.\. \}' "$REFERENCE" "reference must name canonical co surface"
guard_expect_in_file "$TAG" "source_keyword" "$AST" "AST must preserve canonical/compat task scope spelling"
guard_expect_in_file "$TAG" "is_task_scope_statement_start" "$PARSER" "parser must use contextual task-scope detection"
guard_expect_in_file "$TAG" '"TaskScope"' "$PROGRAM_JSON" "Program JSON must carry TaskScope rows"
guard_expect_in_file "$TAG" '"TaskScope"' "$AST_JSON_ROUNDTRIP" "AST JSON roundtrip must preserve TaskScope rows"
guard_expect_in_file "$TAG" "task_scope_lowering_missing" "$JSON_BRIDGE" "JSON bridge must fail-fast until runtime hook lowering lands"

legacy_hits="$(
  rg -n --glob '*.hako' --glob '!**/archive/**' --glob '!tools/archive/**' \
    '\btask_scope[[:space:]]*\{|\bworker_local\b|lock<T>' "$ROOT_DIR" || true
)"
if [[ -n "$legacy_hits" ]]; then
  printf '%s\n' "$legacy_hits" >&2
  guard_fail "$TAG" "active .hako source still uses legacy concurrency surface"
fi

echo "[$TAG] ok"
