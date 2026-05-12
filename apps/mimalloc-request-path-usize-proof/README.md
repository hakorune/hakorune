# mimalloc-request-path-usize-proof

Status: Active
Scope: M188 exact `usize` request-path facades.

This proof app validates typed `usize` inputs across the request path without
migrating stored page, handle, pointer, or failure-sentinel fields.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_request_path_usize_guard.sh
```
