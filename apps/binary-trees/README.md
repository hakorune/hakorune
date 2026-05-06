# Binary Trees

Small CLBG-style binary tree app for the real-app lane.

This slice intentionally uses a fast VM-sized depth:

- build a stretch tree
- keep one long-lived tree
- build many short-lived trees at two depths
- compute recursive item checks

The app is a correctness and allocation-shape smoke first. Larger benchmark
depths can be added after this route is stable.

## Run

```bash
./target/release/hakorune --backend vm apps/binary-trees/main.hako
```

Expected tail:

```text
summary=ok
```
