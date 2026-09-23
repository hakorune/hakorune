---
Status: landed__2026-09-23__producer_cohort_caller_zero
Task: JOINIR-LOOP-M8-LOOPCOND-EXITS-S6D
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-LOOPV0-SCANS-S6C (closed)
PreviousCard: generic-legacy-cross-family-dependency-s0-2026-09-23.md
NextCard: same-row__bounded_producer_cohort_implementation
Implementation permission: false; name the owners to extend and fix the
bounded implementation slice only. No code, no manifest write, no
route/caller change, no new semantic receipt from this card.
---

# JOINIR-LOOP-M8-LOOPCOND-EXITS-S6D — D0 producer-cohort design

## Six-line brief

```text
Decision: one caller-zero LoopCond producer cohort: a policy demand over the existing source projection, one portable Recipe producer, and the row's typed source-policy observation — no new selector/verifier/CFG/PHI/physicalizer.
Source authority + canonical issuer: `loop_route_policy::issue_loop_cond_break_continue_policy_demand_v1` is the sole policy-demand issuer over `VerifiedLoopCondBreakContinueSourceProjectionV1`; `loop_recipe_contract::produce_loop_cond_break_continue_recipe_v1` is the sole Recipe issuer for this profile.
Non-authority: AST/name lookup, family re-selection, `family_selection` dispatch, physical IDs, Item key reissue, retry/fallback, legacy generic composer reuse, production callers.
Fail-fast boundary: exact policy frame key match against the source projection; missing predicate/effect mapping or unmatched projection is a typed producer reject, not Option/skip.
Smallest next slice: `VerifiedLoopCondBreakContinueTypedSourceMapV1` (projection + role-keyed predicate rows, ledger-joined) + `VerifiedLoopCondBreakContinuePolicyDemandV1`/`ReceiptV1` + `LoopRecipeProducerIdV1::LoopCondBreakContinueV1` + `loop_cond_break_continue_producer.rs` emitting `VerifiedLoopRecipeV1` + `VerifiedLoopJoinSigV1` + policy receipt, caller-zero.
Non-claims: no production selection, no `route_loop` connection, no S6E/S6G work, no Row F unblock claim, no legacy deletion, no caller-zero-retirement claim.
```

## Row contract

From `generic-loop-source-to-portable-recipe-ssot.md` (coverage table,
`JOINIR-LOOP-M8-*-S6A..S6E`):

> one producer cohort per stable row: LoopV0 recurrence; exits/joins; scans;
> LoopCond exits; Generic residual — each row emits its common Recipe/core
> product and typed source-policy observation; no new
> selector/verifier/CFG/PHI/physicalizer.

Ordered ladder (`joinir-loop-selfhost-recipe-pipeline-ssot.md`):

```text
S6A (closed) -> S6B (closed) -> S6C (closed) -> S6D <- this row
  -> S6E -> S6G -> S7A..S7G -> M8/M9 selection design -> M10b -> R1/M11/M12
```

The recorded S6D D0 result: the source projection lacks typed
predicate/effect and portable co-seal mapping; requiring an already-connected
issuer was circular, so this row names owners to extend while
connection/tests/caller-zero remain migration outputs.

## Census (what exists vs what is missing)

Existing, reusable authority:

- `src/mir/compiler/loop_cond_break_continue_projection.rs` (262 lines):
  `issue_loop_cond_break_continue_source_projection_v1` and the forest
  projection — the AST-free source projection this cohort consumes.
- `src/mir/loop_route_policy/loop_cond_break_continue_observation.rs`:
  `issue_loop_cond_family_observation_v1`, `LoopCondFamilyObservationV1`,
  `VerifiedLoopCondBreakContinueSourceProjectionV1` — the S1-scoped family
  observation (unchanged; this row does not re-observe).
- `src/mir/loop_recipe_contract/common_v2_condition_producer.rs` (192),
  `common_v2_predicate_branch_plan.rs` (250): the common-V2 predicate /
  condition products the typed predicate/effect mapping reuses.
- `src/mir/loop_recipe_contract/source_bound_core.rs`: the V1→V2 source-bound
  co-seal seam.
- `src/mir/loop_recipe_contract/loop_true_break_continue_producer.rs` (233):
  the exact producer pattern — `VerifiedPolicyDemand` → frame-key match →
  portable `LoopRecipeV1` → `LoopRecipeVerifierV1` → `LoopJoinSigElaboratorV1`
  → product (policy receipt + recipe + join sig).
- `LoopRecipeProducerIdV1` (`producer_id.rs`): currently
  `LoopTrueBreakContinueV1` only.
- `e671cf390e` bounded if-continue transfer layout evidence
  (`join_sig/transfer_view_v1.rs` + `physical_layout.rs`): same-loop
  If/Continue transfer is already physicalizable downstream; this row does
  not touch physicalization.

Missing (the row's deliverable):

- `VerifiedLoopCondBreakContinueTypedSourceMapV1` in `compiler/` —
  the typed predicate/effect + portable co-seal mapping the D0 named
  missing: the sealed projection plus role-keyed rows for the carrier
  declaration and both condition compare triples (lhs binding read,
  `SyntaxBinaryOperatorV1`, rhs literal shape), verified against the
  resolver ledger by the same syntax-observer input that issued the
  projection. Mirrors `main0_derived_predicate_source_map{,_issue}.rs`.
- `VerifiedLoopCondBreakContinuePolicyDemandV1` /
  `VerifiedLoopCondBreakContinuePolicyReceiptV1` in `loop_route_policy`
  — the typed predicate/effect policy demand over the typed map.
- `LoopRecipeProducerIdV1::LoopCondBreakContinueV1`.
- `src/mir/loop_recipe_contract/loop_cond_break_continue_producer.rs`:
  `produce_loop_cond_break_continue_recipe_v1` →
  `VerifiedLoopCondBreakContinueRecipeProductV1`
  (policy receipt + `VerifiedLoopRecipeV1` + `VerifiedLoopJoinSigV1`),
  mirroring the LoopTrue producer exactly; caller-zero.

## Owners to extend (bounded implementation slice)

| File | Change | Owner boundary |
| --- | --- | --- |
| `compiler/loop_cond_break_continue_typed_map.rs` (new) | role-keyed row schema + `VerifiedLoopCondBreakContinueTypedSourceMapV1` | rows only: carrier decl, loop/branch compare triples, exit sites; no Recipe/route/physical IDs |
| `compiler/loop_cond_break_continue_typed_map_issue.rs` (new) | `issue_loop_cond_break_continue_typed_source_map_v1(input, projection)` | sole sealer; syntax observation via `input.source()`, binding/exit verification via resolver records; typed rejects only |
| `loop_route_policy/loop_cond_break_continue.rs` (new) | `issue_loop_cond_break_continue_policy_demand_v1(typed_map, schedule)` + demand/receipt | mirrors `loop_true_break_continue.rs`; frozen-schedule evaluation + winner cursor == `LoopCondBreakContinue` position; no re-observation |
| `loop_recipe_contract/producer_id.rs` | add `LoopCondBreakContinueV1` | one new variant + id string |
| `loop_recipe_contract/loop_cond_break_continue_producer.rs` (new, ≤800 lines) | `produce_loop_cond_break_continue_recipe_v1` + product type | consumes demand once; frame-key match; predicate block (carrier read + const + CompareI64) + body If{break,continue} + one carrier; verified Recipe + JoinSig + receipt; typed rejects only |
| `loop_recipe_contract/mod.rs`, `loop_route_policy/mod.rs`, `compiler/mod.rs` (or registry) | module/export wiring | re-export only |
| focused tests beside each new file | positive product shape + typed reject paths | caller-zero evidence only |

The bounded fixture shape (from
`loop_cond_break_continue_projection_tests::positive_function`):
`local flag = 1; loop(flag < 2) { if (flag == 1) { break } else { continue } }`.
The bounded slice accepts only `Variable <op> Integer` compare
conditions with op in `{Less, LessEqual, Equal}` (the
`LoopCompareI64OpV1` vocabulary) on the same declared binding; any
other operand/operator/declaration shape is a typed map reject — no
widening in this row.

## Fail-fast boundary

- Demand issuance requires the exact LoopCond source projection and its
  typed predicate/effect mapping; anything else is a typed
  `PolicyDemandReject`, never a fallback to observation re-run.
- Producer requires `policy_receipt.frame_key() ==
  projection.root_frame_key()`; mismatch is
  `PolicyFrameMismatch` — same contract as the LoopTrue producer.
- Recipe/JoinSig failures propagate as typed producer rejects; no
  `Option`, no retry, no silent skip.

## Non-claims (stop conditions)

- No production caller, no `route_loop`/registry/selection wiring — this
  row's product stays caller-zero.
- No new selector, verifier, CFG, PHI, or physicalizer authority.
- No S6E Generic residual work, no S6G closeout claim, no Row F
  (`LOOP-PRODUCTION-SELECTION-D0`) unblock claim — Row F remains gated on
  the M10 seal series and M8 all-route coverage.
- No legacy Generic deletion or cutover; `GENERIC-LEGACY-DEAD-CODE-R1`
  stays blocked on M10b.
- No production-switch or caller-zero-retirement claim from green tests.

## Exit

When this card is accepted, `work_mode` moves to `fast` for the bounded
implementation slice named above (one producer cohort, one commit family).

## Landed (2026-09-23)

- `compiler/loop_cond_break_continue_typed_map{,_issue}.rs`:
  `VerifiedLoopCondBreakContinueTypedSourceMapV1` co-seals the sealed
  projection with the carrier declaration and both typed compare triples
  (`Variable <op> Integer`, op in `{Less, LessEqual, Equal}` on the same
  declared binding), ledger-verified; residual variable-ref and
  residual-exit checks keep the bounded boundary.
- `loop_route_policy/loop_cond_break_continue.rs`:
  `issue_loop_cond_break_continue_policy_demand_v1` over the typed map +
  frozen schedule; winner cursor must be the `LoopCondBreakContinue`
  position in `CANONICAL_LOOP_ROUTE_ORDER_V1`.
- `loop_recipe_contract/producer_id.rs`: `LoopCondBreakContinueV1`.
- `loop_recipe_contract/loop_cond_break_continue_producer.rs`:
  `produce_loop_cond_break_continue_recipe_v1` emits a predicate-loop
  portable Recipe (condition block: carrier read + const + CompareI64;
  body block: branch compare + If{break-exit, continue-exit}), verifies
  it, binds the root source claim, and elaborates the shared JoinSig —
  caller-zero, no selector/verifier/CFG/PHI/physicalizer authority added.
- Evidence: `cargo test --features vm-reference` —
  `loop_cond_break_continue` 20/20 (4 new producer tests incl. typed
  reject paths), `producer` 42/42, `loop_true` 48/48, `main0` 63/63.
- Non-claims hold: no production caller, no `route_loop` connection,
  S6E/S6G and Row F unaffected.
