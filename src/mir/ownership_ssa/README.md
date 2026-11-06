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

`verify_ownership_ssa_v1` has one production caller: the published-MIR backend capability check in `src/mir/backend_capability.rs` (reached through `enforce_mir_backend_supported` from the runner exec paths). No other production code calls it; the canonical-owner cutover is still ahead.

`published_homes.rs` runs the same verifier for functions that create object Homes (`NewBox`) but carry no checked move, shared handle or take. Aliases and initial publication are derived from the published MIR itself (`NewBox` results, `Birth` receivers, the single `Copy` of a result); only the declared `take` permission maps come from the published input. Functions it cannot model (Shared handles, handle or text results, non-scalar parameters, calls with a managed operand and no declared take, edge arguments) return `OutsideModel` and are still checked by the source-to-MIR correspondence check. Its production caller is the same `verify_source_moves_on_published_mir` loop.
