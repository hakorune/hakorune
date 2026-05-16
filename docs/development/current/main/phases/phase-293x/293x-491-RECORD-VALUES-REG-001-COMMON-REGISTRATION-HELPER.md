# 293x-491 RECORD-VALUES-REG-001 Common Registration Helper

Status: landed
Date: 2026-05-16

## Decision

`RECORD-VALUES-REG-001` is the BoxShape cleanup selected by
`USERBOX-ROUTE-SPLIT-006`.

`src/mir/builder/record_values.rs` has two builder-local construction paths:
record constructor calls and record literals. Both create
`RecordLocalFieldValue` rows and then register a void placeholder as a
builder-local record value. This row centralizes that repeated construction /
registration step without changing the accepted record surface.

## Scope

- Add a small helper for building one `RecordLocalFieldValue`.
- Add a small helper for registering builder-local record fields behind a void
  placeholder.
- Use the helpers from constructor and literal paths.
- Keep record declaration validation, arity checks, duplicate/missing/unknown
  field diagnostics, and field-read lowering unchanged.

## Stop Lines

- Do not add, remove, or rename accepted record syntax or record lowering
  shapes.
- Do not enable record escape, record materialization, typed-object record
  lowering, packed storage, backend lowering, or allocator/provider behavior.
- Do not change diagnostic tags or error wording except where helper extraction
  mechanically preserves the existing messages.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `RVREG.1` | Document row and owner. | Current points to this card. | no code before docs |
| `RVREG.2` | Extract common field/register helpers. | Constructor/literal code share helper. | no acceptance change |
| `RVREG.3` | Keep focused tests green. | record-related tests pass. | no backend changes |
| `RVREG.4` | Closeout docs and advance current. | Required evidence is green. | no provider activation |

## Required Evidence

```text
cargo test -q record_literal
cargo test -q source_to_program_json_v0_lowers_record_field_read
cargo test -q source_to_program_json_v0_emits_record_literal_shape_metadata
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

This row is landed.

Implementation:

- Added a common `build_record_local_field_value()` helper for constructor and
  literal field expression lowering.
- Added a common `register_record_local_fields()` helper for placeholder
  creation and builder-local record registration.
- Kept record declaration validation, arity checks, literal field diagnostics,
  field-read lowering, record escape stop lines, and backend/provider behavior
  unchanged.

Evidence:

```text
cargo test -q record_literal
cargo test -q source_to_program_json_v0_lowers_record_field_read
cargo test -q source_to_program_json_v0_emits_record_literal_shape_metadata
```

Next:

```text
RECORD-VALUES-REG-002:
  post-record-values-helper row selection
```
