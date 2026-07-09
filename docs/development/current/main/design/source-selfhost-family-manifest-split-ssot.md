# Source Selfhost Family Manifest Split

The Source Selfhost family guard uses two canonical projections:

- `source-selfhost-family-guard-active-v1.json` is the small current index.
- `source-selfhost-family-guard-history-v1.jsonl` is the append-only traceability ledger.

The old `source-selfhost-family-guard-manifest-v0.json` is a frozen compatibility
snapshot. New rows must not be added there. The split generator and family guard
prove that active plus history is an exact, disjoint partition of that snapshot.
The split provenance stores the initial v0 hash and fails if the snapshot is
edited after migration.

Active rows are limited to `current_semantic` and `current_maintenance`. Queue
and historical rows belong in history. `CURRENT_STATE.toml` remains the sole
owner of the live blocker and latest-card pointer.
