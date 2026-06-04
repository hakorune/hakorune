# mimalloc-page-model-proof

Purpose
- Proves the M165 page-local state owner without heap queues or OS memory.
- Exercises `free`, `local_free`, `used`, `capacity`, and `reserved`
  invariants through `HakoAllocPageModel`.
- After M169, this proof also accepts the current page-local behavior where
  `acquire(...)` can reuse one same-thread `local_free` block when the normal
  free stack is empty.
- The wide guard also runs the pure-first EXE front through
  `tools/allocator/mimalloc_direct_exact_env.sh`, because these page-local
  stacks are `DirectArrayI64` fields and the LLVM parity proof must exercise
  the direct array backend, not the public ArrayBox compatibility front.

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

LLVM parity guard:

```bash
bash tools/checks/k2_wide_mimalloc_page_model_guard.sh
```
