# MIR-CALL-R6S0-TRANSPORT-EMITTER-SPLIT-S0 — R6-S0 sibling split row

Status: landed__2026-09-25
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

- [x] `emitters/calls.rs` (304 lines incl. tests) keeps only the v1
  canonical writer + profile dispatch; `calls_compat_v0.rs` (225
  lines) owns the v0 `boxcall`/`externcall`/`call` projection +
  `dst_type` hint table + `emit_call_with_callee_v0`/
  `emit_call_with_optional_func`/`emit_externcall_with_name`.
  `emit_new_box`/`emit_new_closure` (non-call emitters) stay in
  `calls.rs`; sole caller `emitters/mod.rs` unchanged (2 sites).
- [x] Emitted JSON unchanged: all 155 `mir_json_emit` tests pass
  untouched, including byte-shape assertions on v0 boxcall/externcall/
  legacy-func/print-route and v1 mir_call arms; v0 code moved verbatim
  (dead `is_canonical_v1` early-returns retained verbatim inside the
  compat owner).
- [x] `cargo check --lib --profile quick` clean; no warnings in touched
  files; no baseline red deltas.
- [x] Guards green: `current_state_pointer_guard`,
  `mir_root_facade_guard` (modules=215), `mir_root_import_hygiene_guard`,
  `git diff --check`, `repository_artifact_lifecycle_inventory
  --check --strict` (re-pinned). Max file 304 lines.

R6-S0 boundary S0 complete (S0-A + S0-B). Remaining queue: R6-S1
canonical producer cohort selection (requires per-row
Promote/Stop/Delete tuple), R6-S2 backend boundary, R6-S3 quarantine,
R7 caller-zero retirement.
