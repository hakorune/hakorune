# Ownership SSA V1

This box verifies MIR ownership lifetimes. It does not own lexical binding
identity or reaching values.

Authority split:

- Binding SSA owns `BindingRefV1 -> ValueId` reaching definitions.
- Ownership SSA classifies each MIR `ValueId` as `None`, `Borrowed`, or
  `Owned`, and verifies consuming/forwarding dispositions.
- Runtime backends materialize only a sealed `VerifiedOwnershipSsaV1`.

Owned Phi inputs are parallel transfers on exact predecessor edges. The
verifier never counts every Phi input as one simultaneously executed consume.
Canonical V1 rejects edge arguments and unreachable blocks.

Production callers remain zero until the atomic canonical-owner cutover.
