---
Status: active design stop
Decision: provisional — selected as the shared prerequisite for M8 S6B
Date: 2026-08-08
Exception: durable cross-row logical JoinSig contract required before S6B implementation
ParentCurrentCard: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
---

# Loop JoinSig mixed-fallthrough design

## Current Capsule

- **Current decision:** represent one-sided conditional exits with a shared
  JoinSig branch-arm contract; keep `LoopRecipeV1` unchanged.
- **Current implementation status:** design only; no source observer, producer,
  Builder, MIR, CFG, PHI, or physical caller is opened.
- **Next ordered task:** implement and verify the bounded JoinSig contract,
  then resume `JOINIR-LOOP-M8-LOOPV0-EXITS-JOINS-S6B`.
- **Production stop line:** caller-zero; no selector, retry, fallback, or
  physical transfer/layout authority may consume this row.
- **Retirement finish line:** the old branch/exit authority is removed only at
  the later M10b/M11/M12 cutover after common Recipe parity and caller-zero
  evidence.

## Why this row exists

The pinned natural S6B source is:

```text
apps/tests/loop_break_plan_subset_min.hako

loop(i < 10) {
    if i == 5 {
        sum = sum + 10
        break
    }
    sum = sum + 1
    i = i + 1
}
```

The existing portable Recipe schema already represents `Loop`, `If` with
`else_block: None`, `Break`, operations, carriers, and inputs. The existing
JoinSig, however, only publishes a branch row when both arms are direct exits;
it cannot preserve a terminal arm plus the normal fallthrough arm. Patching
the S6B producer or synthesizing an `else` would create a second authority and
is forbidden.

## Change

Extend the shared logical JoinSig model for a bounded mixed-fallthrough branch.
Keep `LoopRecipeV1`, source authority, resolver capabilities, and physical
ownership unchanged. The extension is a logical dataflow obligation only.

The published relation is one `LoopJoinBranchV1` with two arm records:

```text
LoopJoinBranchV1
  owner_loop
  if_item
  condition
  then_arm: LoopJoinBranchArmV1
  else_arm: LoopJoinBranchArmV1

LoopJoinBranchArmV1
  Exit { exit_item, role: Break|Continue, target_loop, payload }
  Fallthrough { payload }
```

`If` without an `else` supplies an implicit `Fallthrough` arm. This is a
logical convention, not a source node or AST rewrite.

## Contract

- `LoopJoinSigElaboratorV1` is the sole issuer.
- A private block-flow summary may carry one normal continuation and direct
  exit observations while recursively elaborating a block. It is not a second
  public Recipe or control-flow authority.
- A terminal arm captures its own binding/value payload at the exit site and
  does not participate in a binding merge.
- Two normal arms must agree on binding/value state; otherwise reject because
  no PHI obligation is introduced by this row.
- A terminal arm plus a normal arm keeps the normal state as the block's
  continuation and records both arm payloads in the branch relation.
- `LoopJoinLoopV1.edges` remains the owner of loop-level `Break`, `Continue`,
  and `Backedge` roles. The branch relation records the conditional origin.
- Existing M7-S2-A `Exit`/`Exit` remains valid. The new positive shape is
  `Exit`/`Fallthrough` with branch-local writes allowed when represented by the
  Recipe and source evidence.
- First bounded implementation rejects `Return`, nested exit, calls, opaque
  effects, multiple direct exits in one arm, and physical consumers.

## Disposition and fail-fast

```text
Rejected:
  foreign owner/frame/site, duplicate arm/exit identity,
  incoherent target or source capability

Unresolved:
  missing resolver flow, source site, exit target, or complete coverage

Declined:
  fully observed but unsupported Return/nested/call/multiple-exit shape

Candidate:
  exact same-loop Break/Continue arm(s), complete normal/exit flow,
  exact per-arm payloads, and no unverified merge requirement
```

`NoSafeSlice` is the current development state, not a fifth source outcome.

## Done

The implementation row may close only when all are observable:

- branch-arm model and private flow summary are implemented below 800 lines;
- deterministic positive tests cover `Exit`/`Exit`, implicit `Fallthrough`,
  terminal-arm payload capture, and normal continuation after the branch;
- negative tests cover divergent normal-arm state, foreign/duplicate identity,
  Return, nested/multiple exit, and missing flow evidence;
- `LoopRecipeV1` JSON/keys are unchanged and no AST/legacy `LoopFacts` enters
  the portable subtree;
- no selector, Recipe kind, Builder/MIR/CFG/PHI/physical ID, retry, or fallback
  is added;
- focused JoinSig tests, diff/format checks, pointer guard, and touched-file
  line budgets are green;
- the same implementation commit updates
  `src/mir/loop_recipe_contract/README.md` and
  `docs/reference/mir/loop-recipe-contract.md` with the exact envelope,
  caller-zero status, and non-claims.

## Stop

Do not implement S6B source observation or modify `direct_branch_row` until
this shared contract is green. If the natural fixture needs a merge/PHI
meaning beyond the bounded arm model, keep S6B at `NoSafeSlice` and open a
separate explicit design decision. Never synthesize an `else`, re-read AST in
the producer, import legacy facts, or add a route-local physical workaround.

