# Bounded Body Snapshot V0

This directory owns the Hako reader for a prevalidated structured ProgramV0
body and the immutable snapshot it publishes.

It must not parse raw JSON, scan source text, infer source kinds, mutate the
input, emit MIR, allocate IDs, select routes, build plans, or execute runtime
behavior. Published nodes form one flat preorder table. Atoms are ordered
`(key, kind, value)` records and children are ordered `(role, target_index)`
records; neither is a map. `snapshot_builder_v0.hako` is the only mutable
publication capsule; failure discards it and no partial snapshot is exposed.

`validated_text_v0.hako` owns the Hako-local decoded text witness. Its normal
factory path is `atom` or `literal`: it derives the UTF-8 byte count through
the internal capability, applies the bounded text budget, and retains only
scalar value/count/class fields. It does not retain a structured input node,
`MapBox`, `ArrayBox`, or `PathV0`. Hako has no runtime-private field seal, so
factory-only construction and replay-only isolation are repository guards.
