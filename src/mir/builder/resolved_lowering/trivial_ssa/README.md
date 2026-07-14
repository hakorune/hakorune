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
