# hako-alloc-purge-heap-decommit-proof

Purpose: M197 proof for purge decommit heap integration.

The app creates an OSVM-backed heap page, observes a live page rejection, then
releases the page-local block and lets `HakoAllocPurgeHeapDecommitIntegration`
decommit the eligible empty/retired page through the M196 page-source adapter.

This proof is pure-first EXE focused because it uses the OSVM leaf execution
path. It intentionally keeps heap/page mutation outside the integration owner
and keeps unreserve / OS release closed.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_purge_heap_decommit_guard.sh
```
