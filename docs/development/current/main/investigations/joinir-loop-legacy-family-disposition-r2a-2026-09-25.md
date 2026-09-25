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

`JOINIR-LOOP-DUPLICATE-FACADE-RETIRE-R2B` (continued): Section B
oracle chain landed. Deleted: all 11 `recipe_tree/*_composer.rs`
duplicate facades + the `RecipeComposer` struct, the
`loop_accum_legacy_oracle_support.rs` oracle + its
`accum_semantic_parity_tests` parent (semantic/physical parity tests,
resolved-snapshot tests, both digest supports, binding-SSA candidate
tests), the `features/loop_cond_co_*` continue-only pipeline cluster
(9 files), `loop_cond_continue_with_return_*` cluster (5 files +
README dir), `loop_cond_return_in_body_*` cluster (5 files),
`loop_true_break_continue_pipeline.rs`, the `loop_cond_bc.rs`
`lower_loop_cond_break_continue` oracle entry + dead helpers, and the
two `PlanNormalizer::normalize_loop_cond_*` dead delegators.
Retained-rehome: `loop_accum_physical_role_plan_tests.rs`,
`loop_accum_binding_ssa_{session,operation,emitter,failure}_tests.rs`,
and `loop_recipe_producer_facade_tests.rs` now hang directly under
`loop_phi_materializer::tests`. Retained live helpers confirmed:
`loop_true_break_continue_{cleanup,phi_materializer,verifier,source}`,
`loop_cond_bc_*` (source uses them), `PlanNormalizer` struct.
Red-baseline receipt: 30 removed test names deleted from inventory, 9
re-homed names re-pathed, expected_passed 7344->7314, inventory sha
refreshed (receipt remains stale vs observed = known baseline debt).
`joinir_logical_demand_contract.sh` re-pinned (deleted-file allowlist
vars + cfg(test)/digest/candidate checks removed). `REGISTRY.md` and
`features/README.md` re-pointed to the source-backed lane;
`recipe-tree-and-parts-ssot.md` primary rule updated. Dormant stale
pins recorded-not-touched: `coreplan_active_v0_inventory_guard.sh`
(COMPOSER file pin), `callable_result_i0_*loop0*.py` family,
`mir_verification_quick_p0_c_guard.py`, `rust_lifecycle_*` guards,
`tools/rust_lifecycle/*.py` inventories, phase-296x cards, rust-lifecycle
fixture JSONs. Verify: `cargo check --profile quick --lib`/`--tests`
green; `loop_phi_materializer` focused 26/26; wider
normalizer/recipe_tree/loop_cond/route-policy/wire-parity run 268/270
with both failures already in `failures.txt` (known-red);
in-place replacement guard + pointer guard green.

## Next

`JOINIR-LOOP-MUTATION-DISPATCH-RETIRE-R2C`: Section C decision rows —
synthetic 19-row `family_route_schedule`/`freeze_loop_route_schedule_v1`
production schedule, `generic/issuer.rs` `matched_routes()` last
LoopRouteId behavior-selection site, `direct_accum_effect_plan.rs`
Facts-layer `LoopBindingKeyV1` minting, VariableAccumBreak
production-owner vs attestation mismatch. Resolve dispositions before
deletion.

## M12-R2C landings (Facts key mint + schedule/winner retirement)

R2C part1 (`ce91387634`): `VerifiedDirectAccumBindingEffectPlanV1` no
longer mints `LoopBindingKeyV1`; Facts entries carry only
role/site/source-binding. `CanonicalDirectAccumBindingPort` now consumes
owned `VerifiedLoopRecipeBindingRelationV1` rows and resolves keys
solely through them — Recipe stays the only key issuer.
`direct_accum_physical_input_with_relations` is the single input
assembly; the old wrapper/`entries()`/`from_direct_accum` were removed.
Focused run 46/46 green.

R2C part2 (this commit): the synthetic 19-row
`family_route_schedule`/`freeze_loop_route_schedule_v1` +
`VerifiedLoopPolicyWinnerV1` ceremony retired. `evaluate.rs`,
`adapter.rs`, `tests.rs`, `loop_true_break_continue_tests.rs`, and the
test-only `family_selection.rs` marker deleted. `policy.rs` now issues
`VerifiedDirectAccumPolicyHandoffV1` directly from
`VerifiedDirectAccumSingletonObservationV1` — no route id, raw cursor,
or winner crosses the boundary. LoopTrue/LoopCond demands seal directly
from their source projection/typed map (infallible issuers).
`issue_selected_loop_recipe_demand_v1` consumes the admission and
returns `(demand, policy_receipt)`; the spine drops the receipt while
`direct_accum_profile` retains it as provenance. `schema.rs` retains
only `CANONICAL_LOOP_ROUTE_*` vocabulary + `LoopRouteSourceUnavailableV1`
for `all_route_observation`; `policy_evidence.rs` retains only
`LoopRoutePolicySourceDeclineReasonV1`. The winner-cursor/frame checks
were tautological over the synthetic schedule (hidden synthetic
authority), so their removal preserves behavior.
`joinir_logical_demand_contract.sh` re-pinned: deleted-file require
list, `mod adapter` check, freeze/evaluate caller checks replaced with
a retired-name absence check, DirectAccum issuer call count 2->1
(deleted test-only `admit_direct_accum_profile_v1`). Receipt: 24 stale
names removed, `policy_evidence` test renamed
(`decline_vocabulary_is_closed`), expected_passed 7314->7291,
inventory/failures sha refreshed. Verify: `cargo check --lib`/`--tests`
green; focused route-policy/facts/direct-accum/producer run 286 green,
1 failure (`qualified_call_map_argument_reaches_the_named_capability_boundary`)
is pre-existing baseline red also present in the HEAD full-run receipt.
Known unrelated reds: `naming_charter_guard` (env rg regex parse +
Stage-A wording at HEAD), `mir_root_facade_guard` (`ConstructionTarget`
allowlist drift at HEAD) — both untouched by this slice.
