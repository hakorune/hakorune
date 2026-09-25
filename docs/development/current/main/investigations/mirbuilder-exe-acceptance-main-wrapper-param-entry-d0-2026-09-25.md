# MIRBUILDER-EXE-ACCEPTANCE-MAIN-WRAPPER-PARAM-ENTRY-D0

Status: accepted__2026-09-25
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

## Decision (worker census + owner audit, 2026-09-25)

**Option W — adopt the injector's already-published `variable_map`
values as the wrapper's entry values.** The injector
(`decls.rs:302-363`) remains the sole physical owner of `main(args)`
materialization; the callable ledger adopts exactly the values the
publisher installed. `main(args)` is the documented retained
bootstrap argv compatibility contract — the injected local IS its
established meaning on this lane, not a new semantic authority.

Option C is rejected for this slice: it forks the entry contract
(`main` shim + cataloged `Main.main/N`) and needs four authorities
(app-main selection role, wrapper→callee call emission owner,
selected-batch coverage accounting, return/result mapping), plus it
risks re-entering the documented "two physical meanings of Main.main"
pattern quarantined behind `NYASH_BUILD_STATIC_MAIN_ENTRY`.

### Answers to the census questions

1. **W legality**: `install_entry_values` never reads
   `current_function.params` — it checks prior install, receiver
   parity, positional count, `ValueId` uniqueness, then delegates to
   `dynamic_origins.install_entry`, which requires only
   binding↔ordinal↔value uniqueness (`formal_by_ordinal` comes from
   `issue_formals` over `Parameter{index}` bindings, not from MIR
   formals). `read_variable`, `rebind`, `local_placement`, PHI
   (`compute_def_blocks` accepts instruction dsts), KeepAlive, and
   parameter-entry contracts are all formality-agnostic.
   `static_from_values` (`body_state_bridge.rs:59-60`) is the
   existing precedent for a non-`params` snapshot.
2. **C authority surface**: new selected-row role for app-main
   (`source_backed.rs:237` excludes it), new wrapper call-emission
   owner, root restructure + coverage accounting — a route
   restructure, not a bounded slice.
3. **One meaning**: `args` remains a *source* `Parameter{index}`
   (the semantic ledger's declaration stays true) whose *physical*
   entry realization on the wrapper route is the injector's local.
   Physical-formal membership stays disjoint from semantic binding
   — the same separation `dynamic_origins` already tolerates.
4. **W fail-fast tuple**: `callable-entry/injected-local-missing`
   (new; a declared param name absent from `variable_map`) ·
   `entry-shape-mismatch` (existing count guard) ·
   `callable-dynamic-origin/DuplicatePhysicalValue` ·
   `FormalOrdinalMismatch` · `duplicate-entry-install`.
   Never synthesize a name, never widen `function.params`, never
   iterate `variable_map` in map order.
5. **C script-args**: rejected arm — runtime-input snapshot keeps
   feeding the injector under W, unchanged.

### Bounded next slice — MIRBUILDER-EXE-ACCEPTANCE-MAIN-WRAPPER-PARAM-ENTRY-S0

1. `normal_callable_binding_materialization_port.rs`: add
   `CallableEntryShapeV1::StaticInjectedLocals{parameter_names:
   Box<[String]>}` (drop `Copy` on the enum if required) whose
   `prepare_values` snapshots `variable_map[name]` in declared order
   and fails `injected-local-missing` on an absent name.
2. `main_root.rs` (`with_app_main_root_lowering_input` adoption
   site): derive `(index, name)` from `input.forest().owner(owner)`
   `Parameter` sites → `declaration_binding` → `diagnostic_name`;
   select `StaticInjectedLocals` when declared params exist and the
   physical formal count is 0; keep `Static{0}` for arity-0.
3. Pins: (a) unit — `main(args)` installs entry binding
   `Parameter{0}`→injected pid and reaches body lowering;
   (b) negative — missing injected name fails
   `injected-local-missing`; (c) arity-0 route unchanged
   (`Static{0}` still selected).
4. Guard: extend the qualified-route-scope guard (or a sibling row
   guard) to pin the new shape's name-ordered snapshot and the
   `function.params` non-widening invariant.
5. Non-claims: no EXE green claim; downstream boundaries
   (loop-cond, birth, NamedArray, qualified String result) remain
   open and are recorded, not fixed here.

### Non-claims

- No change to arity-0 `main()` behavior (still `Static{0}` inline).
- No cataloged `Main.main/N` consumption on the installed lane.
- No change to the raw-compat lane or `NYASH_BUILD_STATIC_MAIN_ENTRY`.
- Injected values are never entered into `function.params` or
  `declared_param_decls` — physical formals and semantic Parameter
  bindings stay disjoint.

## Boundary

- Includes: authority naming, issuer inventory, one bounded S-card
  if the tuple completes.
- Excludes: implementation; ordinary-new claim family; NamedArray;
  ingest body loops downstream.

## Exit

- [x] Decision recorded (bounded slice OR sealed expansion) with
      the six-line brief.
- [x] Next S-card emitted OR NoSafeSlice with reopen triggers;
      pointers synced.
