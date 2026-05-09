# mimalloc-osvm-page-proof

M25 proof app for the allocator capability ladder.

This fixture keeps allocator policy out of the app and proves the narrow
`hako.osvm` reserve/commit/decommit substrate seam in pure-first EXE:

- `OsVmCoreBox.reserve_bytes_i64(len)` returns a native pointer encoded as i64.
- `OsVmCoreBox.commit_bytes_i64(base, len)` returns scalar rc.
- `OsVmCoreBox.decommit_bytes_i64(base, len)` returns scalar rc.

The app intentionally prints only stable facts (`reserved=1`, `rc=0`) and never
prints the native address returned by the OS.

Guard:

```bash
bash tools/checks/k2_wide_mimalloc_osvm_page_exe_guard.sh
```
