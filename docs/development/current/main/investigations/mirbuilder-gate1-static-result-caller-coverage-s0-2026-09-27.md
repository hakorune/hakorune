# MIRBUILDER-GATE1-STATIC-RESULT-CALLER-COVERAGE-S0

Status: landed
Date: 2026-09-27
Emission: `mirbuilder-gate1-static-result-caller-coverage-d0-2026-09-27.md`
bounded S0 (Decision accepted; FreeFunction coverage + `TopLevel`
arm only).
Selection proof: `MIRBUILDER-GATE1-STATIC-RESULT-CALLER-COVERAGE-S0`
-> workstream row H.
MirBuilder goal row 1 (sole caller key issuance into `rows_by_key`),
row 3 (sole target/result ingress), row 4 (sole verified admission —
catalog declaration membership), row 6 (exact consume/drain via
`finish_empty`).

## Exact boundary

- `seal_statements` mints `free_function(name, arity)` `rows_by_key`
  declaration rows for top-level `FunctionDeclaration` statements —
  parity with `issue_source_backed_same_module_callable_catalog_v1`
  (same `validate_parameters` + `DuplicateCanonicalKey` discipline;
  the `SelectedTopLevel` source row recording is unchanged).
- `classify_source_context_v1` adds the `TopLevel` arm:
  `Located{root: TopLevel(key), site}` + `source_backed` ->
  `declarations` absent -> `DeclarationCatalogUnavailable`;
  `catalog.declaration(&free_function(key.declared_name(),
  u32::try_from(key.declared_arity())))` unresolved ->
  `ForeignLineage`; resolved AND `declaration_for(StaticBoxMethod,
  owner, method, argc)` `Some` -> `Cataloged{caller: decl.key(),
  site}`; resolved AND probe `None` -> `Unavailable` (mirrors the
  `Cataloged` arm for non-`StaticBoxMethod` callers).
- The remaining grouped foreign arm keeps
  `ScriptRoot | InstanceConstructor(_) | NestedBoxMethod{..}`.
- `src/mir/builder/README.md` lineage-table update; no other
  caller-family change.

## Out of scope

- `InstanceConstructor`/`BirthConstructor` row co-seal — atomic S1
  (`MIRBUILDER-GATE1-STATIC-RESULT-CALLER-COVERAGE-S1`).
- `NestedBoxMethod`/`ScriptRoot` — permanent exclusion.
- Solver/proof candidacy — stays `StaticBoxMethod`-only.
- Any runtime/VM/legacy-path change.

## Landed implementation

- `callable_declaration_catalog/catalog.rs`: the top-level
  `FunctionDeclaration` branch now also mints the canonical
  `free_function` `rows_by_key` row (`validate_parameters`, arity
  overflow typed as `ArityOverflow{owner:"<top-level>",..}`,
  `DuplicateCanonicalKey` on collision) before recording the
  `SelectedTopLevel` source row — exact parity with the
  source-backed issuer's TopLevel mode.
- `callable_declaration_catalog/selected_source_inventory.rs`:
  `SelectedTopLevelFunctionKeyV1::new` widened to
  `pub(in crate::mir::builder)` for ingress test construction.
- `static_result_publication_ingress.rs`: `TopLevel` arm per the
  exact boundary; `ScriptRoot | InstanceConstructor |
  NestedBoxMethod` grouped foreign arm unchanged.
- `src/mir/builder/README.md` lineage paragraph updated.

## Pinned evidence

- `cargo test --lib static_result_publication_ingress` — 16/16:
  `top_level_lineage_admits_declared_caller_for_static_target`
  (Cataloged with `free_function` caller from the sealed row),
  `top_level_lineage_requires_declaration_catalog`,
  `top_level_lineage_rejects_unrowed_caller`,
  `top_level_lineage_declines_non_declaration_targets`,
  `top_level_lineage_stays_unavailable_without_ledger`.
- `cargo test --lib callable_declaration_catalog` — 22/22:
  `excludes_non_catalog_callable_surfaces` now pins `len()==1` (the
  parity `free_function` row) with constructor/record/sync surfaces
  still unrowed; `one_catalog_scan_owns_top_level_and_box_method_
  occurrences` asserts the `free_function("helper",1)` declaration.
- `cargo test --lib program_root_work_plan` — 6 pass / 4 fail; the
  4 reds are `cohort-missing` baseline debt (identical set fails at
  parent commit). The 3 tests newly broken by parity were repaired:
  `accepts_unique_*` (fixture `param_decls` now matches `params`),
  `rejects_same_name_and_arity_*` and
  `selected_top_level_functions_reject_duplicate_physical_projection`
  now assert the earlier seal-level `DuplicateCanonicalKey`
  rejection — the canonical catalog owns the collision before the
  physical-projection validator (which remains as defense-in-depth;
  `mir_call_d1b_selected_normal_duplicate_projection_guard.sh` ok).
- Sweeps: `source_call_target` 83, `callable_result_representation`
  95, `module_lifecycle_capture` 18, `normal_script_direct_static`
  30, `member_route`/`decls`/`raw_root`/`module_draft_collector`/
  `static_result_publication`/`result_publication_owner` all green;
  `normal_callable` 5 reds and `resolved_value_profile`/
  `expression_port`/`function_call_preflight`/`callable_parameter_
  contract` 1 red each are all in
  `tools/checks/manifests/cargo_lib_red_baseline.tests.txt`.
- Guards: `current_state_pointer_guard` ok,
  `mirbuilder_qualified_route_scope_guard` ok,
  `script_static_result_publication_ingress_guard` PASS,
  `mir_call_d1b_selected_normal_duplicate_projection_guard` ok.
- Line budget: all touched source files <= 730 lines.

## Negative/deletion proof

- No fallback: unrowed caller stays `ForeignLineage`; absent catalog
  stays `DeclarationCatalogUnavailable`; unresolved target stays
  `Unavailable` — no silent admission.
- `observe_all_calls` unchanged: it already iterates
  `declarations()`; `seal_statements` parity makes test/compat
  catalogs consistent with production row coverage. Production
  catalogs already carried `free_function` rows, so every
  `declarations()` consumer already tolerates the namespace.
- The physical-projection validator's duplicate arm is now
  defense-in-depth (seal rejects the canonical collision first);
  its tokens and the d1b guard remain pinned.
- No deletion-set membership: shared seams untouched.
- Gate-1 or overall MirBuilder completion: not claimed.

## Next selected row

`MIRBUILDER-GATE1-STATIC-RESULT-CALLER-COVERAGE-S1` —
`BirthConstructor` `rows_by_key` co-seal (constructor syntax loan
join in the source-backed issuer + `BoxDeclaration.constructors`
iteration parity in `seal_statements`) + `InstanceConstructor`
lineage arm (`published_birth_key` membership probe +
`StaticBoxMethod` target probe), atomic per D0 rule 2. Owns the
observed boxtorrent `foreign-lineage` terminal (`LayoutBox.class_*`
inside `HakoAllocHeap` field-initializer stores).
