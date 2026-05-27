# hako-alloc-mimalloc-comparison-object-lifecycle-known-live-release-smoke

Lightweight smoke for the direct cached-page known-live release keeper.

This app keeps the same alloc-then-release shape as the larger object-lifecycle
proof but runs one 64-block cycle only. It exists so implementation guards can
fail fast without running the full repeated measurement workload.
