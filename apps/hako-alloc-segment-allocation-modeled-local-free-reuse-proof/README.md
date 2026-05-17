# hako-alloc-segment-allocation-modeled-local-free-reuse-proof

This proof app fixes `MIMAP-126A`.

It proves that the modeled local-free integration route can feed
`HakoAllocPageModel.acquire(size)` reuse when the ordinary page free list is
empty. The app stays page-local and does not activate real segment free,
segment-map lookup, atomics, OSVM, threads, providers, host replacement, or
backend matchers.

Run:

```bash
tools/checks/run_proof_app.sh --only MIMAP-126A
```
