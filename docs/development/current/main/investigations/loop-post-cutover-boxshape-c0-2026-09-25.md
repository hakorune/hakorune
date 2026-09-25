# LOOP-POST-CUTOVER-BOXSHAPE-C0 — post-M12 module census

Parent: `joinir-loop-selfhost-recipe-pipeline-ssot.md` post-M12 physical
cleanup handoff. Ordered after M12 (`R2A->R2B->R2C->R2G` landed through
`b50e8b9a56`; baseline re-pinned `e53c0693c5`).

Entry condition (SSOT): sole Loop authority is proven — one recipe
issuer (`route_entry/router.rs` -> `loop_node_winner_spine`), one
physical admission (`loop_node_physical_admission`), one producer call
site per selected family, `LoopRouteId` reduced to data-only
vocabulary, Facts hold no Recipe keys.

## Scope boundary

```text
起点: post-M12 tree (e53c0693c5)
終点: Loop-owned modules in src/mir/** and lang/src/mir/builder/loop_recipe/
includes: loop_recipe_contract, loop_route_policy, loop_structural_facts,
          loop_route_detection, loop_canonicalizer, loop_api/loop_form,
          compiler loop-family artifacts (winner spine, physical admission,
          per-family profile/projection/producer/observation), callable
          loop seam (normal_callable_loop_*, raw_loop_*), joinir
          route_entry + loop_context/routing, plan loop features +
          recipe_tree, resolved_lowering loop adapters, hako wire emitters
excludes: callable_result_representation claims, non-Loop mir surfaces,
          generic callable/package machinery beyond the loop seam,
          backend/provider code
```

## Classification axes

Per SSOT: each Loop module is classified exactly once as
`durable-production` | `bootstrap-compatibility` | `test-oracle` |
`retire-candidate`, with the production caller / consumer evidence named
per row. A `retire-candidate` row requires caller-zero or
equivalent-evidence; the manifest only proposes — deletion is the later
2-5 commit BoxShape series.

## Manifest

`docs/development/current/main/design/fixtures/loop-post-cutover-module-census-c0-v1.tsv`

## Done

- Every in-scope Loop module appears exactly once with a class and named
  caller evidence.
- Retire candidates cite caller-zero proof; compatibility rows cite the
  boundary that still consumes them.
- No file moved or deleted in this row.
- Card + CURRENT_STATE synchronized; pointer guard green.

## C0 landing — census complete

605 in-scope rows classified in
`design/fixtures/loop-post-cutover-module-census-c0-v1.tsv`:

- durable-production: 457 — every non-test module has a named production
  caller (winner spine / router / callable seam / canonicalizer / plan
  machinery / s6c cohort via `normal_callable_semantic_package`,
  `loop_recipe_physicalizer` via `resolved_lowering/mod.rs`).
- test-oracle: 134 — cfg(test) modules, `*_tests.rs`, wire parity suite,
  `#[path]`-included test helpers (`callable_single_loop_recipe_shape`,
  `generic_resolved_carrier_facts_snapshot`, `loop_tests_parts/*`).
- bootstrap-compatibility: 7 — `lang/src/mir/builder/loop_recipe/*.hako`
  emitters (frozen portable wire surface; exercised by wire parity tests
  and `hako_module.toml`, no Rust production caller) plus
  `loop_recipe_contract/s6c_scan_with_init_logical_consumer.rs`
  (facade re-export only, prepared typed consumer pending wiring).
- retire-candidate: 7 — `control_flow/facts/canon/generic_loop/**`
  (7 files: condition/step/types/update + step/extract +
  step/placement + step/placement/matcher). Orphan files: no
  `mod generic_loop` declaration in `facts/canon.rs`, no `#[path]`
  include — stale forwarding shims superseded by
  `control_flow/generic_loop_canon` (mod decl dropped at `8513de549e`
  "migrate generic loop canon imports"). Not compiled; earlier
  "production caller" hits were `expr_generic_loop::` substring
  false-positives. Stale `canon.rs` doc line claiming Facts owns
  `generic_loop` is cleanup for the retire slice.

Move/delete manifest: delete list = the 7 orphan rows above (bounded,
single-cluster retirement). The later BoxShape split series may still
regroup durable modules into neutral contract / source producers /
route policy / physical facades, but that is a file-move series only —
this census names no other semantic removal.

Scope reconciliation (corrected during census):

- `#[path]`-renamed children (`normal_callable_loop_source_route_items`,
  `normal_callable_loop_handoff_*`, `recursive_child_lowering_loop_true`)
  resolved through parent declarations — initially flagged caller-zero.
- `loop_recipe_physicalizer` is live durable infrastructure via
  `resolved_lowering/mod.rs` (`lower_generic_g0_function_draft_v1`,
  `lower_generic_g0_function_draft_pending_v1`) — module-tree liveness,
  not stem search.
- S6C cohort re-included: `s6c_child.rs` / `loop_break_source.rs` /
  `s6c_effects.rs` / `s6c_storage_header.rs` are loop-family production
  homes in `normal_callable_semantic_package`.
- `selected_dynamic_physical_emitter` / canonical SSA modules excluded:
  shared dynamic emitter infrastructure, not Loop-owned.
- Stale `// Caller-zero` comments in `loop_recipe_contract` audited
  one-by-one — all but `s6c_scan_with_init_logical_consumer` have live
  external production callers; comments describe a narrower physical
  effect, not module liveness.
- Directory rows expanded to all descendants: basename-scoped find
  missed nested children (`loop_break/facts/**` 24 files,
  `recipe_tree/**` 21, `generic_loop_canon/**` 17, `loop_cond/**` 15,
  `loop_tests_parts` 5, others) — all re-added and classified.

Verify: per-file caller sweep over `src/`, `lang/src/`, `tools/`
excluding test-named callers; `mod.rs` rows resolved through parent
`mod` declarations; `#[path]` inclusions resolved to parent modules;
cfg(test)-gated modules checked individually.

## Orphan retire landing

The 7 retire-candidate rows (`control_flow/facts/canon/generic_loop/**`)
were deleted as the manifest's delete list — pure re-export shims, never
compiled (no `mod` declaration, no `#[path]` include). Stale `canon.rs`
doc line corrected in the same commit. `cargo check --profile quick
--lib` green.
