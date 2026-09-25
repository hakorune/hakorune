# MIR-CRATE-BOUNDARY-RECHECK0-P0 — packaging recheck after context split

Parent: `mir-crate-split-prep-ssot.md` context decomposition order
(`CENSUS0-D0` -> `SPLIT0-S0` -> `SPLIT0-R0` ->
`MIR-CRATE-BOUNDARY-RECHECK0-P0`), cleanup SSOT step 3 tail.

## Scope boundary

```text
起点: builder mutable context at HEAD (post SPLIT0-S0/R0)
終点: crate-packaging recheck decision (reconsider only — no moves)
includes: CompilationContext residual fields (22 census rows),
          MirBuilder top-level (9), ScopeContext (1),
          FunctionLoweringStateV1 (17)
excludes: nested sub-context internals; any packaging move itself
```

## Task

"Reconsider packaging only after the mixed mutable context is gone."
Decide whether any builder packaging slice is now safe, or record the
named blocker and `removal_when` that keeps it parked.

## Decision — packaging remains parked

The mixed mutable context is **not** gone. `CompilationContext` still
holds 22 census rows across four families:

- 14 catalog fields (module-lifetime): `user_defined_boxes`,
  `brand_decls`, `user_box_field_decls`, `record_decls`,
  `record_field_defaults`, `enum_decls`, `callable_declaration_catalog`,
  `static_scalar_method_facts`, `weak_fields_by_box`,
  `property_registry`, `field_origin_by_box`, `using_import_boxes`,
  `method_tail_index` (now a `MethodTailIndexV1` owned by
  `builder_method_index`, but still stored on the shared struct),
  `type_registry`.
- 3 environment snapshots (`callable_main_compatibility_policy`,
  `emit_debug_policy`, `plugin_method_sigs`), 3 session fields
  (`compilation_context`, `current_static_box`,
  `current_slot_registry`), 1 scratch (`field_origin_class`),
  1 options (`quiet_internal_logs`).

The named SSOT blockers are still live inside that mix:

- `current_slot_registry: Option<FunctionSlotRegistry>` — session field,
  14 writers; FunctionSlotRegistry ownership not separated.
- `type_registry: TypeRegistry` — catalog field, 11 readers;
  TypeRegistry ownership not separated.
- `callable_declaration_catalog` — catalog field carrying declaration
  AST-adjacent state; ASTNode ownership seam unresolved.

## Named blocker + removal_when

```text
Blocker: CompilationContext mixed catalog/environment/session/scratch
         ownership — packaging would widen visibility and move shared
         authority without changing it (violates move-only / one-owner
         rules).
removal_when: the catalog block (14 rows) is extracted behind a named
              owner type (candidate: SourceCatalog / CallableCatalog /
              TypeEnvironment per split-prep SSOT) AND
              FunctionSlotRegistry / TypeRegistry ownership is
              separated; then MIR-CRATE-BOUNDARY-RECHECK0-P0 reopens.
```

## Evidence

- Census fixture:
  `design/fixtures/mir-context-owner-census-d0-v1.tsv` (49 rows).
- SPLIT0-S0/R0 landed (`94152b06e9`, `5e103b77ff`): one seam proven
  and guarded; remaining seam stays behind the census.
- No source changed; classification/decision only.
  `reference_delta = 0`.

## Next

`MIR-AUTHORITY-ROLE-MANIFEST0-D0` and
`MIR-LEGACY-JOINMODULE-DISPOSITION0-D0` (parked cells), ahead of
`REPO-FINAL-CONVERGENCE-AUDIT0-G0`. Packaging reopens only via the
`removal_when` above.
