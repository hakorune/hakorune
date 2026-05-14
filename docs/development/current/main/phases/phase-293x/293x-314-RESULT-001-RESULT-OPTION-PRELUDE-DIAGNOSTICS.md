# 293x-314 RESULT-001 Result/Option prelude diagnostics

Status: complete

## Decision

Decision: accepted.

`Option<T>` and `Result<T,E>` are available as built-in enum surfaces for
Stage1 constructor lowering and diagnostics. Canonical enum constructors use
`Type::Variant`; known enum variants written with dot syntax fail-fast.

## Scope

- Add a shared Result/Option prelude enum registry.
- Seed the parser known-enum table so unit variants such as `Option::None`
  parse without explicit source enum declarations.
- Seed Stage1 Program JSON v0 known enum lookup so `Result::Ok`,
  `Result::Err`, `Option::Some`, and `Option::None` lower without explicit
  enum declarations.
- Preserve existing `Option::Some(null|void)` fail-fast contract.
- Reject `Result.Ok(...)` / `Option.None` when the target is a known enum
  variant.

## Non-goals

- No `try`, `throw`, or `?` sugar.
- No `guard let`.
- No unqualified `Ok(x)` canonical constructor.
- No generic enum type inference.
- No extra Program JSON v0 synthetic `enum_decls` output.

## Guard

- `tools/checks/k2_wide_result_option_prelude_diagnostics_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_result_option_prelude_diagnostics_guard.sh` passed locally.
