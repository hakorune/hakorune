#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="guard-result-cache-dirty-untracked-memo"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" git
guard_require_command "$TAG" sha256sum

TMP_DIR="$(mktemp -d /tmp/hakorune-guard-result-cache-dirty.XXXXXX)"
COUNTER_FILE="$(mktemp /tmp/hakorune-guard-result-cache-counter.XXXXXX)"
CACHE_DIR="$(mktemp -d /tmp/hakorune-guard-result-cache-dir.XXXXXX)"
cleanup() {
  rm -rf "$TMP_DIR" >/dev/null 2>&1 || true
  rm -rf "$CACHE_DIR" >/dev/null 2>&1 || true
  rm -f "$COUNTER_FILE" >/dev/null 2>&1 || true
}
trap cleanup EXIT

git -C "$TMP_DIR" init -q

cat >"$TMP_DIR/counted.sh" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
counter="$1"
value="$(cat "$counter" 2>/dev/null || printf '0')"
value=$((value + 1))
printf '%s\n' "$value" >"$counter"
printf 'counted=%s\n' "$value"
SH
chmod +x "$TMP_DIR/counted.sh"

printf 'alpha\n' >"$TMP_DIR/untracked.txt"
printf '0\n' >"$COUNTER_FILE"

first="$(
  cd "$TMP_DIR"
  HAKO_GUARD_RESULT_CACHE_DIR="$CACHE_DIR" \
  HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 \
  guard_cached_run "$TAG" bash "$TMP_DIR/counted.sh" "$COUNTER_FILE"
)"
second="$(
  cd "$TMP_DIR"
  HAKO_GUARD_RESULT_CACHE_DIR="$CACHE_DIR" \
  HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 \
  guard_cached_run "$TAG" bash "$TMP_DIR/counted.sh" "$COUNTER_FILE"
)"

if [[ "$first" != "counted=1" || "$second" != "counted=1" ]]; then
  printf 'first=%s\nsecond=%s\n' "$first" "$second" >&2
  guard_fail "$TAG" "dirty untracked cache did not memoize identical worktree"
fi

printf 'beta\n' >"$TMP_DIR/untracked.txt"
third="$(
  cd "$TMP_DIR"
  HAKO_GUARD_RESULT_CACHE_DIR="$CACHE_DIR" \
  HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 \
  guard_cached_run "$TAG" bash "$TMP_DIR/counted.sh" "$COUNTER_FILE"
)"

if [[ "$third" != "counted=2" ]]; then
  printf 'third=%s\n' "$third" >&2
  guard_fail "$TAG" "dirty untracked cache did not invalidate after untracked content changed"
fi

cat <<'REPORT'
output_contract=guard-result-cache-dirty-untracked-memo-v0
dirty_untracked_guard_cache_memo=1
untracked_content_digest_participates=1
dirty_cache_requires_allow_dirty=1
summary=ok
REPORT
