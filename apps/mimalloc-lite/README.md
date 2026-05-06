# mimalloc-lite

Allocator-shaped real app for the phase-293x lane.

This is not the real allocator port. It is a small deterministic model with:

- fixed-size pages
- block handles
- free-list reuse
- allocation/free accounting
- peak usage tracking

The goal is to stabilize app-level allocator vocabulary before porting the real
allocator.

## Run

```bash
./target/release/hakorune --backend vm apps/mimalloc-lite/main.hako
```

Expected tail:

```text
summary=ok
```
