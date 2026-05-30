---
Status: Landed
Date: 2026-05-30
Scope: probe the emitted symbol table so the public ArrayBox / DirectArray / legacy helper boundary is pinned by actual exports, not by source surface alone.
Blocker: SYMBOL-PRESENCE-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-391-PUBLIC-ARRAYBOX-RUNTIME-SURFACE-CLASSIFIER-REFRESH.md
  - target/release/libnyash_kernel.a
  - crates/nyash_kernel/src/exports/birth.rs
  - crates/nyash_kernel/src/exports/typed_object.rs
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
  - crates/nyash_kernel/src/plugin/array_runtime_aliases.rs
---

# 296x-392 Symbol Presence Probe

## Purpose

Row391 source-classified the public ArrayBox runtime surface. The remaining
question is whether the emitted symbol table confirms the split or whether the
boundary still hides an export mismatch. Probe the built artifact symbol table
and choose exactly one next diagnostic owner.

## Contract

```text
output_contract=symbol-presence-probe-v0
input_contract=public-arraybox-runtime-surface-classifier-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
symbol_table_source=target/release/libnyash_kernel.a
public_array_birth_symbol_present=1
direct_array_birth_symbol_present=1
legacy_object_field_symbol_present=1
legacy_object_exact_slot_symbol_present=1
source_surface_remains_public_arraybox_and_directarray_split=1
selected_next=typed_object_legacy_field_helper_callsite_inventory
selected_reason=emitted_symbols_confirm_public_and_direct_array_surfaces_but_legacy_field_helper_callsites_still_need_attribution
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Mini Task Board

Keep each item small enough for a mini worker. This row is still docs/report
only. Do not open implementation.
Treat each task below as independently runnable. Do not bundle multiple files
into one worker pass.

### SPR-001: Row391 Evidence Sync

Input:
- row391 card
- current state

Output:
- confirm row392 is landed
- confirm row393 is current

Acceptance:
- row392 is `Status: Landed`
- row393 is `Status: Current`
- no DirectArray optional member selection is opened

### SPR-002: Public And Direct Birth Symbols

Input:
- `target/release/libnyash_kernel.a`

Output:
- report whether `nyash.array.birth_h` is emitted
- report whether `nyash.array.direct_i64.birth_h` is emitted

Acceptance:
- both symbols are present in the emitted symbol table
- symbol presence is not treated as perf evidence

### SPR-003: Legacy Helper Symbol Group

Input:
- `target/release/libnyash_kernel.a`

Output:
- report whether legacy `nyash.object.field_*` and `nyash.object.exact_slot_*`
  symbols are emitted

Acceptance:
- both helper groups remain visible in the emitted symbol table
- symbol presence is not treated as perf evidence

### SPR-004: Next Owner Selection

Input:
- SPR-002 through SPR-003 outputs

Output:
- choose exactly one next owner:
  - `typed_object_legacy_field_helper_callsite_inventory`

Acceptance:
- one owner selected
- the reason mentions that the emitted symbols confirm the split but not the
  callsite attribution
- implementation remains closed

### SPR-005: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row392 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_symbol_presence_probe_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The emitted symbol table is now the right probe boundary. It confirms both the
public ArrayBox birth symbol and the DirectArray birth symbol are exported,
while the legacy field helpers remain emitted too. The next durable diagnostic
owner is therefore the typed-object legacy field helper callsite inventory.

```text
selected_next=typed_object_legacy_field_helper_callsite_inventory
selected_reason=emitted_symbols_confirm_public_and_direct_array_surfaces_but_legacy_field_helper_callsites_still_need_attribution
```

## Acceptance

- row391 real surface classifier is the input
- emitted symbol presence is checked directly from the built artifact
- public ArrayBox birth and DirectArray birth stay distinct
- legacy helper and exact-slot exports stay visible
- next selected row is docs-first
- no implementation is opened

## Forbidden

- no new DirectArray member
- no helper micro-optimization
- no provider activation
- no allocator replacement
- no hook installation
- no `#[global_allocator]`
- no public ArrayBox handle reinterpretation

## Guard

```bash
bash tools/checks/k2_wide_phase296x_symbol_presence_probe_guard.sh
```
