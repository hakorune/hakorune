#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="guard-result-cache-helper-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

COMMON="$ROOT_DIR/tools/checks/lib/guard_common.sh"
ENV_DOC="$ROOT_DIR/docs/reference/environment-variables.md"

guard_require_files "$TAG" "$COMMON" "$ENV_DOC"
guard_require_command "$TAG" mktemp

TMP_DIR="$(mktemp -d /tmp/guard-result-cache-helper.XXXXXX)"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

PROBE="$TMP_DIR/probe.sh"
COUNT="$TMP_DIR/count.txt"
CACHE_DIR="$TMP_DIR/cache"

cat >"$PROBE" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
count_file="$1"
count=0
if [[ -f "$count_file" ]]; then
  count="$(cat "$count_file")"
fi
count=$((count + 1))
printf '%s\n' "$count" >"$count_file"
printf 'probe_count=%s\nsummary=ok\n' "$count"
SH
chmod +x "$PROBE"

OUT1="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 HAKO_GUARD_RESULT_CACHE_DIR="$CACHE_DIR" guard_cached_run "$TAG" "$PROBE" "$COUNT")"
OUT2="$(HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY=1 HAKO_GUARD_RESULT_CACHE_DIR="$CACHE_DIR" guard_cached_run "$TAG" "$PROBE" "$COUNT")"

if ! grep -q '^probe_count=1$' <<<"$OUT1"; then
  printf '%s\n' "$OUT1" >&2
  guard_fail "$TAG" "first probe did not execute once"
fi
if ! grep -q '^probe_count=1$' <<<"$OUT2"; then
  printf '%s\n' "$OUT2" >&2
  guard_fail "$TAG" "second probe did not reuse cached output"
fi
if [[ "$(cat "$COUNT")" != "1" ]]; then
  guard_fail "$TAG" "cached command executed more than once"
fi
if [[ "$(find "$CACHE_DIR" -name '*.out' -type f | wc -l | tr -d ' ')" != "1" ]]; then
  guard_fail "$TAG" "expected one cached output"
fi
if ! grep -q 'HAKO_GUARD_RESULT_CACHE' "$ENV_DOC"; then
  guard_fail "$TAG" "guard cache env knobs missing from environment reference"
fi

cat <<'REPORT'
output_contract=guard-result-cache-helper-guard-v0
cache_status=miss_then_hit
cached_command_executed_once=1
env_reference_documented=1
summary=ok
REPORT
