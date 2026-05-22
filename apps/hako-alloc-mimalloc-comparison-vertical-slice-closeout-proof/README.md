# hako-alloc mimalloc comparison vertical slice closeout proof

Row: `294x-59`

This proof closes the comparison-quality vertical slice by aligning the stable
`.hako` hako_alloc slice schema with the existing C mimalloc runner/report
evidence surface.

It does not execute C mimalloc, generate a provider package, install hooks, use
worker/TLS behavior, stress remote-free behavior, or claim native allocator
replacement.

Run:

```bash
bash apps/hako-alloc-mimalloc-comparison-vertical-slice-closeout-proof/test.sh
```
