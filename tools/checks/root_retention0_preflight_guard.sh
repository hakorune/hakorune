#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

preflight="src/mir/builder/raw_root_completion_preflight.rs"
root_batch="src/mir/builder/module_draft_collector/root_batch.rs"
ledger="src/mir/builder/raw_expansion_receipt_ledger.rs"
ledger_preflight="src/mir/builder/raw_expansion_receipt_ledger/preflight.rs"

test -f "$preflight"
test "$(rg -c '^pub\(in crate::mir::builder\) struct RawRootCompletionInputV1' "$preflight")" -eq 1
test "$(rg -c '^pub\(in crate::mir::builder\) struct RejectedRawRootCompletionV1' "$preflight")" -eq 1
test "$(rg -c 'fn validate_root_batch' "$root_batch")" -eq 1
test "$(rg -c 'fn validate_required_root_batch' "$ledger_preflight")" -eq 1
test "$(rg -c 'fn commit\(self\) -> RawCompleteInvocationV1' "$preflight")" -eq 1

# The retention row is disconnected: only its focused fixture may construct
# the input until the later COMMIT/TOKEN-HANDOFF rows open production callers.
test "$(rg -c 'RawRootCompletionInputV1::new' "$preflight")" -eq 1
test "$(rg -c 'prepared_commit_publishes_one_root_pair' "$preflight")" -eq 1
if rg -n 'RawRootCompletionInputV1::new' src/mir/builder/raw_root_completion.rs src/mir/builder/raw_physical_finalization.rs; then
  echo "unexpected production RawRootCompletionInputV1 consumer" >&2
  exit 1
fi
if rg -n 'crate::mir::compiler' "$preflight" "$root_batch" "$ledger_preflight"; then
  echo "retention preflight must not import compiler authority" >&2
  exit 1
fi

for file in "$preflight" "$root_batch" "$ledger" "$ledger_preflight"; do
  lines="$(wc -l < "$file")"
  test "$lines" -lt 800
done

echo "ROOT-RETENTION0-PREFLIGHT guard: PASS"
