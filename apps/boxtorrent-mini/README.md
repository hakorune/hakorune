# BoxTorrent Mini

Minimal content-addressed Box store for the real-app lane.

This first slice is intentionally local-only:

- split a payload into fixed-size chunks
- address each chunk by deterministic content digest
- reuse duplicate chunks through the store
- keep explicit refcounts for cache lifecycle
- allocate chunk storage through the `hako_alloc` page/free-list seam
- materialize the payload back from a manifest

Transport/P2P, leases, and peer exchange are later seams. The owner model is
already fixed around `BoxTorrentStore`, `ContentChunk`, and `BoxTorrentManifest`.

## Run

```bash
./target/release/hakorune --backend vm apps/boxtorrent-mini/main.hako
```

Expected tail:

```text
summary=ok
```
