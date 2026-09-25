# M12-R2A — `JOINIR-LOOP-LEGACY-DISPOSITION-R2A` record

Parent row: `JOINIR-LOOP-LEGACY-FAMILY-ADAPTER-RETIRE0-R2` (M12), ordered
`R2A -> R2B -> R2C -> R2G` per
`docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md`.

This card records the R2A disposition classification only. No code moved in
this commit.

## Scope boundary

```text
起点: post-M11-B tree (8dff99ef22)
終点: src/mir/loop_route_policy/, src/mir/loop_recipe_contract/,
      src/mir/compiler/ loop-family artifacts,
      src/mir/builder/control_flow/plan/{recipe_tree,features,normalizer},
      src/mir/loop_structural_facts/ residual/break facts
includes: first-mutation profile artifacts, family receipts, route wrappers,
          producer files, oracle/parity test surface
excludes: callable_result_representation claims, non-Loop mir surfaces,
          loop_route_detection support internals (live via router/return_stmt)
```

## Manifest

`docs/development/current/main/design/fixtures/loop-legacy-family-disposition-r2a-v1.tsv`

- Section A (10 rows): caller-zero migration rows — the sealed
  `generic_residual` chain (demand/producer/projection/typed-map/facts/id) and
  the `variable_accum_break` cohort (producer/projection/facts/id). The
  residual chain is `duplicate-facade`-class retire in R2B; the
  variable-accum-break cohort is `decision-required` because
  `ATTESTED_RECIPE_BACKED_V1` still claims it covers `LoopBreakRecipe` while
  production LoopBreak lowers via `loop_break_source` — the attestation/owner
  mismatch needs a design decision, not a silent delete.
- Section B (9 rows): the oracle-only route-local physical/PHI wrapper chain —
  `RecipeComposer::compose_*` (12 composer files) is reachable only through
  `loop_accum_legacy_oracle_support.rs`, itself `#[path]`-included only from
  the `#[cfg(test)]` semantic-parity test module; the four non-source
  `*_pipeline.rs` families and their `*_phi_materializer`/`*_verifier`/
  `*_cleanup`/`*_join` companions hang off that oracle. `wire_*_parity_tests`,
  `producer_id_migration_tests`, and the `normalizer` dead delegators are
  migration evidence / duplicate facades retiring in R2B with the red-baseline
  receipt absorbing the removed test names.
- Section C (5 rows): decision-required — the synthetic 19-row
  admission/cursor schedule is still executed in production
  (`family_route_schedule` -> `freeze_loop_route_schedule_v1` at
  `loop_node_winner_spine.rs:610`, cursor-verified inside each family demand
  issuer); `generic/issuer.rs` `matched_routes()` singleton selection is the
  last `LoopRouteId` behavior-selection site; `direct_accum_effect_plan.rs`
  mints `LoopBindingKeyV1` inside the Facts layer; `LoopRouteId` retention as
  data-only diagnostic; and the honest record that family-name dispatch moved
  to `CanonicalLoopFamily*`/`CanonicalFirstFamilyPlanV1` rather than reaching
  zero.
- Section D (11 rows): retained semantic input — family
  observation/admission/selection machinery, live producers, the neutral
  contract core, the membership oracle (`single_planner`/`RecipeMatcher`/
  builders), route-local source lowerers, shared features helpers, the
  DirectAccum physical lane, and the winner-spine chain.
- Recorded contradictions: stale `caller-zero` docs on `family_selector.rs`
  and the winner spine; `ATTESTED_RECIPE_BACKED_V1` attests residual/break
  cohorts as route coverage while production owners differ.

## Contract notes

- `ComposerMutationFamily`-equivalent legacy first-mutation enums: zero
  production references. The only textual `mutation_family` mention is test
  code (`loop_recipe_contract/tests.rs:643`).
- `LoopRouteId` never drives physical lowering; remaining uses are frozen
  provenance, attestation tables, cursor guards, the synthetic schedule, and
  the one `issuer.rs` selection site (C02).
- Nothing in this manifest is a deletion order by itself: R2B/R2C execute the
  retire_row assignments; C-rows resolve before their delete.

## Evidence

- Census greps recorded per-row in the manifest `evidence` column.
- `cargo check --profile quick --lib` and `--tests` green at the manifest base
  commit (`8dff99ef22`); this commit changes no code.
- Guards green at base: `mirbuilder_inplace_replacement_guard.sh`,
  `coreplan_varmap_boundary_inventory_guard.sh`,
  `current_state_pointer_guard.sh`, generic corpus front receipt
  (398/4/270).

## M12-R2B partial landing (residual chain)

Section A rows A01-A06 landed: the sealed `generic_residual` chain
(`loop_route_policy/generic_residual.rs`,
`loop_recipe_contract/generic_residual_producer.rs`,
`compiler/generic_residual_projection.rs`,
`compiler/generic_residual_typed_map{,_issue}.rs`,
`loop_structural_facts/generic_residual_source.rs`,
`LoopRecipeProducerIdV1::GenericResidualV1`) is deleted with its tests,
the `ATTESTED_RECIPE_BACKED_V1` `GenericLoopV1` row, the M8E wire
emitter (`emit_m8e_generic_wire.hako` + module registration + fixture
`hako_loop_recipe_wire_m8e_v1.json` + `m8e_tests`), the residual probe
rows, and the stale allowlist pins in `joinir_logical_demand_contract.sh`.
`GenericLoopV1` stays a `legacy_only`/typed-declined census row;
wire-backed attestation is now 7 routes / 12 declines. The red-baseline
receipt absorbs 25 removed unit tests (expected_passed 7369->7344; the
removed names were already absent from the stale receipt inventory).
`cargo check --profile quick --lib`/`--tests` green; focused
route/parity/family/probe tests 29/29 green; in-place replacement guard
green.

## Next

`JOINIR-LOOP-DUPLICATE-FACADE-RETIRE-R2B` (continued): execute the
Section B oracle chain deletions (`RecipeComposer::compose_*` +
`loop_accum_legacy_oracle_support.rs` + non-source `*_pipeline.rs`
families and their `*_phi_materializer`/`*_verifier`/`*_cleanup`/`*_join`
companions — proven reachable only through the `#[cfg(test)]`
semantic-parity module), re-point the red-baseline receipt for every
removed test file, and update the varmap / in-place guards if any pinned
site dies. Section C decision rows (19-row schedule disposition, issuer
route-vector selection, DirectAccum facts-layer key minting) resolve
before their delete.
