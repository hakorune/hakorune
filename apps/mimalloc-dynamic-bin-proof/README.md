# mimalloc-dynamic-bin-proof

M23 proof fixture for the mimalloc substrate ladder.

This app proves that `StaticDataLoad` works when the table index is chosen at
runtime by a simple size-class decision:

- request `48`
- first class size `32`
- selected class index `1`
- selected page shape `64/2`

The fixture then runs a tiny raw-page allocation/release/reuse sequence using
the selected class. It does not add a dynamic allocator policy; it only locks
the MIR/backend seam for non-constant static table indices.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_dynamic_bin_exe_guard.sh
```

Expected tail:

```text
summary=ok
```
