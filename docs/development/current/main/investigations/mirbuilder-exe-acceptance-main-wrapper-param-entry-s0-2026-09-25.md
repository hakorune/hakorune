# MIRBUILDER-EXE-ACCEPTANCE-MAIN-WRAPPER-PARAM-ENTRY-S0

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-MAIN-WRAPPER-PARAM-ENTRY-D0
  (accepted — Option W injected-local entry adoption)
Owner: workstream row H / unified resume gate 1
Authority: D0 Decision; injector = sole physical owner
  (`decls.rs:302-363`); `install_entry_values` is
  formality-agnostic (D0 census, verified).

## Slice

1. `src/mir/builder/normal_callable_binding_materialization_port.rs`:
   add `CallableEntryShapeV1::StaticInjectedLocals { parameter_names:
   Box<[String]> }` (drop `Copy` from the enum if the variant forces
   it — update existing pattern matches accordingly). Its
   `prepare_values` arm snapshots `variable_map[name]` in declared
   name order; an absent name fails
   `[freeze:contract][callable-entry/injected-local-missing]` —
   never synthesize a name, never widen `function.params`, never
   iterate `variable_map` in map order.
2. `src/mir/builder/normal_callable_semantic_loan_port/main_root.rs`
   (`with_app_main_root_lowering_input` adoption site, ~:212-224):
   derive declared `(index, name)` from
   `input.forest().owner(owner)` `Parameter` sites →
   `declaration_binding` → `diagnostic_name`; select
   `StaticInjectedLocals` when declared params exist and physical
   formals are 0; keep `Static{0}` for arity-0.

## Pins (required before close)

- Unit: `main(args)` installs entry binding `Parameter{0}` ->
  injected pid and proceeds past `install_entry_values` into body
  lowering (any honest downstream terminal is acceptable; the pin
  asserts `entry-shape-mismatch` no longer fires).
- Negative: a declared param name missing from `variable_map`
  fails `injected-local-missing` (constructed port-level pin, not a
  source probe).
- Regression: arity-0 `main()` keeps `Static{0}` and existing
  route/pins unchanged.

## Guard

- Extend `tools/checks/mirbuilder_qualified_route_scope_guard.sh`
  (or add a sibling row guard) to pin: the new shape arm exists,
  `variable_map` is read in declared-name order (not map
  iteration), and `function.params` is never widened.

## Fail-fast boundary

- `callable-entry/injected-local-missing` — declared param name
  absent from `variable_map` (injector did not run / name drift).
- `callable-semantic-lowering/entry-shape-mismatch` — name count !=
  `Parameter` binding count.
- `callable-dynamic-origin/DuplicatePhysicalValue` /
  `FormalOrdinalMismatch` — ordering or uniqueness violations.
- `duplicate-entry-install` — adoption invoked twice.

## Non-claims

- No EXE green claim; downstream boundaries (loop-cond, birth,
  NamedArray, qualified String result commit) remain open and are
  recorded honestly.
- No cataloged `Main.main/N` consumption; no arity-0 behavior
  change; no raw-compat lane change; injected values never enter
  `function.params` or `declared_param_decls`.

## Landed evidence

- `CallableEntryShapeV1::StaticInjectedLocals{parameter_names}` added
  (`Copy` dropped from the enum); `injected_locals` snapshots
  `variable_map[name]` in declared order, fails
  `callable-entry/injected-local-missing` on an absent name.
- `main_root.rs` adoption site derives declared `(index, name)` from
  the owner product `Parameter` sites (`declared_parameter_names_v1`)
  and selects `StaticInjectedLocals` when declared params exist;
  arity-0 keeps `Static{0}`.
- The diversion predicate now checks the DECLARED arity (`arity0`)
  instead of physical formals — the previous `parameter_count == 0`
  check was vacuous on the wrapper route (physical formals are
  structurally 0); entry adoption succeeding would have diverted
  `main(args)` into the canonical route and failed at
  `qualified-preflight`. This fold-in is the same slice: the adoption
  change is what makes the gate reachable.
- Pins: port-level `injected_locals_snapshot_follows_declared_name_
  order` + `injected_locals_missing_name_fails_named_boundary`
  (new test module); compile pin updated — `main(args)`+qualified
  String callee now reaches `published-mir-backend-view
  /StaticMethodRequiresIntegerReturn` (adoption + `inner.lower_body`
  + publication lane all exercised; String-returning static callee
  is the next owned backend-view boundary).
- Direct probe: `main(args){ local n = args.size(); return 0 }`
  emits MIR JSON successfully (`entry-shape-mismatch` gone); backend
  recipe stops at `mir_call_no_route` for `size` (separate family).
  Bare `main(){return 0}` shows the same `no_lowering_variant`
  backend stop, so the remaining probe failures are not
  `main(args)`-specific.
- Tests: 4/4 focused green (2 new port pins + updated arity pin +
  relation pin); focused regression sweep 148 pass / 4 known
  baseline debt (manifest-recorded).
- Fresh `real-apps-exe-boundary` receipt: 4 pass / 7 fail — counts
  and terminals unchanged (all arity-bearing apps still stop in
  static-child lowering before reaching the root body, so the suite
  does not yet exercise this lane).
- Guard: `mirbuilder-qualified-route-scope` extended — new shape,
  `injected-local-missing`, `declared_parameter_names_v1`,
  declared-arity diversion (`if arity0`), and both new pin names.

## Exit

- [x] Both edits landed with pins green.
- [x] Guard extended and row guard passes.
- [x] Fresh `real-apps-exe-boundary` receipt recorded with honest
      terminals.
- [x] CURRENT_STATE + workstream row H synced; committed/pushed.
