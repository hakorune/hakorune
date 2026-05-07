# allocator-stress

Real-app stress slice for the `hako_alloc` VM-only page/free-list seam.

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
