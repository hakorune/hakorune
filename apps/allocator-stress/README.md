# allocator-stress

Real-app stress slice for the `hako_alloc` page/free-list policy seam.

The original allocator row was VM-only, but current EXE parity is owned by the
shared real-app EXE boundary suite. This app does not make VM a new allocator
feature owner.

This app checks:

- small and medium page saturation
- free-list reuse after releases
- oversize allocation rejection
- double-free rejection
- deterministic accounting across the public `hako_alloc` seam

## Run

```bash
./target/release/hakorune --backend vm apps/allocator-stress/main.hako
```

Expected tail:

```text
summary=ok
```
