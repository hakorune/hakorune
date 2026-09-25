# MIR-CALL-R6S0-TRANSPORT-EMITTER-SPLIT-S0 — R6-S0 sibling split row

Status: selected__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R6S0-BOUNDARY-SELECTION-D0 (accepted; S0-A landed)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             section "R6/R7 migration program" (~line 127) row R6-S0.

## Decision

Bounded behavior-neutral split row for the second half of accepted
boundary S0: separate the v1 canonical call writer from the v0
compatibility projection inside the JSON transport emitter.

- Source authority + canonical issuer: `JsonEgressProfile`
  (`runner/mir_json_emit/root.rs`) already gates v1 vs v0; this row
  only moves code, it does not change profile semantics.
- Non-authority: no receipt, no guard, no new authority, no fallback.
- Fail-fast boundary: existing reject/terminal paths in both arms are
  moved verbatim; no error text changes.
- Smallest next slice: split `emitters/calls.rs` (498 lines) into
  `calls.rs` (v1 writer + dispatch) and `calls_compat_v0.rs` (v0
  `boxcall`/`externcall` projection + `dst_type` hint table); sole
  caller is `emitters/mod.rs` (2 sites).
- Non-claims: no deletion of the v0 path (compat retire is R6-S3/R7),
  no parse-side move (`mir_json_v0/` stays), no Stage-A scanner
  changes, no vm_hako validator changes.

## Scope

### Inventory

- `src/runner/mir_json_emit/emitters/calls.rs`: `emit_call()` v1 arm
  (:17-27) stays; v0 `boxcall` arm (:43-76) + by-name `dst_type` hint
  table (:59-74) + `SameModuleInstance`/`BirthConstructor` v0 arms
  (:77-120) move out; `emit_new_box`/`emit_new_closure` stay with
  whichever arm uses them (verify call graph before splitting).
- `src/runner/mir_json_emit/emitters/mod.rs`: dispatch updated to route
  v0 shapes to the new module.
- `src/runner/mir_json_emit/helpers.rs`: `emit_unified_mir_call` —
  verify whether v0 arm depends on it before the split.

### Refusal

Do not touch parse-side owners (`runner/json_v1_bridge/`,
`runner/mir_json_v0/`), Stage-A scanners, or vm_hako subset validators —
the v0 wire vocabulary retire is a later compat stage. Do not change
any emitted JSON byte.

### Reopening

If the v0 arm shares helpers with the v1 arm such that a clean file
boundary requires signature changes, record the shared set and return
to `design_stop`.

## Exit

- [ ] `emitters/calls.rs` keeps only the v1 canonical writer +
  dispatch; `calls_compat_v0.rs` owns the v0 projection + hint table.
- [ ] Emitted JSON is byte-identical before/after on a v0-profile and a
  v1-profile fixture (diff evidence).
- [ ] `cargo check --profile quick` clean; focused mir_json_emit tests
  pass; baseline reds classified.
- [ ] Pointer guards green; no file crosses 800 lines.
