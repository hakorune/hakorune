# Bounded Body Snapshot V0

This directory owns the Hako reader for a prevalidated structured ProgramV0
body and the immutable snapshot it publishes.

It must not parse raw JSON, scan source text, infer source kinds, mutate the
input, emit MIR, allocate IDs, select routes, build plans, or execute runtime
behavior. `snapshot_builder_v0.hako` is the only mutable publication capsule;
failure discards it and no partial snapshot is exposed.
