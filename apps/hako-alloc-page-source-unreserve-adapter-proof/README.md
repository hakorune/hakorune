# hako-alloc-page-source-unreserve-adapter-proof

Purpose: MIMAP-033A proof for the `hako_alloc` page-source unreserve adapter.

The app reserves, commits, and decommits one page through
`HakoAllocPageSourcePolicy`, then releases the reserved range through
`HakoAllocPageSourceUnreserveAdapter.unreservePage(...)` exactly once.

This proof is pure-first EXE focused because the Rust VM does not own the OSVM
leaf execution path. It intentionally keeps facade huge-unreserve behavior,
provider activation, hooks, host allocator replacement, and `#[global_allocator]`
closed.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_page_source_unreserve_adapter_guard.sh
```
