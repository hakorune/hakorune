# mimalloc facade lifecycle route proof

Decision: accepted for `MIMAP-011`.

This app proves the allocator facade route to the lifecycle-aware page selection
policy. It intentionally uses scalar lifecycle observations at the facade boundary
and keeps page-object retention out of this row.

Acceptance backend: LLVM/EXE primary.

VM is allowed to keep the smaller scalar selector smoke; VM object-heavy page queue
or facade retention is not a completion requirement for this row.
