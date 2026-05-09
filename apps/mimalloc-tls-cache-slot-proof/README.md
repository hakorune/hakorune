# mimalloc-tls-cache-slot-proof

Purpose: M26 pure-first EXE proof for the narrow `hako.tls` cache-slot get/set
seam.

The source uses `TlsCoreBox.cache_slot_get_i64/1` and
`TlsCoreBox.cache_slot_set_i64/2`. MIR owns the route facts for
`hako_tls_cache_slot_get_i64` and `hako_tls_cache_slot_set_i64`; pure-first
only reads those facts.

This fixture intentionally avoids:

- generic TLS cells
- allocator TLS policy
- atomics / remote-free
- native pointer attrs

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_tls_cache_slot_exe_guard.sh
```
