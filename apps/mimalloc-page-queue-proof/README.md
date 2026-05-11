# mimalloc-page-queue-proof

Purpose
- Proves the M166 page queue/direct-page cache owner without integrating the
  allocator fast path.
- Exercises page selection through `HakoAllocPageQueue` while leaving block
  popping to the page model and later rows.

Stop line
- `page_queue_box.hako` must not call `acquire`.
- No OSVM page source calls.
- No TLS or atomic routes.
- No remote-free integration.
- No page-map or process allocator replacement.

Run

```bash
apps/mimalloc-page-queue-proof/test.sh
```
