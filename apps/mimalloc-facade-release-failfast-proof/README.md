# mimalloc facade release fail-fast proof

This proof belongs to `MIMAP-015B`.

It proves facade-level double-release and stale-page rejection using the
existing `objectLifecycleReleaseBlock(page_id, block_id)` route. The proof does
not add page-map lookup, arbitrary pointer resolution, realloc behavior, or
selected-object return.
