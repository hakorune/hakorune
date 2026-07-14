# Trivial Binding SSA Lowering

This box is the production materializer for the pre-Builder trivial owner
profile. It owns mechanical BindingRef-to-ValueId SSA construction and
canonical CFG sealing only.

It must not read the legacy flat value map, RegionFlow effects, name-keyed
bindings, or legacy RC insertion policy. Representation and source coverage
come only from the sealed trivial profile; PHI placement comes only from
`BindingSsaBuilderV1`.
