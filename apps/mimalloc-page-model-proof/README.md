# mimalloc-page-model-proof

Purpose
- Proves the M165 page-local state owner without heap queues or OS memory.
- Exercises `free`, `local_free`, `used`, `capacity`, and `reserved`
  invariants through `HakoAllocPageModel`.

Stop line
- No page queues.
- No OSVM page source calls.
- No TLS or atomic routes.
- No remote-free integration.
- No allocator replacement or hook activation.

Run

```bash
apps/mimalloc-page-model-proof/test.sh
```
