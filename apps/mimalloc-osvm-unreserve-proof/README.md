# mimalloc-osvm-unreserve-proof

MIMAP-032A proof app for the allocator capability ladder.

This fixture keeps allocator policy out of the app and proves the narrow
`hako.osvm` unreserve substrate seam in pure-first EXE:

- `OsVmCoreBox.reserve_bytes_i64(len)` returns a native pointer encoded as i64.
- `OsVmCoreBox.commit_bytes_i64(base, len)` returns scalar rc.
- `OsVmCoreBox.decommit_bytes_i64(base, len)` returns scalar rc.
- `OsVmCoreBox.unreserve_bytes_i64(base, len)` releases the reserved backing
  range and returns scalar rc.

The app intentionally prints only stable facts (`reserved=1`, `rc=0`) and never
prints the native address returned by the OS.

Guard:

```bash
bash tools/checks/k2_wide_mimalloc_osvm_unreserve_exe_guard.sh
```
