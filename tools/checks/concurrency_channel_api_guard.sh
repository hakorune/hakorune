#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="concurrency-channel-api-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

REFERENCE="$ROOT_DIR/docs/reference/concurrency/boundary-model.md"
SEMANTICS="$ROOT_DIR/docs/reference/concurrency/semantics.md"
TASKBOARD="$ROOT_DIR/docs/development/current/main/design/concurrency-boundary-migration-taskboard-ssot.md"

guard_require_command "$TAG" rg
guard_require_files "$TAG" "$REFERENCE" "$SEMANTICS" "$TASKBOARD"

for doc in "$REFERENCE" "$SEMANTICS"; do
  guard_expect_in_file "$TAG" 'await[[:space:]]+[A-Za-z_][A-Za-z0-9_]*\.send' "$doc" "$(basename "$doc") must show await-visible send"
  guard_expect_in_file "$TAG" 'await[[:space:]]+[A-Za-z_][A-Za-z0-9_]*\.recv' "$doc" "$(basename "$doc") must show await-visible recv"
  guard_expect_in_file "$TAG" 'await[[:space:]]+[A-Za-z_][A-Za-z0-9_]*\.close' "$doc" "$(basename "$doc") must show await-visible close"
  guard_expect_in_file "$TAG" 'try_send' "$doc" "$(basename "$doc") must show non-blocking try_send"
  guard_expect_in_file "$TAG" 'try_recv' "$doc" "$(basename "$doc") must show non-blocking try_recv"
done

guard_expect_in_file "$TAG" "landed-api-docs" "$TASKBOARD" "taskboard must record CONC-CHANNEL-001 API-docs landing"
guard_expect_in_file "$TAG" "src/core/channel_box.rs" "$SEMANTICS" "semantics doc must quarantine legacy P2P ChannelBox wording"
guard_expect_in_file "$TAG" "src/lib.rs" "$SEMANTICS" "semantics doc must note legacy ChannelBox export"
guard_expect_in_file "$TAG" "src/boxes/box_trait.rs" "$SEMANTICS" "semantics doc must note legacy builtin ChannelBox name"

if hits="$(rg -n 'await[[:space:]]+receive|try_receive|receive_timeout|send/receive|ChannelBox<T>|TypedChannelBox' "$REFERENCE" "$SEMANTICS" "$TASKBOARD" 2>/dev/null)"; then
  printf '%s\n' "$hits" >&2
  guard_fail "$TAG" "stale channel API spelling found"
fi

echo "[$TAG] ok"
