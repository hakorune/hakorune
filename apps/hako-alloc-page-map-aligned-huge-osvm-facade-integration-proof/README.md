# hako-alloc-page-map-aligned-huge-osvm-facade-integration-proof

Purpose: prove the aligned small-path, huge-page, and OSVM page-source seams
stay read-only behind one integration owner, and that the current pure-first
driver still fails closed on the integration shape.

Run:

```bash
bash tools/checks/k2_wide_hako_alloc_page_map_aligned_huge_osvm_facade_integration_guard.sh
```
