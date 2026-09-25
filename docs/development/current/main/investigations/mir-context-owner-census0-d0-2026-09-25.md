# MIR-CONTEXT-OWNER-CENSUS0-D0 — mixed context field census

Parent: `mir-crate-split-prep-ssot.md` context decomposition order
(`MIR-CONTEXT-OWNER-CENSUS0-D0` -> `SPLIT0-S0` -> `SPLIT0-R0` ->
`MIR-CRATE-BOUNDARY-RECHECK0-P0`), reached via cleanup SSOT step 3 after
`MIR-TOPOLOGY-REBASE0-P0`/`MIR-ROOT-MODULE-SURFACE0-G0` landed.

## Scope boundary

```text
起点: MirBuilder state at HEAD
終点: per-field owner census (classification only — no moves)
includes: MirBuilder top-level fields (9), scope_context::ScopeContext
          (1), compilation_context::CompilationContext (23),
          function_lowering_state::FunctionLoweringStateV1 (17)
excludes: nested sub-context internals (VariableContext, TypeContext,
          BindingContext, MetadataContext, CoreContext fields) — their
          own owners; method-local temporaries
```

## Task

Enumerate every state field, writer, reader, lifetime, and publication
owner. Classification only — no source/lowering/runtime change.

## Manifest

`docs/development/current/main/design/fixtures/mir-context-owner-census-d0-v1.tsv`

Columns: `struct`, `field`, `type`, `role` (catalog|environment|options|
session|scratch|publication), `writers` (count + top writer),
`readers` (count), `lifetime` (module|function|invocation|process),
`disposition` (named owner or blocker).

## D0 landing — census complete

50 context fields classified in
`design/fixtures/mir-context-owner-census-d0-v1.tsv`:

- MirBuilder top-level: 9 invocation/session-or-options fields
  (`current_module`, `function_state`, `core_ctx`, `scope_ctx`,
  `metadata_ctx`, `comp_ctx`, `recursion_depth`, `root_is_app_mode`,
  `repl_mode`).
- ScopeContext: 1 function-lifetime field (`debug_scope_stack`,
  observation-only).
- CompilationContext: 23 fields — the mixed seam: 14 module-lifetime
  catalogs (`user_defined_boxes`, `brand_decls`, `user_box_field_decls`,
  `record_decls`, `record_field_defaults`, `enum_decls`,
  `callable_declaration_catalog`, `static_scalar_method_facts`,
  `weak_fields_by_box`, `property_registry`, `field_origin_by_box`,
  `using_import_boxes`, `method_tail_index`, `type_registry`), 3
  environment snapshots (`callable_main_compatibility_policy`,
  `emit_debug_policy`, `plugin_method_sigs`), 3 session fields,
  2 scratch, 1 options (`quiet_internal_logs`).
- FunctionLoweringStateV1: 17 function-lifetime fields — already the
  named sole owner of FunctionOwned lowering surfaces; no seam action.

The mixed field family is `CompilationContext`: module catalogs,
environment snapshots, and session/scratch state share one struct.
`MIR-CONTEXT-OWNER-SPLIT0-S0` may act only on a seam this census proves
behavior-neutral; the catalog block (14 fields, all module-lifetime,
written at module ingress, read through narrow accessors) is the
named candidate. No field was moved; classification only.

## SPLIT0-S0 landing — method tail index owner split

The smallest proven seam — the `method_tail_index` /
`method_tail_index_source_len` pair — was consolidated into one
`MethodTailIndexV1` catalog owned by `builder_method_index.rs` (the
module that already owns all rebuild/lookup logic):

- `src/mir/builder/builder_method_index.rs`: new `pub(crate) struct
  MethodTailIndexV1 { index, source_len }`.
- `src/mir/builder/compilation_context.rs`: two fields replaced by one
  `method_tail_index: MethodTailIndexV1`; accessors
  (`get_method_tail_candidates`, `maybe_rebuild_method_tail_index`,
  `add_method_tail_entry`, `clear_method_tail_index`) and the vacancy /
  preflight witnesses updated in place; public method surface unchanged.
- `src/mir/builder/compilation_context/tests.rs`: freshness witness
  reads `.source_len` through the new owner.
- Census fixture now 49 fields (the pair is one catalog row:
  `builder_method_index owner (split S0)`).

No caller signature changed; `module_lifecycle.rs` still uses the
unchanged `clear_method_tail_index` accessor. Verification:
`cargo check --profile quick --lib` and `--tests` green; focused
`method_tail` tests 3/3 ok. `reference_delta = 0` (no reference
contract touched).
