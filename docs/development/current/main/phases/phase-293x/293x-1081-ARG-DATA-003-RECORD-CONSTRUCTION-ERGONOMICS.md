# 293x-1081 ARG-DATA-003 Record Construction Ergonomics

Status: completed
Date: 2026-05-21

## Purpose

Reduce wide report argument / field-copy noise by making record construction
less repetitive before continuing allocator comparison closeout work.

## Scope

- Accept record field defaults: `record R { x: i64 = 0 }`.
- Accept empty record literals when omitted fields have defaults: `R {}`.
- Accept same-name shorthand: `R { x }` as `R { x: x }`.
- Lower tracked-record `with` update: `r with { x: 1 }`.
- Keep record-local scalarization compiler-local.

## Stop Lines

- No `...fields` spread.
- No named function arguments.
- No automatic record-to-box copy.
- No ordinary-box `with` copy/update.
- No `::default()` surface.
- No runtime record object.
- No record return ABI.
- No backend record route.

## Validation

Required:

```bash
cargo test -q parser_record_literal_surface
cargo test -q record_construction_ergonomics
```

Evidence:

- `cargo check -q`
- `cargo test -q parser_record_literal_surface`
- `cargo test -q record_construction_ergonomics`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Resume

After ARG-DATA-003 lands, return to MIMAP-456A result ledger closeout unless a
new compiler acceptance blocker appears.
