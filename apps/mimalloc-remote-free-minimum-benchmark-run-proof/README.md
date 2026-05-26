# mimalloc remote-free minimum benchmark run proof

Decision: accepted for `295x-245`.

These proof apps execute the remote-free minimum benchmark workloads through an
exact-EXE-first process-repeat contract. Each workload has a dedicated no-arg
`.hako` entrypoint so the row does not depend on EXE argv forwarding. The apps
themselves do not measure time internally. Instead, each app performs:

- `operation_repeat=128`
- `timing_repeat_kind=process-invocation-v0`

for exactly one fixed workload id per process invocation:

- `main.hako` → `local-alloc-free-cycle-v0` (allocate → realloc → free)
- `remote_free_publish_only.hako` → `remote-free-publish-only-v0`
- `remote_free_collect_only.hako` → `remote-free-collect-only-v0`
- `remote_free_publish_collect_cycle.hako` → `remote-free-publish-collect-cycle-v0`

The fixed run contract is:

- `output_contract=mimalloc-comparison-remote-free-minimum-benchmark-run-v0`
- `benchmark_pack=remote-free-minimum-v0`
- `backend_scope=exact-exe-first`
- `backend_split_scope=remote-free-minimum-v0`
- `backend_split_family=split-observation`
- `warmup_count=1`
- `sample_count=5`
- `stop_line=provider:0,replacement:0,winner:0`

Guard reason:

```text
This catches a changed .hako contract that the benchmark selection proof cannot observe.
```

The external guard measures process elapsed time across warmup and sample
invocations. This row does not open backend split, native C comparison,
provider/DLL packaging, replacement, hooks, or winner claims.
