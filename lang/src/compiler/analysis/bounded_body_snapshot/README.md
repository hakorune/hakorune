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

Tiny JSON-kind diagnostic formatters remain private to each reader family.
The spelling is too small to justify another cross-module route dependency;
keeping it local does not move semantic authority. Compile-time acceptance is
measured with the fixed release+VM-reference compiler, while debug compiler
timing remains diagnostic evidence only.

`reader_expr_leaf_v0.hako` owns the first accepted expression family:
`Int`, `Str`, `Bool`, `Null`, and `Var`. It closes each leaf object, emits one
ordered atom, delegates decoded text to the local checked text carrier, and
returns an observation rather than publishing a partial snapshot. Accepted
child expressions remain explicit pending `Unsupported` for the next slice.

`canonical_i64_v0.hako` owns ProgramV0 number/string normalization to i64.
It inspects only one decoded string scalar, validates canonical decimal grammar and
the full signed range before arithmetic, and accumulates negative values in
the negative direction so `i64::MIN` never requires a positive overflow.
Its `substring` use is limited to one decoded decimal scalar and never observes
raw JSON, node tags, field names, paths, token offsets, or source text. Every
structured reader module remains substring/index-search free.

`reader_expr_child_v0.hako` is the recursive expression coordinator for
`Binary`, `Compare`, and `Logical`. It observes the parent before its children,
validates the schema-owned operator partition, and preserves exact `lhs` then
`rhs` order in invocation-local typed edges. Leaf observations share the same
small read-only child interface. Call/Method/Field and flat snapshot-index
publication remain outside this slice.
