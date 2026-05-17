# hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-applied-recycle-proof

Proof app for `MIMAP-142A`.

It proves that a local-free reuse ledger row can be release-applied, read back
as non-live, and then recorded again as a new live row with the same modeled
reuse token. A still-live duplicate remains rejected.
