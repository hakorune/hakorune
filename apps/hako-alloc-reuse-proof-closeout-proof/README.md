# hako-alloc-reuse-proof-closeout-proof

Purpose: prove M206 reuse closeout for the hako_alloc purge/recommit loop.

This fixture composes the landed M199 decommit duplicate guard and the M205
recommit heap integration. It proves that a heap-owned page/backing can be
acquired, released, decommitted, blocked from duplicate decommit, recommitted,
selected again by the page queue, then acquired/released/decommitted and
recommitted again as a second generation.

Stop line:

- no new allocator owner
- no page-source reservation or fresh page creation beyond heap construction
- no unreserve or OS release
- no provider activation, hook install, or process allocator replacement
- no object-return allocator API parity expansion
