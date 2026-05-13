# hako-alloc-purge-execution-failfast-proof

Purpose: M194 proof for the `hako_alloc` purge/decommit execution fail-fast
entry.

The app creates missing, ineligible, and eligible purge decisions, then proves
`HakoAllocPurgeExecutionFailFastEntry.attempt(...)` returns blocked reports
without executing decommit, unreserve, or OS release.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_purge_execution_failfast_guard.sh
```
