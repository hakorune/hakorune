# 296x-1378 RUST-LIFECYCLE-PROJECTION-SSOT-001

Status: planned
Date: 2026-06-20

## Purpose

Close the lifecycle projection design as a post-aggregation task.

This row is planned after:

```text
296x-1377-HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-001
```

## SSOT

```text
docs/development/current/main/design/rust-lifecycle-projection-ssot.md
```

## Decision

Rust lifecycle migration is not modeled by importing Rust lifetime / borrow
syntax into `.hako`.

Instead:

```text
rustc semantic adapter:
  emits Rust lifecycle facts

Hako lifecycle resolver:
  chooses Hako lifecycle plan

Hako lifecycle verifier:
  validates the projection

emitter:
  emits only verified .hako / canonical MIR
```

## Planned Follow-Up

```text
HAKO-LIFECYCLE-PLAN-VOCAB-000
MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-PILOT-001
```

## Stop Line

```text
implementation_started=0
rust_lifetime_syntax_added=0
converter_owns_rust_lifecycle_inference=0
rust_adapter_chooses_hako_representation=0
crate_bundle_aggregation_scope_changed=0
```

This row must not replace 296x-1377. It is the next design milestone after the
crate-bundle transport milestone is closed.
