# lang/src/runtime/substrate/worker

MIMAP-WORKER-001 worker identity substrate.

Scope:

- Expose the narrow allocator-internal current-worker id helper.
- Keep the proof single-worker and deterministic: worker id is `0`.
- Keep the source surface closed; this is not `worker_local` syntax or public
  worker identity semantics.

Non-goals:

- No TLS/cache-slot storage.
- No atomics, remote-free, abandoned-owner policy, or task scheduling.
- No true thread pool semantics.
