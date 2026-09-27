# MIRBUILDER-ADJACENT-MODULE-CONTRACT-PIN-RESYNC-S14

**Row**: `MIRBUILDER-ADJACENT-MODULE-CONTRACT-PIN-RESYNC-S14`
**BoxShape**: mostly test-only resync plus two small producer-side
coverage repairs split out of the census (same class as S12/S13).
Repin stale assertions to the honest terminals produced by
already-landed contracts.
**Mode**: `fast` — focused gates only.

## Census

Adjacent-module run after S12/S13 surfaced ~22 residual reds in the
`mir::builder`/`mir::compiler`/`resolved_semantics` lane (the broader
`tests::mir::*` inventory stays LEGACY-TESTS-RETIRE-R0 debt). Most
trace to a landed contract change whose test pin was never updated —
the same disposition class as S9/S11. Two live gaps were found during
the resync and fixed in this slice (see "Landed splits" below).

### Landed splits (producer-side, same class as S12/S13)

- **Grouped-assignment draft coverage**: `GroupedAssignmentExpr` no
  longer parses (admission narrowing), so direct-AST fixtures became
  the only reachability check — they exposed that
  `resolve_named_assignment` (`shadow/expr.rs`) recorded the
  `GroupedAssignmentTarget` site into the resolver inventory but never
  recorded the site itself nor its draft shape row. The resolver now
  records `record_expression_site` plus the
  `BindingAssignmentTarget` shape (the same vocabulary
  `record_assignment_target_shape` issues for `Variable` targets).
  `grouped_assignment_resolves_as_a_complete_script_rebind` green.
- **String corridor canonical-Call observation**:
  `infer_fact_from_instruction` (`string_corridor.rs`) read only
  `LegacyCallV0` plus the compat quarantine; typed `Call`
  (`Callee::Method`/`Callee::Global`/`Callee::Extern`) emitted by the
  canonical cutover was invisible, so corridor facts/candidates went
  empty on the typed lane. Canonical inference now reads the typed
  carrier structurally (`StaticBoxMethod { owner, method, arity }`,
  intrinsic `source_name()`, runtime exports via the existing
  quarantine) with the same carrier vocabulary. The two previously
  `#[ignore]`d corridor compile tests are un-ignored and green.

## Repin groups

### Group A — `runtime-box-fate` retired boundary (compat lane)

`7167aee18e` retired the raw runtime-box-fate lane for compatibility
boxes; fixtures entering through `compile_with_source`/`compile_normal`
on script/compat sources now hit the honest retired terminal
(`[freeze:contract][raw-compat/runtime-box-fate-retired/{static,instance}]`)
instead of producing the legacy artifact the pins expected.

- `callable_semantic_source` parity tests ×3 (`*_matches_legacy*`):
  repinned to `expect_err` + retired-terminal vocabulary.
- `normal_script_runtime_work` ×4 and `normal_script_record_literal`
  ×3: repinned expected diagnostics to the retired terminal.

### Group B — retired ordinary-new / env-selector races

- `legacy_candidate_session` parity + `numeric_contracts` ×3: the
  `new <Class>` birth-global lane is retired
  (`[freeze:contract][ordinary-new/birth-global-legacy-stopped]`);
  repinned to the honest boundary (typed-edge claim authority stays
  production-owned, no test mints it).
- Env-selector races: `NYASH_MIR_CORE13_PURE` (`mir_pure_envbox`
  writer now scoped via `with_env_var`; `numeric_contracts` readers
  pin unset), `NYASH_MIR_UNIFIED_CALL` (success-path compiles in
  `string_corridor`, `finish_schedule`, `basic_lowering`,
  `await_lowering`, `mir_peek_lower`, `binding_rebind` parity helpers,
  `external_destination_bridge` pin unset — default ON is the
  intended route; `"off"` writers already serialize via the shared
  mutex), `HAKO_MIR_BUILDER_METHODIZE` (`METHODIZE_SELECTOR_UNSET`
  preset in `test_support`; ingress/capture/lifecycle modules pin
  unset).
- `mir_pure_envbox` needle updated to the current printer vocabulary
  (`call_extern env.box.new`).

### Group C — narrow admission / terminal renames

- `binding_rebind` ×3: `9a55f1e0ca` (typed pure Script parser
  admission) deliberately narrowed the admitted Script statement
  vocabulary; `(x = …)` grouped syntax no longer parses. Fixtures
  construct `ASTNode::GroupedAssignmentExpr` directly so the rebind
  coverage is preserved (surfaced the draft-coverage split above).
- enum tests ×3: enum declaration handling lives in the selected
  (package-bearing) lane; the compat `compile_normal(for_mir_mode)`
  path declines `EnumDeclaration` per the ongoing compat retirement.
  Repinned to `expect_err` + unsupported-node terminal.
- `accepted_vocabulary_is_closed_and_reviewable`: `0ac2b93e52` added
  `"Program"` to `SHADOW_ACCEPTED_STATEMENTS_V0`; added to the pin.
- `forest_parent_rejects_unsupported_ancestry_and_orphan_scope`: same
  `IfThen`-supported-ancestry drift as S9 (`440c16216d`); fixture's
  unsupported ancestor moved to `MatchArm`.
- Bare statement-AST inputs (admission boundary): `compile`/
  `compile_normal` now require `ASTNode::Program` for the selected
  normal/default path; `basic_lowering`, `await_lowering`,
  `finish_schedule`, `string_corridor`, `mir_peek_lower`,
  `mir_pure_envbox` fixtures wrapped.
- `production_skip_while_*` ×2, `string_corridor_relation`
  benchmark ×2, `source_backed_loop_*` stack-depth flake, and the
  `rejecting_routes_precede_children`/`loopfacts_*`/`nested_box`/
  `field_receiver_provenance` family stay classified baseline debt
  (verified red on remote HEAD without this diff); they route to the
  owning lineages, not this batch.

## Pins

- Touched tests green under honest pins; corridor compile tests
  un-ignored and green.
- Focused batch `mir::builder::calls` / `mir::compiler::tests` /
  `mir::string_corridor` / `mir::resolved_semantics` /
  `normal_script*` / `normal_callable_semantic_source` /
  `binding_rebind` / `mir_peek_lower` / `mir_pure_envbox`:
  750 passed, 3 failed — all three confirmed baseline debt
  (loop-winner freeze on compat benchmark lane ×2,
  `rejecting_routes_precede_children` ×1).
- `binding_rebind` family 7/7 green including the
  grouped-assignment semantic-source test.

## Non-claims

- Does not change any contract, admission breadth, or seal invariant;
  the two production repairs only record already-required
  provenance/shape rows and observe the already-canonical call
  carrier.
- Does not reopen Gate-1, Gates 2-4, or the retired compat lane.
- Binding-rebind/enum narrowing stays as designed; this batch never
  re-broadens admission to satisfy a stale pin.
- `tests::mir::*` legacy inventory (~100 reds) remains
  LEGACY-TESTS-RETIRE-R0 debt, untouched.
