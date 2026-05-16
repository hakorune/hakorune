# mimalloc-facade-huge-decommit-proof

MIMAP-029A proof fixture for the facade huge decommit-after-unregister success
route.

The app proves one scalar sequence:

```text
page-source-backed huge allocation
  -> M181 unregister on the same huge model
  -> M196 page-source decommit of the same backing range
```

Stop lines:

- no duplicate/stale decommit diagnostics
- no unreserve or recommit
- no provider hooks or host allocator replacement
- no backend matcher shortcut

Run:

```bash
bash tools/checks/k2_wide_mimalloc_facade_huge_decommit_exe_guard.sh
```
