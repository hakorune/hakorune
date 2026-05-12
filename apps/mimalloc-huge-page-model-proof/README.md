# mimalloc-huge-page-model-proof

M180 proof app. It freezes the one-allocation huge page model without adding
huge release, OS release, secure-list hardening, or native allocator hooks.

The proof keeps four boundaries explicit:

1. huge pages get page ids outside the small page index range;
2. huge handles are published through `HakoAllocPageMap`;
3. requested/committed sizes live in the huge page model, not page-local free
   lists;
4. invalid sizes and under-committed requests reject before registration.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_huge_page_model_guard.sh
```
