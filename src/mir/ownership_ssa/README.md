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

Call policy is deliberately narrower than generic operand projection. Legacy
`callee: None` keeps the existing `func`/args/dst known-`None` check. Typed
`Global`, `Extern`, `Constructor`, and receiver-less `Method` have no ownership
target; typed `Method` receivers and `Value` targets must be known `None` before
liveness. `Closure` captures and `me_capture` remain generic `used_values`
liveness until the pre-canonical construction shape is retired. Typed calls
never consult the legacy `func` field, and `ManagedCallOwnershipUnsupported`
remains the fail-fast terminal until a named managed-call ABI issuer exists.

Production callers remain zero until the atomic canonical-owner cutover.
