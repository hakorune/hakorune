# mimalloc-size-class-table-proof

M21 proof fixture for the mimalloc substrate ladder.

This app composes already-landed rows:

- M11b static `u16` const table declarations, const-expression evaluation, and
  `StaticDataLoad`.
- M14-M20 raw-page pure-first EXE routes over `RawBufCoreBox` and
  `RawArrayCoreBox`.

The size-class table is not built as a runtime `ArrayBox` or `MapBox`. It is a
MIR-owned `static_data_plans` row that the backend reads directly.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_size_class_table_exe_guard.sh
```

Expected tail:

```text
summary=ok
```
