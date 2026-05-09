# mimalloc-two-class-page-proof

M22 proof fixture for the mimalloc substrate ladder.

This app composes already-landed rows:

- M11b/M21 static `u16` const table declarations, const-expression evaluation,
  `StaticDataLoad`, and pure-first static-data lowering.
- M14-M20 raw-page pure-first EXE routes over `RawBufCoreBox` and
  `RawArrayCoreBox`.

The fixture builds two raw pages from MIR-owned static size-class metadata:

- small class: 32-byte blocks, capacity 4
- medium class: 64-byte blocks, capacity 2

It exercises full-page rejects, oversize rejects, release, and reuse for both
classes without runtime `ArrayBox` / `MapBox` table materialization.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_two_class_page_exe_guard.sh
```

Expected tail:

```text
summary=ok
```
