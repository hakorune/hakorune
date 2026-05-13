# hako-alloc-page-source-decommit-adapter-proof

Purpose: M196 proof for the `hako_alloc` page-source decommit adapter.

The app reserves and commits a page through the existing production facade,
then lets `HakoAllocBoundedDecommitPolicy` call
`HakoAllocPageSourceDecommitAdapter.decommitPage(...)` exactly once.

This proof is pure-first EXE focused because the Rust VM does not own the OSVM
leaf execution path. It intentionally keeps unreserve and OS release closed.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_page_source_decommit_adapter_guard.sh
```
