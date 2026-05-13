# hako-alloc-heap-reuse-priority-policy-proof

Purpose: prove M208 heap reuse priority policy.

This fixture freezes the reuse ordering over the existing lifecycle vocabulary:

1. active page reuse
2. recommitted-active page reuse
3. retired page reactivation
4. fresh page fallback

Stop line:

- no allocation, release, decommit, recommit, or reactivation behavior is added
- no scheduler, purge widening, or OS release/unreserve behavior
- no verifier enforcement
- no provider hook or process allocator replacement
