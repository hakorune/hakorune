# 293x-312 LOCALTYPE-001 local type annotation metadata capsule

Status: complete

## Decision

Decision: accepted.

Local type annotations are required before typed array literal context can be
implemented cleanly. This row only transports source metadata and does not add
type checking or inference.

## Scope

- Parse `local name: Type` and `local name: Type = expr`.
- Keep typed local declarations single-binding in the MVP.
- Add AST `Local.declared_type_names` metadata aligned with local variables.
- Carry local declared type metadata through AST JSON and Program JSON v0.
- Preserve existing untyped local and `local ... fini` behavior.

## Non-goals

- No local type checker.
- No expected-type propagation.
- No array literal semantics.
- No `PackedArray` planner.

## Guard

- `tools/checks/k2_wide_localtype_metadata_capsule_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_localtype_metadata_capsule_guard.sh` passed locally.
