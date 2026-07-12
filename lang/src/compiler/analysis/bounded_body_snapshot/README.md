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

`strict_json_tree_v0.hako` is the only Hako-side ingress facade for the
invocation-scoped opaque strict-JSON arena. Rust injects one session handle and
the root node; Hako receives only generic kind/object/array/scalar accessors.
It does not open or close sessions, receive `MapBox`, recover ProgramV0
meaning in Rust, or publish a carrier that retains a handle/node reference.
ProgramV0 field closure, classification, paths, budgets, traversal, and
snapshot publication remain Hako-owned reader responsibilities.

`reader_root_v0.hako` owns only the ProgramV0 envelope and empty-body slice.
It enumerates ordered generic object members, closes the thirteen root fields,
validates version/kind/body and optional root container shapes, and publishes
only a complete zero-node snapshot for an empty body. A non-empty body remains
an explicit pending `Unsupported` until the statement-family reader lands.
The internal root envelope may retain the body node only during the same
invocation; published snapshots retain no session handle or node id.

`reader_common_v0.hako` owns only generic JSON-kind diagnostic spelling.
Reader families share it instead of duplicating `type.expected_*.got_*`
construction; it owns no ProgramV0 fields, tags, traversal, or outcomes.
