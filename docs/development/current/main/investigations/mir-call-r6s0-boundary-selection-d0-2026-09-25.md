# MIR-CALL-R6S0-BOUNDARY-SELECTION-D0 — R6-S0 exact boundary selection

Status: accepted__2026-09-25__boundary=S0-A
Date: 2026-09-25
Parent: SELFHOST-RESUME-ENTRY-RECHECK0-P0 (closed 2026-09-25)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
  (staged R6-S0..R7 queue, "remains unopened until its exact boundary
  is selected")
Implementation permission: false; this is a design_stop selection row.
No code, route, fixture, caller, or fallback change.

## Purpose

The Call/R7 queue is unopened only because the R6-S0 split boundary was
never selected. This row selects exactly which instruction-schema/visitor
owners and which published-view/transport owners split before any
semantic growth — or records `NoSafeSlice` with an explicit reopen
trigger. It does not re-run the forbidden aggregate inventory and does
not grant implementation permission to any downstream row.

## Six-line brief

```text
Decision: name the R6-S0 boundary — one instruction-schema/visitor
  owner set and one published-view/transport owner set — as the first
  behavior-neutral split of the staged queue, or NoSafeSlice.
Source authority + canonical issuer: the R7 retirement card's staged
  queue definition and the existing owner census fixtures; this card
  selects a boundary, it does not mint authority.
Non-authority: no inventory repeat, no semantic growth, no caller
  switch, no deletion, no new semantic receipt.
Fail-fast boundary: if no boundary has a finite caller set plus a
  behavior-neutral split shape, the row records NoSafeSlice and parks
  the queue again with a reopen trigger.
Smallest next slice: boundary candidate table (owner, callers, split
  shape, delete-set impact) plus one accepted Decision.
Non-claims: does not open R6-S1..R7; does not fix the EXE suite reds;
  does not touch B3 or language-v1 rows.
```

## Census boundary

`このcensusが覆う境界: staged R6-S0 queue description -> one accepted
split boundary; includes instruction-schema/visitor and
published-view/transport owner candidates under src/mir/**; excludes
semantic changes, caller migration, deletion, and any other queue stage.`

## Exit

An accepted Decision naming the R6-S0 boundary (owners, files, split
shape, finite caller set, planned delete-set impact) or `NoSafeSlice`
with an observable reopen trigger. On acceptance, work_mode returns to
`fast` for the bounded split row and `next_execution_card` names it.

## Worker census result (2026-09-25, integrated)

The nominal schema/view/transport split is already physically realized
(`hakorune_mir_defs` crate owns `Callee`/`MirCall`/`CallFlags`;
published-view files are all reader-only; JSON emit/parse owners are
separate files). Three live cross-owner duplications remain:

1. `Callee`/`Global` target -> `(CanonicalSameModuleCallableKeyV1,
   receiver_param_count)` projection is owned twice:
   `published_backend_view.rs` private `static_method_key`/`free_function_key`
   + `expected_physical_arity` (lines ~695-729) feeding
   `physical_program_call_helpers.rs::ordinary_callable_key`/`ordinary_call_receiver`,
   and `verification/invoke.rs::cataloged_edge_key` (~lines 316-339)
   re-implements the identical mapping. External callers
   (`lifecycle_admission.rs`, `parameter_entry_backend_capability.rs`)
   reach into the published-view owner for a schema-level fact.
2. v0 wire vocabulary (`"boxcall"`/`"externcall"`) is owned in 3-4
   places (emit `emitters/calls.rs`, parse `mir_json_v0/module.rs`,
   Stage-A reject scanner, vm_hako subset validators) — belongs to a
   later compat-retire stage, not S0.
3. Carrier fan-in (`Call`|`LegacyCallV0`|`ExternCall`) is re-matched
   inline in >=6 visitor consumers — unifying them is a semantic
   choice, declined for S0 (not behavior-neutral).

## Accepted Decision — boundary S0-A

R6-S0 boundary = extract the callee -> cataloged-callable-key +
receiver-slot projection into one neutral schema-side visitor owner.
Both existing implementations are line-for-line identical modulo
return-shape fusion, so the split is provably behavior-neutral.

- New owner: `src/mir/ssot/callable_key.rs` (sibling of the existing
  `ssot/method_call.rs`/`ssot/closure_call.rs` visitor owners).
  Exported per existing `ssot` visibility (`pub(in crate::mir)` or
  `pub(crate)` as callers require).
- Moved owners: `static_method_key`, `free_function_key`,
  `expected_physical_arity` (out of `published_backend_view.rs`),
  `ordinary_callable_key`, `ordinary_call_receiver` (out of
  `physical_program_call_helpers.rs`).
- Rewired callers (6 production files): `physical_program.rs` (:290,
  :368, :474), `physical_program_json.rs` (:517, :556),
  `compiled_entry_contract.rs` (:357, :365), `lifecycle_admission.rs`
  (:78), `parameter_entry_backend_capability.rs` (:80-84),
  `verification/invoke.rs::cataloged_edge_key` + `invoke_map.rs:47`
  (delegates to the new owner; verifier keeps only its Option-shaped
  local adapter if the signature differs).
- No deletion, no receipt, no guard, no caller-visible behavior change.
- Named follow-up, not in this split: S0-B transport v0/v1 emitter
  split (`emitters/calls.rs` v1 writer vs v0 boxcall/externcall
  projection + dst-type hint table) is a candidate sibling row inside
  the same staged queue; S0-D carrier fan-in unification is declined
  (semantic, not neutral).

Next execution row: `MIR-CALL-R6S0-CALLABLE-KEY-PROJECTION-S0` —
behavior-neutral extraction of the projection owner and rewiring of the
six caller files above.
