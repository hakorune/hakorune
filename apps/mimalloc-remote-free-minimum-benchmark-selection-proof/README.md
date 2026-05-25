# mimalloc remote-free minimum benchmark selection proof

Decision: accepted for `295x-244`.

This app closes the remote-free semantic proof into an implementation-first
benchmark selection surface. It does not measure time yet. Instead, it proves
that the selected minimum pack is executable through stable `.hako` workload
shapes:

- `local-alloc-free-cycle-v0`
- `remote-free-publish-only-v0`
- `remote-free-collect-only-v0`
- `remote-free-publish-collect-cycle-v0`

The fixed selection contract is:

- `benchmark_pack=remote-free-minimum-v0`
- `backend_scope=exact-exe-first`
- `policy=warmup:1,samples:5,summary:min,median,max`
- `stop_line=provider:0,replacement:0,winner:0`

Guard reason:

```text
This catches a changed .hako contract that the existing remote-free evidence guard cannot observe.
```

The proof stays on the semantic/selection side of the lane. It does not add DLL
packaging, provider activation, replacement, hooks, repeated medians, or timing
claims.
