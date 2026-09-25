# MIR-CALL-R6S0-CALLABLE-KEY-PROJECTION-S0 — R6-S0 bounded split row

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R6S0-BOUNDARY-SELECTION-D0 (accepted, boundary S0-A)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             section "R6/R7 migration program" (~line 127) row R6-S0.

## Decision

Bounded behavior-neutral split row for accepted boundary S0-A:
extract the `Callee`/`Global` -> `(CanonicalSameModuleCallableKeyV1,
receiver_param_count)` projection into one neutral schema-side visitor
owner, `src/mir/ssot/callable_key.rs`, and rewire the duplicated or
cross-seam consumers to it.

- Source authority + canonical issuer: `Callee`/`MirCall` schema
  remains in `hakorune_mir_defs::call_unified`; the projection is a
  pure read-side visitor primitive with no issuance power.
- Non-authority: no receipt, no guard, no new authority, no fallback.
  `published_backend_view/` stops hosting schema-level projection;
  `verification/invoke.rs` stops re-implementing the mapping.
- Fail-fast boundary: `ordinary_callable_key`/`ordinary_call_receiver`
  keep their `Err(fault(...))` terminals; the verifier keeps its
  Option-shaped adapter — semantics unchanged.
- Smallest next slice: this row only — move five helpers, rewire six
  production files, keep signatures/semantics identical.
- Non-claims: no deletion (moved helper modules keep forwarding
  re-exports if call sites require), no semantic growth, no transport
  split (S0-B stays queued), no carrier fan-in unification (declined).

## Scope

### Inventory

- `src/mir/ssot/callable_key.rs` (new): `static_method_key`,
  `free_function_key`, `expected_physical_arity`,
  `ordinary_callable_key`, `ordinary_call_receiver`.
- Moved-from: `published_backend_view.rs` (~lines 695-729),
  `published_backend_view/physical_program_call_helpers.rs`.
- Rewired callers: `physical_program.rs`, `physical_program_json.rs`,
  `compiled_entry_contract.rs`, `lifecycle_admission.rs`,
  `parameter_entry_backend_capability.rs`,
  `verification/invoke.rs` + `invoke_map.rs`.
- Existing re-export barrels (`published_backend_view.rs:48-49`,
  `physical_program.rs:29`) must keep resolving — keep or re-point
  `pub(crate) use` paths to the new owner.

### Refusal

Do not repeat the aggregate caller inventory; the census set is fixed
by the accepted D0 boundary. Do not touch `emitters/calls.rs`,
`mir_json_v0`, `methods.rs::call`, or any carrier fan-in — different
rows.

### Reopening

No reopened rows. If the extraction reveals the two implementations
are NOT identical (e.g. divergent fault strings callers depend on),
record the divergence and return to `design_stop` instead of changing
semantics in this slice.

## Exit

- [x] `src/mir/ssot/callable_key.rs` (104 lines) owns the projection:
  `static_method_key`/`free_function_key`/`expected_physical_arity`
  removed from `published_backend_view.rs` (now 706 lines);
  `physical_program_call_helpers.rs` reduced to a forwarding
  re-export shim so existing barrel paths keep resolving;
  `verification/invoke.rs::cataloged_edge_key` delegates to the new
  owner (Option adapter only — identical coverage).
- [x] All six caller files compile and resolve via the new owner
  (`physical_program.rs`, `physical_program_json.rs`,
  `compiled_entry_contract.rs`, `lifecycle_admission.rs`,
  `parameter_entry_backend_capability.rs`, `verification/invoke.rs`
  + `invoke_map.rs`).
- [x] `cargo check --lib --tests --profile quick` clean (baseline
  warnings only; none in touched files). `cargo test
  verification::invoke` all pass incl. `cataloged_call_*` and
  `map_argument_borrow_*` edges; `cargo test published_backend_view`
  90 pass / 6 fail — all six are recorded rows in
  `tools/checks/manifests/cargo_lib_red_baseline.failures.txt`
  (lines 74-76, 93-95; emit-lane/preflight/backend debt, projection
  not reached).
- [x] Guards green: `current_state_pointer_guard`,
  `mir_root_facade_guard` (exports=129, modules=215),
  `mir_root_import_hygiene_guard`, `git diff --check`,
  `repository_artifact_lifecycle_inventory --check --strict` (re-pinned
  for the two new files). No file crosses 800 lines.

Fault strings retained verbatim
(`[freeze:contract][published-lifecycle-program/*]`) — split is
behavior-neutral by construction; no receipt, no guard, no deletion
of call sites, no semantic change.
