# Trivial Binding SSA Lowering

This box is the production materializer for the pre-Builder trivial owner
profile. It owns mechanical BindingRef-to-ValueId SSA construction and
canonical CFG sealing only.

It must not read the legacy flat value map, RegionFlow effects, name-keyed
bindings, or legacy RC insertion policy. Representation and source coverage
come only from the sealed trivial profile; PHI placement comes only from
`BindingSsaBuilderV1`.

Exact static `i64` parameters enter through `parameter_entry.rs`. It consumes
the sealed profile row, checks the reserved formal `ValueId` and exact MIR
signature, then publishes that value into Binding SSA. It must not allocate a
replacement value or reconstruct source type/name authority.

`callable_abi.rs` is the route-local metadata boundary. It installs completed
callable declarations from sealed rows before body effects and refreshes
parameter then return boundary carriers on the unpublished draft. Raw source
return annotations and broad compatibility type mapping must not cross this
facade. R0a-I1 installs an exact typed result only from the co-sealed return
witness; raw annotation reads remain forbidden in resolved lowering.

`direct_call.rs` is the P0c-I1/P0c-B1 materialization boundary. It consumes one
whole `VerifiedTrivialDirectCallV1`. The function input transports the caller
header and the row transports the target header; Lower checks the current
unpublished draft only against the caller, then emits the target from the row.
It publishes one explicit VM-only capability. Raw call names, module-table
lookup, legacy call builders/resolvers, fallback, and ownership operations are
forbidden here.
