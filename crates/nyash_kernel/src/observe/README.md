# observe module notes

Status: opt-in observability boundary for perf evidence.

This module records runtime evidence for perf investigations. It must stay
feature-gated and side-effect-light: production behavior must not depend on an
observe counter, sink, or trace hook.

## Module Split

- `mod.rs` owns the public in-crate recording facade. Callers should use these
  `record_*` helpers instead of reaching into backends or sinks.
- `contract.rs` is the SSOT for counter family names, field names, snapshot
  ordering, and sink/test projection helpers.
- `backend/tls.rs` owns TLS counter storage, increments, snapshots, and flushes.
- `sink/stderr.rs` owns stderr serialization only. It should read named
  projection helpers from `contract.rs`, not duplicate snapshot index knowledge.
- `config.rs` owns observe enablement and env gates.
- `trace.rs` owns `perf-trace` hooks separately from `perf-observe` counters.

## Counter Change Rule

When adding or moving a counter family:

1. Add names and projection policy in `contract.rs`.
2. Add storage and increment/flush behavior in the backend.
3. Expose only the narrow `record_*` facade needed by call sites.
4. Format through `contract.rs` projections in sinks and tests.

Do not let a sink, test, or call site become a second owner of snapshot layout.

## Logging Rule

- No unconditional observe output.
- Keep tags stable and one-line when output is enabled.
- Do not add new env gates outside `config.rs`.
- `perf-observe` counters are evidence only; they must not steer runtime
  semantics.
