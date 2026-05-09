# mimalloc-lite

Allocator-shaped real app for the phase-293x lane.

This is the `hako_alloc` page/free-list policy-state port slice. The original
row was VM-only, but current real-app EXE parity uses the shared typed-object
and pure-first compiler seams. It is still not native allocator backend
migration. The app keeps a small deterministic contract with:

- fixed-size pages
- block handles
- free-list reuse
- allocation/free accounting
- peak usage tracking

The goal is to stabilize allocator vocabulary through the public `hako_alloc`
seam before the native allocator backend and typed object EXE plan land.

## Run

```bash
./target/release/hakorune --backend vm apps/mimalloc-lite/main.hako
```

The EXE parity owner is the real-app EXE boundary suite, not a second
allocator-specific VM implementation.

Expected tail:

```text
summary=ok
```
