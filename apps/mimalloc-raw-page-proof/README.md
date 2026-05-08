# mimalloc-raw-page-proof

M12 proof fixture for the mimalloc substrate ladder.

This app keeps allocator policy out of `hako.mem` / `RawBuf` / `RawArray`:

- `RawBufCoreBox` owns raw page byte allocation/free.
- `RawArrayCoreBox` owns the explicit free-list slot operations.
- `MiRawPageProof` owns only the page/free-list policy state.

The fast-path methods are annotated with `Contract(no_alloc)` and
`Contract(no_safepoint)` so MIR verification catches accidental allocation or
explicit safepoints before allocator fast-path backend work starts.

Run:

```bash
apps/mimalloc-raw-page-proof/test.sh
```
