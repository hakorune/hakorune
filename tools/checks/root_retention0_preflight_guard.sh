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

# The retention row is disconnected: only focused fixtures may construct the
# input until later rows open production callers.  Every current construction
# must occur after a cfg(test) module boundary.
test "$(rg -c 'RawRootCompletionInputV1::new' "$preflight")" -eq 1
test "$(rg -c 'prepared_commit_publishes_one_root_pair' "$preflight")" -eq 1
for fixture in src/mir/builder/raw_root_completion.rs src/mir/builder/raw_physical_finalization.rs; do
  awk '
    /#\[cfg\(test\)\]/ { in_test = 1 }
    /RawRootCompletionInputV1::new/ && !in_test { bad = 1 }
    END { exit bad }
  ' "$fixture"
done

raw_physical="src/mir/builder/raw_physical_finalization.rs"
test "$(rg -c '^pub\(in crate::mir\) struct RawPhysicalCompleteInvocationV1' "$raw_physical")" -eq 1
test "$(rg -c '^    token: ModuleInvocationTokenV1,' "$raw_physical")" -eq 1
if awk '
  /fn bind_physical\(/,/\) -> Result/ { if ($0 ~ /token:/) bad = 1 }
  END { exit bad }
' "$raw_physical"; then :; else
  echo "bind_physical must consume the complete product token" >&2
  exit 1
fi
test "$(rg -c 'fn token\(&self\) -> &ModuleInvocationTokenV1' src/mir/builder/raw_root_completion.rs)" -eq 1
test "$(rg -c 'session\(complete: &RawCompleteInvocationV1\)' "$raw_physical")" -eq 1
if awk '
  /fn complete_raw_root\(/,/\) -> Result/ { if ($0 ~ /token:/) bad = 1 }
  END { exit bad }
' src/mir/builder/raw_root_completion.rs; then :; else
  echo "complete_raw_root must consume the branded input owner" >&2
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

echo "ROOT-RETENTION0-PREFLIGHT/TOKEN-HANDOFF guard: PASS"
