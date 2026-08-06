# Generic raw structured static-call result publication I1/R0

Status: `bounded issuer/terminal wiring implemented 2026-08-07; strict receipt gate pending`

The implementation row opens with the bounded issuer slice from the closed
source-bound handoff design.  See:

- `generic-static-call-publication-source-bound-handoff-design-stop-2026-08-07.md`

## Current slice: `GENERIC-STATIC-CALL-PUBLICATION-SOURCE-BOUND-ISSUER-S0`

Issue one AST-free, non-`Clone`, single-use handoff after CatalogInstall and
before callable lowering.  Consume it at the raw static-call terminal using
the exact located caller/site and the physical receipt.  This bounded
normal-lifecycle terminal consumer is now wired; GenericLoop selection and
the broader production caller remain closed.

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

## Done so far

- The move-only owner issues the exact source/site row once and rejects
  foreign/duplicate consumption.
- Focused publication and lifecycle/raw-terminal tests pass; source/check
  files remain below 800 lines.
- Call-argument source scoping now preserves the exact BindingRef site while
  the terminal consumes the handoff.

The fresh strict VM receipt and any broader Generic production caller switch
are still pending and remain outside this bounded commit.

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
