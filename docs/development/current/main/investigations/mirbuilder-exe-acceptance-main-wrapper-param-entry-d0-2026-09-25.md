# MIRBUILDER-EXE-ACCEPTANCE-MAIN-WRAPPER-PARAM-ENTRY-D0

Status: open__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-MAIN-IMPORT-ARITY-SCOPE-S0
  (landed — exposed this boundary)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  ARITY-SCOPE-S0 post-slice terminal.

## Problem

Every arity-bearing app main (`main(args)`) on the installed lane
stops at `[callable-semantic-lowering/entry-shape-mismatch]`
(`normal_callable_semantic_lowering_state.rs:477-481`). The boundary
is PRE-EXISTING and independent of the qualified-relation family:

- `open_module_main_wrapper` (`module_lifecycle.rs:450-466`) opens
  the physical `main` wrapper with `params: vec![]` — 0 formals.
- `lower_static_main_function_parts_with_port_v1`
  (`decls.rs:302-363`) materializes each source parameter as a LOCAL
  (`args` receives a `NewBox ArrayBox` filled from script args;
  other names receive void) and installs them in `variable_map`.
- The installed callable ledger (`CallableSemanticLoweringState`)
  declares each source parameter as `Parameter{index}` and
  `install_entry_values` requires the handed
  `PreparedCallableEntryValuesV1.parameters().len()` to equal the
  declared count. `CallableEntryShapeV1::Static{parameter_count}`
  snapshots `current_function.params` (0) — mismatch vs 1.

5 of the 11 real-app EXE entries declare `main(args)`
(json_stream, boxtorrent, binary_trees, mimalloc_lite,
allocator_stress); none has ever passed on this lane — earlier
stops (relation issue, ordinary-new claims, NamedArray) hid it.

## Facts established (S0 probes + source audit)

- Route scope is fixed: arity!=0 mains no longer issue the
  qualified relation nor enter the NormalMainQualifiedMethods
  diversion (ARITY-SCOPE-S0, landed).
- The wrapper injector (`decls.rs:302-363`) is the sole physical
  owner of app-main param materialization on this route; injected
  values are live in `variable_map` when
  `lower_app_main_root_body_v1` runs
  (`with_callable_source_scope` does not touch `variable_map`).
- `install_entry_values` binds declared `Parameter` bindings to the
  handed values positionally and feeds `dynamic_origins
  .install_entry` — it does not require the values to be formals.
- All canonical Main0 routes (`select_main0_root_product_v1`) are
  arity-0-only (`capability.rs:279,328`); `main(args)` always opens
  the wrapper.
- `CallableEntryShapeV1` today has exactly Static/Instance arms,
  both snapshotting `current_function.params`; no injected-local
  path exists (`normal_callable_binding_materialization_port.rs`).

## Questions

1. Option W — keep the wrapper-inline contract: extend the entry
   adoption port with a shape that snapshots the injector's locals
   (`variable_map[name]` in declared-parameter order, names taken
   from the resolved owner's `Parameter` bindings). Does binding a
   source `Parameter` to a non-formal value violate any downstream
   receipt (`dynamic_origins.install_entry`, slot registry,
   rebind/assignment lanes)?
2. Option C — canonical callable: admit `Main.main/N` as a real
   cataloged function draft and have the wrapper emit a call with
   the fabricated `args`. Required authorities: cataloged draft
   admission for the app-main callable, wrapper call emission,
   EXE-entry contract. Does this converge with the Main0 canonical
   draft family, or fork it?
3. Which option preserves "one meaning": is `args` a function
   parameter (C) or a wrapper-injected local (W) in the final
   pipeline? The source declares it as a parameter.
4. If W: what is the exact fail-fast tuple (missing injected name,
   count mismatch, foreign binding)?
5. If C: what happens to `RuntimeInputSnapshot` script-args
   materialization and the existing wrapper contract?

## Boundary

- Includes: authority naming, issuer inventory, one bounded S-card
  if the tuple completes.
- Excludes: implementation; ordinary-new claim family; NamedArray;
  ingest body loops downstream.

## Exit

- [ ] Decision recorded (bounded slice OR sealed expansion) with
      the six-line brief.
- [ ] Next S-card emitted OR NoSafeSlice with reopen triggers;
      pointers synced.
