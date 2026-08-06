# Generic raw structured static-call result publication I1/R0

Status: `issuer slice active after design close 2026-08-07; production caller remains closed`

The implementation row opens with the bounded issuer slice from the closed
source-bound handoff design.  See:

- `generic-static-call-publication-source-bound-handoff-design-stop-2026-08-07.md`

## Current slice: `GENERIC-STATIC-CALL-PUBLICATION-SOURCE-BOUND-ISSUER-S0`

Issue one AST-free, non-`Clone`, single-use handoff after CatalogSeal and
before CatalogInstall.  Add the neutral activation-source seam required by
the existing located schedule/ledger/batch products.  Do not switch the
production caller in this slice; that remains the next atomic row.

Parent design:

- `generic-raw-structured-static-call-result-publication-i1-d0-activation-task-2026-08-07.md`

## Change

Wire exactly one sealed `StringHelpers.int_to_str/1` source/site demand to the
current-owner static call terminal and its `CompletedUnifiedValueCallEmissionV1`
receipt.  Consume the receipt once to publish the result type, then let the
existing local materializer feed GenericLoop verification.  Remove the
post-effect retry/debt continuation for this selected activation.

## Contract

- The outer module candidate session is the sole rollback owner.
- `with_saved_variable_map_typed` remains a local binding helper only; it does
  not claim physical rollback.
- No alternate terminal, name/arity selector, inferred type, AST reread,
  GenericLoop type backfill, retry, or fallback.
- Failed physical emission, publication mismatch/duplicate, or any effect-after
  failure returns terminal `Freeze`; the candidate is discarded by the outer
  session.

## Done

- Exact success and failure fixtures prove no partial Call/type/cleanup state
  escapes a discarded candidate.
- Focused publication and route-terminal tests pass; source/check files remain
  below 800 lines.
- Fresh strict VM probe crosses `MissingTransientType` and preserves the exact
  caller/site/target/destination relation.
- `CURRENT_STATE.toml`, the active workstream, and
  `docs/reference/mir/generic-loop-stage-matrix.md` are updated in this same
  implementation commit.

## Stop

Return to design if the selected source cannot be sealed before effects, if a
second rollback owner is needed, if the receipt cannot be obtained from the
current terminal, or if the route still needs retry/fallback to pass.

### Design-stop evidence

The audit found that the live `RawInvocationChildPortV1` loop path discards
the located source receipts before entering `route_generic_loop_v1`.  The
activation plan, caller ledger, and exact site claim therefore have no
production owner.  `compose_located_generic_loop_v1` and
`CorePlanEffectEmissionPortV1::Claimed` are test-only.  Do not add a by-name
selector or directly wire that disconnected path; the next row is the compact
source-bound handoff design above.
