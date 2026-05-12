# mimalloc-page-map-proof

M171 proof app. It fixes the first pointer-to-page ownership model after M170:
`HakoAllocPageMap` records caller-visible block pointer identity and resolves it
to `page_id` / `block_id`.

This is still a model row. It does not implement arbitrary free, realloc,
pointer arithmetic, OSVM release, provider activation, hooks, or process
allocator replacement.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_page_map_guard.sh
```
