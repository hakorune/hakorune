# LOOP-RECIPE-V2-JOINSIG-DYNAMIC-I0

Status: ordered after `LOOP-JOINSIG-NEUTRAL-ENGINE-R0`
Date: 2026-08-10
Design authority:
`loop-recipe-v2-joinsig-dynamic-d0-design-task-2026-08-10.md`

## Goal

Feed the complete unchanged verified Dynamic Recipe V2 through the common
JoinSig engine and issue one typed V2 logical signature. Add exactly one
control family: the existing one-sided If whose terminal arm is Return to
FunctionExit and whose other arm falls through.

## Exact output

```text
edges:
  Enter          Preheader -> Header       [B0=V1:Dynamic]
  PredicateTrue  Header    -> Body         [B0=V1:Dynamic]
  PredicateFalse Header    -> After        [B0=V1:Dynamic]
  Return         Body      -> FunctionExit [B0=V1:Dynamic]
  Backedge       Body      -> Header       [B0=V17:Dynamic]

branch I10 / condition V13:
  then = Exit(I12, Return, FunctionExit, [B0=V1:Dynamic])
  else = Fallthrough([B0=V1:Dynamic])

port bindings:
  Header B0:Dynamic
  After  B0:Dynamic
```

## Implementation contract

- add a private borrowed V2 view, not a second flow walker;
- preserve `LoopValueClassV2::Dynamic` without V1 conversion;
- enumerate V2 operation def/use rules exhaustively;
- use a typed branch-exit target (`Loop` or `FunctionExit`);
- derive payloads only from Recipe carriers;
- retain Return operand authority only in `E0 -> V14`;
- expose no Continuation constructor and consume no source/profile product.

## Required tests

```text
positive:
  exact five-edge / one-branch / two-port-binding golden
  V1 normalized regression parity

negative:
  Return with Loop target
  Break/Continue with FunctionExit target
  missing/duplicate/wrong-class carrier payload
  wrong backedge value
  V10/ch in payload or port binding
  V14 duplicated as carrier payload
  outer Return represented as Recipe Exit/JoinSig edge
  Fault represented as Recipe/JoinSig transfer
  unsupported V2 operation fails closed
```

## Same-slice updates

```text
src/mir/loop_recipe_contract/README.md
docs/reference/mir/loop-recipe-contract.md
this task receipt
focused positive/negative tests
```

All touched source files remain below 800 lines.

## Nonclaims

```text
source/Recipe/JoinSig atomic co-seal
After/Continuation issuance
Completion consumption
Dynamic Fault execution or cleanup
Home classification/install/cleanup
physical Layout / Builder / MIR / CFG / PHI
production selection / retry / fallback
```
