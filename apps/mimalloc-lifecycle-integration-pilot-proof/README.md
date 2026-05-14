# mimalloc lifecycle integration pilot proof

Decision: accepted for `MIMAP-009`.

This app proves the page-local lifecycle sequence on `HakoAllocPageModel`:

```text
active -> retired -> decommitted -> recommitted -> reusable active
```

The proof intentionally remains page-local. It does not call OSVM, segment sources,
provider activation, allocator hooks, or host allocator replacement.

The expected transition contract is:

- `decommit` is accepted only for retired, unused, non-decommitted pages.
- direct acquire stays rejected while the page is retired/decommitted.
- direct `reactivate` is rejected while decommitted.
- `recommit` clears the decommitted state but does not make the page active.
- `reuse` reactivates only after recommit and drains local-free blocks.
