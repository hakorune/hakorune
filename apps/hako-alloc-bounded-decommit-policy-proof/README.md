# hako-alloc-bounded-decommit-policy-proof

Purpose: M195 proof for the `hako_alloc` bounded decommit execution policy.

The app supplies a fake page-source executor and verifies that
`HakoAllocBoundedDecommitPolicy.attemptDecommit(...)`:

- rejects missing / ineligible decisions
- rejects invalid base and oversized byte requests
- attempts at most one bounded decommit per eligible call
- reports source failure without opening unreserve or OS release

It intentionally avoids direct OSVM/page-source calls and process allocator
replacement.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_bounded_decommit_policy_guard.sh
```
