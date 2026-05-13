# hako-alloc-recommit-heap-integration-proof

Status: M205 proof app.

This proof composes M200, M202, M203, and M204 into a heap-level recommit path.

It proves:

- retired page-local allocation rejects before recommit integration
- successful recommit transitions marker state
- page-local state is reactivated without unreserve or OS release
- queue selection can see the reactivated page again
- allocation can acquire from the recommitted page

Run:

```bash
bash apps/hako-alloc-recommit-heap-integration-proof/test.sh
```
