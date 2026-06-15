#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-740-MANUAL-SYNC-CONCURRENCY-001.md"
README="README.md"
SEM="docs/reference/concurrency/semantics.md"
BOUNDARY="docs/reference/concurrency/boundary-model.md"
STAGE="docs/reference/language/stage-profiles.md"
DOCS_README="docs/README.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_manual_sync_concurrency_guard.sh"

[[ -f "$CARD" ]] || { echo "[manual-sync-concurrency] missing card: $CARD" >&2; exit 1; }
grep -q '^Status: Landed$' "$CARD" || { echo "[manual-sync-concurrency] card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[manual-sync-concurrency] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[manual-sync-concurrency] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-manual-sync-concurrency-v0" \
  "co_canonical_surface=1" \
  "task_scope_compat_surface=1" \
  "nowait_os_thread_spawn=0" \
  "readme_async_sample_uses_co=1" \
  "worker_scope_design_reserved=1" \
  "worker_scope_workers_is_upper_bound=1" \
  "worker_scope_exact_thread_count_promise=0" \
  "raw_thread_parser_enabled=0" \
  "threadapi_substrate_not_source_syntax=1" \
  "concurrency_reference_nav_linked=1" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "co {" "$README" || { echo "[manual-sync-concurrency] README async sample must use co" >&2; exit 1; }
grep -F -q "not an OS-thread spawn promise" "$README" || {
  echo "[manual-sync-concurrency] README missing nowait non-thread-promise wording" >&2
  exit 1
}
grep -F -q "Current MIRBuilder v0:" "$SEM" || { echo "[manual-sync-concurrency] semantics missing current MIRBuilder v0 heading" >&2; exit 1; }
if grep -F -q "Next MIRBuilder row:" "$SEM"; then
  echo "[manual-sync-concurrency] semantics still contains stale Next MIRBuilder row heading" >&2
  exit 1
fi
grep -F -q "nowait expr" "$BOUNDARY" || { echo "[manual-sync-concurrency] boundary model missing nowait expr wording" >&2; exit 1; }
grep -F -q "worker_scope workers=N" "$STAGE" || { echo "[manual-sync-concurrency] stage profiles missing worker_scope reserved wording" >&2; exit 1; }
grep -F -q "reference/runtime/threading.md" "$DOCS_README" || { echo "[manual-sync-concurrency] docs README missing runtime threading link" >&2; exit 1; }
grep -F -q "reference/concurrency/boundary-model.md" "$DOCS_README" || { echo "[manual-sync-concurrency] docs README missing boundary model link" >&2; exit 1; }

echo "[manual-sync-concurrency] ok"
