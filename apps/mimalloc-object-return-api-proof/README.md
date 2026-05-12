# mimalloc-object-return-api-proof

Status: Active
Scope: M189 object-return allocate/realloc EXE parity.

This proof app exercises the semantic `HakoAllocHeap` object-return API:

- `allocate(size) -> HakoAllocHandle`
- `realloc(handle, size) -> HakoAllocHandle`
- `release(handle)` cleanup side effect

The proof checks the VM and pure-first EXE proof lines for the same observable
handle fields, release side effect, and final heap state. Harness noise such as
plugin warnings or process result trailers is not part of the proof contract. It
does not add nullable result wrappers; M190 owns that shape.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_object_return_api_guard.sh
```
