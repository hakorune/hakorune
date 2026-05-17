# Hako Alloc Segment Allocation Modeled Ledger Release Span Facts Proof

This proof app fixes `MIMAP-104A`.

It proves that a successful modeled ledger release reports the scalar block span
that was originally allocated: block start, request block count, block end,
allocation-time page usage, and remaining blocks. The route stays ledger-only
and does not execute real segment free, mutate a free-list, use raw pointers,
segment maps, atomics, OSVM, threads, provider activation, host allocator
replacement, or backend matchers.
