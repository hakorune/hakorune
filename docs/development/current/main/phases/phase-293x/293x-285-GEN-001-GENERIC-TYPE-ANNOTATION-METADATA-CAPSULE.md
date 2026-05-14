# 293x-285 GEN-001 Generic Type Annotation Metadata Capsule

Status: complete
Date: 2026-05-14

## Scope

Parse and transport generic type annotations such as `Array<T>`,
`PackedArray<T>`, `Span<T>`, and nested type references in declaration metadata.

## Landed changes

- Reused the TYPE_REF parser for ordinary and weak box field annotations.
- Preserved generic annotation metadata for params, returns, records, aliases,
  brands, enum payloads, box fields, and Program JSON v0 declarations.
- Added parser and Program JSON tests for `Array<T>`, `PackedArray<T>`,
  `Span<T>`, nested generics, record type parameters, and alias metadata.
- Added a dedicated guard for the GEN-001 metadata capsule.

## Non-goals

- No generic arity checking.
- No constraint solving or `where` clauses.
- No `Array<T>` semantics.
- No `PackedArray<T>` planner or eligibility gate.
- No `Span<T>` no-escape semantics.
- No backend fallback policy.

## Guard

```bash
bash tools/checks/k2_wide_generic_type_annotation_metadata_guard.sh
```

## Next selected row

`GEN-002 generic arity check`.

`PACKED-001 PackedArray eligibility gate` remains Stage1/CorePlan-owned.
