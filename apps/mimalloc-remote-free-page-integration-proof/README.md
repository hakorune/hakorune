# mimalloc-remote-free-page-integration-proof

M170 proof app. It composes `HakoAllocRemoteFreePolicy` bounded pointer CAS
publish with `HakoAllocPageModel.releaseLocal(...)` page-owned state by way of
`HakoAllocRemoteFreePageInbox`.

This proof deliberately uses caller-provided `block_id` values. It does not add
page-map lookup, arbitrary pointer free, pointer `fetch_add`, OSVM release,
provider activation, hooks, or process allocator replacement.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_page_integration_guard.sh
```
