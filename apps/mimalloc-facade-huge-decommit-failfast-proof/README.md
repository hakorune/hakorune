# mimalloc-facade-huge-decommit-failfast-proof

MIMAP-030A proof fixture for facade huge decommit fail-fast diagnostics.

The app proves one scalar sequence:

```text
MIMAP-029A successful same-backed unregister + decommit
  -> allocator-side decommit state records the backing range
  -> duplicate/stale decommit attempts reject before another adapter call
```

Stop lines:

- no OSVM unreserve, release, or recommit
- no provider hooks or host allocator replacement
- no backend matcher shortcut
- no reliance on page-source duplicate rejection

Run:

```bash
bash tools/checks/k2_wide_mimalloc_facade_huge_decommit_failfast_exe_guard.sh
```
