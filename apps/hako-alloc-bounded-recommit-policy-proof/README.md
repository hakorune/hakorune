# hako-alloc-bounded-recommit-policy-proof

Status: M202 proof app.

This proof fixes the bounded recommit execution policy before the allocator
opens a real page-source recommit adapter.

It proves:

- committed/unmarked pages are blocked as "no recommit needed"
- marked decommitted pages can execute one caller-provided recommit source call
- missing pages, oversized recommits, and source rejects are blocked reports
- marker clearing, unreserve, OS release, and heap direct decommit counters stay
  closed

Run:

```bash
bash apps/hako-alloc-bounded-recommit-policy-proof/test.sh
```
