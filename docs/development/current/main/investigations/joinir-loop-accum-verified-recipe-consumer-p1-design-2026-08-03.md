---
Status: topology design sealed / caller-zero only
Date: 2026-08-03
Decision: accepted boundary — topology must be fixed before the Accum pilot
Scope: `JOINIR-LOOP-ACCUM-VERIFIED-RECIPE-CONSUMER0-P1`, no M10a wiring
Related:
  - docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - docs/development/current/main/investigations/joinir-loop-phi-materializer-m6b-design-2026-08-03.md
  - docs/development/current/main/design/phi-lifecycle-ssot.md
  - docs/development/current/main/design/binding-ssa-first-control-lowering-ssot.md
---

# Accum verified-recipe consumer: topology design stop

## Decision boundary

M6-B proves only a PHI seam over an explicitly seeded physical function. It is
not yet an Accum MIR-parity or production-consumer proof. The next pilot must
keep the semantic JoinSig and the physical Standard5 layout separate:

```text
portable JoinSig:   Body ───────────────▶ Header
physical Standard5: Body ─▶ Step ──────▶ Header
```

The logical `Body -> Header` backedge is an obligation, not permission for the
PHI materializer to invent a `Step` block. A later CFG skeleton owner must
issue the explicit physical edge/path and sealed predecessor witness. M6-B
continues to consume only that sealed witness; it must not recompute or repair
CFG topology.

## Evidence matrix

| Concern | Portable JoinSig | Existing PlanLowerer / CorePlan | M6-B fixture |
| --- | --- | --- | --- |
| loop entry | `Preheader -> Header` (`Enter`) | preheader to header branch setup | explicit `bb0 -> bb1` |
| predicate | `Header -> Body/After` | header branch to body/after | explicit header branch |
| loop continuation | `Body -> Header` (`Backedge`) | `Body -> Step`, then `Step -> Header` | direct `Body -> Header` |
| carrier PHI | logical payload on incoming edges | header PHI uses `(preheader, init)` and `(step, next)` | header PHI uses `(preheader, init)` and `(body, next)` |
| nested `Always` child | logical child edges and exits | route-specific nested physical blocks | seeded blocks only |

Anchors:

- `src/mir/loop_recipe_contract/join_sig.rs` emits the logical backedge.
- `src/mir/builder/control_flow/plan/features/coreloop_frame.rs` defines the
  Standard5 `step_bb` and header PHI inputs.
- `src/mir/builder/control_flow/plan/steps/loop_wiring_standard5.rs` wires
  `body_bb -> step_bb -> header_bb`.
- `src/mir/builder/control_flow/plan/loop_phi_materializer.rs` currently uses
  the seeded direct-backedge map and therefore must not claim PlanLowerer
  parity.

## P1-D0 topology seal (design-only)

The portable logical edge remains unchanged. The later physical mapping must
carry an explicit one-to-many path and all PHI consequences; the materializer
must never infer a path from logical port names.

| logical obligation | physical witness required | PHI consequence | M6-B scope |
| --- | --- | --- | --- |
| `Enter: Preheader -> Header` | canonical preheader/header edge | header incoming `(preheader, init)` | in scope |
| `PredicateTrue: Header -> Body` | header branch to body | no new PHI row | in scope |
| `PredicateFalse: Header -> After` | header branch plus sealed after reachability | explicit after/final merge row from the header exit predecessor | design sealed; P1b witness |
| `Backedge: Body -> Header` | Standard5 path `Body -> Step -> Header` | header predecessor is terminal `Step`; any staging PHI is explicit | direct body predecessor is not a parity claim |
| `Break: Body -> After` | body-to-after edge or sealed forwarding path | explicit after merge over every reachable break predecessor | design sealed; P1b witness |
| nested `Always` child | parent-body segment -> child header; child break -> child after -> parent step | inherited carrier payload is forwarded; no child predicate/backedge PHI in this golden | design sealed; no physical writer |

The fixtures are intentionally separate:

- `DirectAccumConstLoop` is the singleton/fallthrough row: root
  `Enter + PredicateTrue + PredicateFalse + Backedge`, with the physical
  `Body -> Step -> Header` path and after rows sourced from the exact
  `after_cond_preds` set.
- `accum_nested_v1.json` is the recursive golden: root `Continue` (not
  `Backedge`) and an `Always` child with `Enter + Break`, no child-owned
  carrier, and inherited carrier payload visible on the child edges. The child
  break forwards through child `After` into the parent `Step` path.
- The current M6-B seeded direct `Body -> Header` map is a caller-zero PHI
  seam fixture only; it is not Standard5 parity for either row.

`final_values` is a binding-publication obligation, not another JoinSig edge:
the physical witness must connect the after-PHI/final value to the existing
binding publication step without creating a second SSA owner.

This seal keeps `VerifiedLoopJoinSigV1` as the semantic authority and makes
logical-edge expansion a separate physical capability. `PlanLowerer` remains a
parity oracle only. P1-D0 is now closed as a design boundary; P1b must issue
the sealed physical witness and reject any missing after/step/nested forwarding
row before PHI materialization. M6-B itself remains unchanged and caller-zero.

## Chosen shape for the next owner

Keep `LoopJoinSigV1` semantic and implementation-neutral. Do not add a
route-specific `Step` port to the portable recipe merely to match the legacy
physical layout. Instead, introduce the physical topology as a separate,
test-only sealed product in the pilot:

```text
VerifiedLoopJoinSigV1
  -> test-only canonical CFG skeleton / predecessor seal
  -> logical-edge-to-physical-path mapping
  -> existing LoopPhiMaterializerV1
```

The physical mapping must state whether one logical edge is represented by one
MIR edge or a path of edges. It must carry the exact PHI incoming block/value
rows and be rejected before Builder effects when incomplete. The materializer
does not receive AST, `CanonicalLoopFacts`, `CorePlan`, `variable_map`, route
names, or a retry capability.

## Ordered pilot tasks

### P1-D0 — topology matrix (this stop)

Freeze the logical-to-physical correspondence for the direct Accum row,
including `step_bb`, carrier PHI sites, break/after reachability, and the
nested `Always` child. No production code or `route_loop` wiring is allowed.

### P1a — candidate isolation scaffold

Use the existing `ModuleBuilderInvocationSessionV1` only in `#[cfg(test)]`:

1. Run the legacy Accum composer/PlanLowerer oracle in a dedicated candidate;
   never run it and the new consumer on the same candidate.
2. Drop a candidate after a post-first-PHI injected failure and assert the live
   Builder fingerprint, cursors, module, and function state are unchanged.
3. Open a fresh session from the same live Builder and prove deterministic
   success/reuse. Keep all IDs alpha-normalized in parity assertions.

P1a is an isolation/topology scaffold, not full MIR/result parity.

### P1b — fixed physical skeleton and parity snapshot

After P1-D0 closes, build a test-only Standard5 fixture from explicit CFG
operations and sealed predecessor witnesses. Compare the legacy oracle and
new path using normalized CFG/terminators, PHI rows, type rows, and result
semantics. Raw `ValueId`/`BasicBlockId` numbers and route labels are not truth.

If P1b needs a second operation lowerer, AST reconstruction, or route-local
PHI writer, stop and reopen the design; do not create a parallel scheduler.

## Gates and non-claims

- `route_loop` production caller remains unchanged.
- `materialize_loop_phis` production callers remain zero until the pilot is
  complete and a separate cutover card is accepted.
- `RecipeComposer`, `CorePlan`, and `PlanLowerer` are parity oracles only;
  they are never imported by the new physical consumer.
- `PhiTxn` and Binding-SSA remain the sole PHI/SSA lifecycle owners.
- No M10a singleton bridge, Generic policy change, Retry deletion, or D2
  promotion is authorized by this card.
