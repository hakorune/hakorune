# mimalloc-lite

Allocator-shaped real app for the phase-293x lane.

This is now the VM-only `hako_alloc` page/free-list policy-state port slice.
It is not native allocator backend migration or EXE parity. The app keeps a
small deterministic contract with:

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

Expected tail:

```text
summary=ok
```
