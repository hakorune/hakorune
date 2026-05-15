# mimalloc facade release one block proof

This proof belongs to `MIMAP-015A`.

It proves that `HakoAllocObjectLifecycleFacade` can release one known
`(page id, block id)` pair returned by the facade small allocation observers,
using the existing page-local `releaseLocal(block_id)` route without page-map
lookup or arbitrary pointer resolution.
