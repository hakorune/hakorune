# JOINIR-IF-RECIPE-D0-B3-JOINSIG-PHYSICAL-INPUT-DESIGN-STOP

Status: D0-B3-A/B/C landed; D0-C producer/consumer design stop active
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

1. **Schema/owner design** — frozen by this card. D0-B3-A landed the logical
   port, edge, join-row, reject, and non-Clone wrapper vocabulary in
   `46a4ccfcf8`.
2. **Caller-zero elaborator** — D0-B3-A landed deterministic elaboration from
   `VerifiedIfRecipeV1` only, with no production caller or physical imports.
3. **One-shot physical-input seal** — D0-B3-B landed in `1d9b8aa78d`:
   `from_artifact` consumes one verified artifact, internally elaborates the
   matching signature, and keeps the same artifact+signature pair. No physical
   IDs, Builder, PHI/CFG, or production caller was added.
4. **D0-B3-C guard and boundary gates** — landed in `2e3bdb5be0` with nine
   focused contract tests, the reusable If helper in the existing lane guard,
   and zero production JoinSig/physical-input callers. The existing Loop
   guard's three Nested profile allowlist omissions were corrected separately
   in `b6a936999b`; no Loop semantics changed.
5. **D0-C design stop** — choose the first named producer/consumer adapter for
   the verified artifact+physical-input pair. D0-D owns PHI/CFG adoption and
   caller census; it is not part of this card.

## D0-B3-C design decision

The next bounded slice is a shared guard plus the smallest boundary tests. It
does not add a new per-row shell guard.

**Source authority**

- `VerifiedIfRecipeArtifactV1` is the sole input product.
- `VerifiedIfJoinSigV1` is the logical-edge product.
- `VerifiedIfPhysicalInputV1::from_artifact` is the only physical-input issuer.

**Non-authority**

- raw schema, AST/facts rescans, route selection/retry, Builder, physical IDs,
  CFG, PHI/SSA, and production callers remain outside this slice.
- `into_parts` is a test-only observation until D0-C.

**Guard and gates**

- Add `guard_joinir_if_recipe_contract` to the existing
  `tools/checks/lib/joinir_logical_demand_contract.sh` and call it from the
  existing `mirbuilder_inplace_replacement_guard.sh` entry. The helper is
  currently 549 lines; the entry guard is 783 lines, so keep the entry change
  to one call and keep all checks in the reusable helper.
- Guard the If contract production files for `<800` lines and forbid
  `MirBuilder`, physical IDs, CFG/PHI, AST, route/retry, and `Option` in the
  physical-input/JoinSig files. Do not scan raw `schema.rs` for `Option` because
  its explicit-else field legitimately uses it.
- Guard non-`Clone` verified wrappers, one `from_artifact` definition, zero
  production callers, zero production `into_parts` callers, and zero external
  JoinSig/physical-input construction.
- Extend focused tests only with (a) raw malformed artifact stops at the
  verifier, and (b) changing source receipt preserves the logical signature
  while the physical input retains the source identity. Do not synthesize a
  malformed verified product or force unreachable JoinSig reject arms.

**Fail-fast boundary and non-claims**

Verifier errors stop before physical input. The guard/test slice proves only
  caller-zero API and logical identity; it does not prove physical MIR
  predecessors, PHI placement, CFG shape, candidate isolation, or production
  behavior. D0-C remains the next design row after this slice is green.

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
- `join_sig/` and `physical_input.rs` contain no `MirBuilder`, `ValueId`,
  `BasicBlockId`, `CanonicalCfgSession`, `PhiTxn`, AST, route/retry, or
  `Option` dependencies;
- production Recipe caller count remains zero;
- every touched Rust/test file remains below 800 lines.
- `RUSTFLAGS='-Awarnings' cargo test --lib if_recipe_contract -- --test-threads=1`
  passes 9/9;
- `RUSTFLAGS='-Awarnings' cargo check -q --lib`,
  `bash tools/checks/mirbuilder_inplace_replacement_guard.sh`, and the current
  pointer guard are green.

## Non-claims

This row does not prove physical MIR predecessors, PHI placement, CFG shape,
Builder candidate isolation, or production If behavior. Those belong to the
D0-C producer/consumer design and D0-D after a named adapter and old-edge
retirement plan exist.
