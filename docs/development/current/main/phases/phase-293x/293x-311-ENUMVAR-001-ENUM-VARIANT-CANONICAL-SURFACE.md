# 293x-311 ENUMVAR-001 enum variant canonical surface

Status: complete

## Decision

Decision: accepted.

`Type::Variant` is the canonical enum variant spelling. `.` remains object
field/method access, so transition metadata and examples must not keep
`Enum.Value` as the canonical form.

## Scope

- Keep existing `Type::Variant`, `Type::Variant(...)`, and
  `Type::Variant { ... }` constructor surfaces.
- Add `Enum::Unit` to the EBNF `qualified_ctor` shape.
- Parse transition metadata state references with canonical `Enum::Value`.
- Preserve legacy transition metadata `Enum.Value` as compatibility input, but
  normalize transported metadata to `Enum::Value`.

## Non-goals

- No enum type checker.
- No transition legality checker.
- No general static-method migration for `::`.
- No dot-variant constructor support.

## Guard

- `tools/checks/k2_wide_enum_variant_canonical_surface_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_enum_variant_canonical_surface_guard.sh` passed locally.
