# mimalloc-alloc-fast-path-proof

Purpose
- Proves M167 allocation orchestration over `HakoAllocPageQueue` and
  `HakoAllocPageModel`.
- Exercises fast page selection, page-local free-list pop, deterministic
  fallback page creation, and local release accounting.

Stop line
- No OSVM page source composition.
- No local-free collection / retire.
- No TLS, atomic, remote-free, page-map, provider, hook, or process allocator
  replacement behavior.
- Production hako_alloc fields stay on `i64`; this proof does not consume the
  294x `usize` field probe.

Run

```bash
bash tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
```
