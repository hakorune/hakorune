---
Status: active implementation task
Decision: accepted — implement the bounded S6B source observer/Recipe producer
Date: 2026-08-08
DesignPrerequisite: docs/development/current/main/investigations/loop-joinsig-mixed-fallthrough-d0-design-2026-08-08.md
ParentCurrentCard: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
---

# M8 S6B — LoopV0 exits/joins source-to-Recipe implementation

## Scope

Implement one source observer and one deterministic producer for the selected
natural fixture:

```text
apps/tests/loop_break_plan_subset_min.hako
```

The accepted source shape is an `i < 10` loop whose body contains an `if` with
one same-loop `break`, normal fallthrough, accumulator update, and induction
step. It must enter the existing recursive `LoopRecipeV1` algebra and the
already-landed mixed-fallthrough `LoopJoinSigV1` contract.

## Ownership

```text
resolver       exact Loop membership, frame, BindingRef, source sites
observer       source roles and complete body coverage
Facts          one atomic typed Candidate/Declined/Unresolved/Rejected result
producer       role -> Recipe keys and source relation projection
Recipe verifier existing structural Recipe owner
JoinSig        existing logical arm/edge owner
```

Facts must not mint `LoopBindingKeyV1`, `LoopItemKeyV1`, `LoopValueKeyV1`, or
JoinSig edges. The producer must not reread AST, reclassify the family, or
create a physical route.

## Required bounded contract

- two distinct source bindings: induction `i` and accumulator `sum`;
- two initializer/input relations;
- condition `i < literal`;
- terminal arm: same-loop `Break` after the conditional update;
- normal arm: implicit logical `Fallthrough`;
- accumulator update `sum = sum + 10` in the terminal arm;
- normal body update `sum = sum + 1`;
- induction step `i = i + 1`;
- exact source coverage, owner/frame coherence, and no opaque call/effect;
- existing Recipe/JoinSig/Core/input/effect owners only.

`print(sum)` and the callable `return` tail are outside this Loop Recipe.
No PHI, CFG, `BasicBlockId`, `ValueId`, Builder, selector, retry, fallback,
production caller, or legacy deletion is opened by this task.

## Acceptance

Positive:

- exact fixture yields one `Candidate` and the normalized existing Recipe;
- JoinSig contains one `Exit`/`Fallthrough` branch row, one terminal edge,
  and one normal backedge;
- terminal and normal payloads are preserved independently;
- source relation counts and exact source anchors are deterministic.

Negative:

- foreign/duplicate source identity -> `Rejected`;
- missing resolver/source/coverage -> `Unresolved`;
- fully observed unsupported operator/body/exit -> `Declined`;
- no AST rewrite, name lookup, legacy-facts import, or fallback route.

## Guard and closeout

- keep every touched source file below 800 lines;
- run focused S6B observer/Recipe/JoinSig tests, `cargo fmt --check`,
  `git diff --check`, pointer guard, and the active replacement guard;
- update `src/mir/loop_recipe_contract/README.md` and
  `docs/reference/mir/loop-recipe-contract.md` in the same implementation
  commit;
- after green, update `CURRENT_STATE.toml` and this task to `closed`, then
  stop at the S6B boundary before S6C.

## Stop conditions

Return to a new design stop if the fixture requires a new Recipe kind, a
second source authority, a PHI/merge rule beyond the shared JoinSig contract,
an AST reread, or any physical/production fallback.
