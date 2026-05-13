# hako-alloc-page-source-recommit-adapter-proof

Status: M203 proof app.

This proof wires M202 bounded recommit policy to a recommit-only page-source
adapter.

It proves:

- the adapter delegates only `commitPage(base, bytes)`
- a decommitted page decision can recommit through the adapter
- marker state remains marked until a future marker transition row
- heap direct decommit counters, unreserve, and OS release stay closed

Run:

```bash
bash apps/hako-alloc-page-source-recommit-adapter-proof/test.sh
```
