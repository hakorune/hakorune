# JOINIR-IF-RECIPE-D0-B3-JOINSIG-PHYSICAL-INPUT-DESIGN-STOP

Status: D0-B3-A authorized; caller-zero logical JoinSig only
Date: 2026-08-04
Decision target: fixed-shell `IfRecipe` -> logical `IfJoinSig` -> one-shot physical-input capability

This card starts only after D0-B2-D classified the mapper rejection enum. The
first bounded slice is authorized below; PHI, CFG, and production consumers
remain outside the authorization.

## Current authority

`VerifiedIfRecipeArtifactV1` is the only input authority for this row. Its
`VerifiedIfRecipeV1` already passed the structural verifier; the source
receipt, resolved facts, AST, owner brands, and mapper state are not reopened.
The JoinSig elaborator must not re-verify the recipe, rescan source, infer
predecessors, or repair a malformed shape.

The portable contract remains separate from physical ownership:

```text
same-pass facts + function origin
  -> D0-B2 mapper
  -> VerifiedIfRecipeArtifactV1
  -> D0-B3 logical IfJoinSig / physical-input seal
  -> D0-C producer/consumer design
  -> D0-D canonical SSA/PHI adoption
```

The existing canonical physical owner is named but not globally exclusive:
`CanonicalSsaFunctionSessionV2` = `CanonicalCfgSessionV1` +
`BindingSsaBuilderV1` + `PhiTxn`. D0-B3 must not claim repository-wide PHI/SSA
adoption.

## Selected logical product

`IfJoinSigV1` is a semantic transfer proof for the fixed explicit-else shell.
It has no physical block or instruction identity.

Logical ports:

```text
Entry, Condition, Then, Else, Continuation
```

Logical edges:

```text
Entry       -> Condition     (Enter)
Condition   -> Then          (True)
Condition   -> Else          (False)
Then        -> Continuation  (ThenTransfer)
Else        -> Continuation  (ElseTransfer)
```

The continuation obligation is a semantic transfer, not an actual MIR
terminator or CFG predecessor calculation. The sole join row preserves the
recipe-local `(binding, class, entry_value, then_value, else_value)` tuple.
The two branch predecessors must be distinct and exactly `Then` and `Else`.
`entry_value` is the mapper's recipe-local input; it must never be inferred
from the condition read or an arbitrary branch expression.

## One-shot/non-Clone seal

The public construction boundary is consuming and binds the pair together:

```text
VerifiedIfPhysicalInputV1::from_artifact(artifact)
  -> elaborate JoinSig from artifact.recipe()
  -> seal the same artifact + VerifiedIfJoinSigV1
```

`VerifiedIfJoinSigV1` and `VerifiedIfPhysicalInputV1` are private-field,
non-`Clone` wrappers. No API may accept independently verified artifact and
signature values and combine them later. The D0-B3 capability contains no
`ValueId`, `BasicBlockId`, `MirBuilder`, `CanonicalCfgSession`, `PhiTxn`, AST,
route, retry, or `Option`.

## Typed rejection boundary

Only logical-contract failures are owned here:

```text
UnsupportedElseDisposition
PredecessorCountMismatch
NonDistinctPredecessor
MissingJoinValue
ValueClassMismatch
MissingContinuationTransfer
LogicalEdgeMismatch
```

Malformed ordinary input should already have stopped in the recipe verifier or
D0-B2 facts boundary. Do not create synthetic malformed non-`Clone` products
to inflate negative coverage. D0-B2-D's defensive mapper variants remain an
invariant firewall until a real future producer makes one reachable.

## Ordered task slice

1. **Schema/owner design** — frozen by this card. D0-B3-A is authorized for
   the logical port, edge, join-row, reject, and non-Clone wrapper vocabulary.
2. **Caller-zero elaborator** — D0-B3-A: implement deterministic elaboration
   from `VerifiedIfRecipeV1` only; no production caller and no physical
   imports. The one-shot physical-input seal remains a later D0-B3-B slice.
3. **One-shot physical-input seal** — consume an artifact and internally
   elaborate the matching signature; prove independent artifact/signature
   mixing is impossible.
4. **Focused gates** — golden deterministic edge/row digest, exactly-two
   distinct predecessor checks, missing/foreign logical value rejects, and
   static grep guards for physical/route dependencies.
5. **Design close** — only after all gates are green, open D0-C for the
   canonical producer/consumer adapter. D0-D owns PHI/CFG adoption and caller
   census; it is not part of this card.

## Acceptance gates

- valid explicit-else golden produces exactly the five logical ports, five
  edges, one join row, and stable semantic digest on repeated elaboration;
- then/else are distinct predecessors and no implicit-else/nested/effect
  widening is admitted;
- missing/duplicate logical values and class drift return typed rejection
  before any physical effect;
- `VerifiedIfJoinSigV1` and `VerifiedIfPhysicalInputV1` have no `Clone` and
  expose no raw mutable inner constructor;
- artifact+signature pairing is consuming and same-product; no independent
  pair constructor is public;
- `join_sig.rs` and `physical_input.rs` contain no `MirBuilder`, `ValueId`,
  `BasicBlockId`, `CanonicalCfgSession`, `PhiTxn`, AST, route/retry, or
  `Option` dependencies;
- production Recipe caller count remains zero;
- every touched Rust/test file remains below 800 lines.

## Non-claims

This row does not prove physical MIR predecessors, PHI placement, CFG shape,
Builder candidate isolation, or production If behavior. Those belong to D0-C
and D0-D after a named producer/consumer and old-edge retirement plan exist.
