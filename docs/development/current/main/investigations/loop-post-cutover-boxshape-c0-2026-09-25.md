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
