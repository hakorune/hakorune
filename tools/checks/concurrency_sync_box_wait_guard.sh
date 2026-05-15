#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="concurrency-sync-box-wait-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TASKBOARD="$ROOT_DIR/docs/development/current/main/design/concurrency-boundary-migration-taskboard-ssot.md"
SEMANTICS="$ROOT_DIR/docs/reference/concurrency/semantics.md"
SYNC_VALIDATOR="$ROOT_DIR/src/parser/declarations/box_def/sync_box.rs"
BOX_PARSER="$ROOT_DIR/src/parser/declarations/box_def/mod.rs"
PARSER_TEST="$ROOT_DIR/src/tests/parser_sync_box_surface.rs"

guard_require_command "$TAG" rg
guard_require_files "$TAG" \
  "$TASKBOARD" \
  "$SEMANTICS" \
  "$SYNC_VALIDATOR" \
  "$BOX_PARSER" \
  "$PARSER_TEST"

guard_expect_in_file "$TAG" "CONC-SYNCBOX-002" "$TASKBOARD" "taskboard must keep sync wait verifier row"
guard_expect_in_file "$TAG" "landed-verifier" "$TASKBOARD" "taskboard must record landed verifier state"
guard_expect_in_file "$TAG" "Sync methods reject" "$SEMANTICS" "semantics must describe sync method wait rejection"
guard_expect_in_file "$TAG" "validate_no_waits_in_sync_box" "$BOX_PARSER" "box parser must invoke sync wait validator"
guard_expect_in_file "$TAG" "sync_box/wait_forbidden" "$SYNC_VALIDATOR" "validator must emit stable fail-fast tag"
guard_expect_in_file "$TAG" "ASTNode::AwaitExpression" "$SYNC_VALIDATOR" "validator must reject await"
guard_expect_in_file "$TAG" "ASTNode::Nowait" "$SYNC_VALIDATOR" "validator must reject nowait"
guard_expect_in_file "$TAG" "sync_box_rejects_await_in_methods" "$PARSER_TEST" "tests must cover await rejection"
guard_expect_in_file "$TAG" "sync_box_rejects_nowait_in_methods" "$PARSER_TEST" "tests must cover nowait rejection"

echo "[$TAG] ok"
