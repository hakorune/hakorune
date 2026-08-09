# LOOP-JOINSIG-NEUTRAL-ENGINE-R0

Status: ready; executable BoxShape row
Date: 2026-08-10
Design authority:
`loop-recipe-v2-joinsig-dynamic-d0-design-task-2026-08-10.md`

## Goal

Extract the existing V1 JoinSig flow algorithm behind one private borrowed
Recipe view without changing any accepted V1 shape or normalized V1 output.
This is the sole prerequisite for adding V2 without copying control logic.

## Change

```text
VerifiedLoopRecipeV1
  -> private V1 JoinRecipeView
  -> common private flow engine
  -> existing VerifiedLoopJoinSigV1 seal
```

Split by responsibility before any file reaches 800 lines:

```text
join_sig/
  engine/
    mod.rs          orchestration only
    view.rs         private borrowed input contract
    flow.rs         binding/value state
    branch.rs       arm disposition
    visibility.rs   carrier projection
```

Exact filenames may follow the current directory, but each file owns one
responsibility and remains below 800 lines.

## Acceptance

- all existing V1 JoinSig positive and negative tests pass unchanged;
- normalized loop rows, branch rows, payload order, and port bindings are
  byte-for-byte/equality identical;
- the engine has no AST, profile, Builder, MIR, CFG, PHI, Tail, Completion,
  Fault, Home, physical layout, retry, or fallback dependency;
- every V1 operation/item/exit arm is exhaustive;
- no V2 schema import or V2 accepted shape is introduced;
- implementation, focused tests, `src/mir/loop_recipe_contract/README.md`, and
  the active task receipt update land in the same commit;
- all touched source files remain below 800 lines.

## Nonclaims

```text
V2 JoinSig
Dynamic payload
one-arm Return
FunctionExit branch target
semantic-program co-seal
physical transfer authority
production activation
```

## Stop

If behavior parity requires a copied walker, a lossy DTO, a public generic
Plan, or an accepted-shape change, stop and return to D0.
