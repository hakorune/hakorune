---
Status: control/source parity R0 closed; complete producer I0 next
Date: 2026-08-10
Closed row: `LOOP-V2-CONTROL-STRUCTURE-GUARD-R0`
Current row: `LOOP-V2-DYNAMIC-FULL-PRODUCER-I0`
Parent: `dynamic-dispatch-execution-envelope-d0-task-2026-08-10.md`
Mode: BoxShape first; unchanged-source producer second
---

# Complete Dynamic Loop Recipe

## Decision

The accepted `skip_while/4` source stays unchanged. If a valid source row is
not accepted, widen the compiler boundary or stop at a named design question;
never delete, rewrite, rename, or selector-refine the source to fit the
current verifier.

The current V2 operation vocabulary is sufficient for the complete lexical
Loop. The blocker is structural: `LoopRecipeVerifierV2` does not yet seal the
control/source invariants already owned by V1. Therefore the next executable
row is a behavior-neutral verifier/source-claim parity guard, followed by one
complete producer. No call-only or operation-only producer is permitted.

```text
unchanged resolver-backed full source inventory
        ↓ consume once
deterministic complete V2 Recipe + private source-role claims
        ↓ common V2 structural verifier
DynamicFullLoopRecipeCandidateV2
        ↓ later atomic co-seal
source + Recipe + two borrowed Dynamic envelopes
```

The candidate is not named `Verified*`: its artifact is structurally
verified, but its private source-to-key claims have not yet been co-sealed.

## Exact normalized Recipe

```text
loops      = 1
blocks     = 3
bindings   = 1   // mutable induction i only
inputs     = 4   // src, pos-derived i entry, end, pred_chars
carriers   = 1
values     = 18
items      = 17  // 15 operations + If + Exit
exits      = 1   // inner Return only
CallSlots  = 2
ConstI64   = 3   // distinct source anchors: 1, 0, 1
```

Keys and classes:

```text
L0 = root predicate Loop
K0 = condition
K1 = body
K2 = inner If then block
B0 = induction i : Dynamic
V0 = src : Dynamic
V1 = i entry derived from pos : Dynamic
V2 = end : Dynamic
V3 = pred_chars : Dynamic
C0 = Carrier(L0, B0, Dynamic, entry V1)
```

Canonical recursive item order:

| Item | Block | Logical row | Source role |
| --- | --- | --- | --- |
| I0 | K0 | `ReadBinding(B0) -> V4:Dynamic` | `LoopConditionI` |
| I1 | K0 | `DynamicLess(V4,V2) -> V5:Bool` | `LoopCondition` |
| I2 | K1 | `ReadBinding(B0) -> V6:Dynamic` | `SubstringStartI` |
| I3 | K1 | `ReadBinding(B0) -> V7:Dynamic` | `SubstringEndI` |
| I4 | K1 | `ConstI64(1) -> V8:I64` | `SubstringEndDelta` |
| I5 | K1 | `DynamicAdd(V7,V8) -> V9:Dynamic` | `SubstringEndAdd` |
| I6 | K1 | `CallSlot(V0,[V6,V9]) -> V10:Dynamic` | `SubstringCall` |
| I7 | K1 | `CallSlot(V3,[V10]) -> V11:Dynamic` | `IndexOfCall` |
| I8 | K1 | `ConstI64(0) -> V12:I64` | `InnerIfZero` |
| I9 | K1 | `DynamicLess(V11,V12) -> V13:Bool` | `InnerIfCondition` |
| I10 | K1 | `If(V13, then=K2, else=None)` | `InnerIf` |
| I11 | K2 | `ReadBinding(B0) -> V14:Dynamic` | `InnerReturnI` |
| I12 | K2 | `Exit(E0)` | `InnerReturn` |
| I13 | K1 | `ReadBinding(B0) -> V15:Dynamic` | `StepReadI` |
| I14 | K1 | `ConstI64(1) -> V16:I64` | `StepDelta` |
| I15 | K1 | `DynamicAdd(V15,V16) -> V17:Dynamic` | `StepAdd` |
| I16 | K1 | `WriteBinding(B0,V17)` | `StepTargetI` + `StepAssignment` |

```text
K0.items = [I0, I1]
K1.items = [I2..I10, I13..I16]
K2.items = [I11, I12]
L0.condition = Predicate(K0, V5)
L0.body      = K1
E0           = Return(Some(V14)), owner L0
```

`ch` is an iteration-local source relation, not a Recipe binding or carrier.
V10 flows directly into the `indexOf` CallSlot. A later local-value/Home
co-seal proves `V10 CallSlot result -> ch declaration -> exact ch read -> Home`.

The source `return i` after the Loop is Callable Tail. It is not a second
Recipe Exit. The retained two-site Completion is later partitioned exactly:

```text
inner return -> Recipe E0
outer return -> Callable Tail after Loop After
```

Dynamic TypeError/Fault is not lexical `If`, Exit, Return, Completion, false,
Void, or a Recipe value. The producer describes only the normal-result wire.

## Private candidate boundary

```rust
struct DynamicFullLoopRecipeCandidateV2 {
    source: VerifiedDynamicLoopFullBodySourceInventoryV1,
    artifact: VerifiedLoopRecipeArtifactV2,
    claims: DynamicFullLoopRecipeClaimsV2,
}
```

Rules:

- consume the complete source inventory exactly once;
- retain its Loop membership, six binding rows, twenty-eight source rows, and
  two-site Completion without copying or dropping them;
- claims store only semantic source role to Recipe key/role relations;
- claims store no source site, AST, selector, target, envelope, Home/effect,
  Fault, physical ID, or copied operation payload;
- no public claims accessor, arbitrary/test constructor, `first_call`,
  `take_calls`, filter, or partial product;
- expose the structurally verified artifact read-only for golden tests;
- only the later atomic co-seal may consume the candidate parts;
- borrow the complete seven-row envelope catalog later; never consume it or
  store a self-referential `EnvelopeRef`.

Non-item claims cover the exact siblings omitted from the Recipe item table:

```text
Loop -> L0
LoopConditionEnd -> V2
SubstringReceiverSrc -> V0
IndexOfReceiverPredChars -> V3
IndexOfArgumentCh -> V10
PreludeLocalI + PreludeInitializerPos -> B0/C0/V1
ChLocal + IterationLocalCh -> V10 local-value relation
OuterReturn + OuterReturnI -> Callable Tail candidate
```

## R0 — V2 control/source structural parity

This is BoxShape-only. It adds no operation, value class, source shape,
selector, route, or production caller.

The common V2 verifier must seal:

1. exact root existence and `parent=None`;
2. every non-root Loop has one earlier valid parent and one exact Loop item;
3. condition/body blocks belong to their Loop;
4. If child blocks follow the parent, belong to the same Loop, and are used
   exactly once;
5. every block, item, Loop, and Exit has exact structural use;
6. an Exit belongs to the containing block's Loop and is terminal in that
   block;
7. Break/Continue targets are an ancestor or self; Return has no fake Loop
   target;
8. recursive Loop/Block/Item preorder is canonical;
9. the V2 artifact source binding has exact Loop coverage, canonical unique
   paths, and parent/descendant path structure;
10. a resolver-issued Loop source capability has a consuming V2 source-root
    adapter; callers may not manufacture coordinates.

Path-sensitive branch value availability and control edges remain the later
JoinSigV2 owner. R0 must not create a second control-flow graph or claim that
`Return | Fallthrough` is already JoinSig-authorized.

Required negatives:

```text
invalid root parent
foreign condition/body/If block owner
reused or unused block/item/Loop/Exit
If child before parent
Exit owner mismatch
Exit followed by another item
invalid Break/Continue ancestry
non-canonical recursive preorder
missing/duplicate/foreign source Loop row
duplicate or non-descendant source path
manual V2 source coordinates
```

Implementation shape:

```text
typed_schema_v2.rs
  public verifier orchestration + typed operation domains
typed_schema_v2_structure.rs
  neutral topology/use/preorder/terminal checks
source_binding.rs
  one shared source-path verifier over V1/V2 Loop shape
```

Do not grow `typed_schema_v2.rs` beyond the 760-line split trigger. Every
source file remains below the 800-line hard limit.

### R0 closeout

R0 is landed as a behavior-neutral compiler correction.

```text
typed_schema_v2_structure.rs
  root/parent ownership
  exact block/Loop/Exit use
  If child ownership and order
  terminal Exit
  Break/Continue ancestry
  recursive preorder

source_binding.rs
  one shared V1/V2 Loop-source path verifier

resolved_source_adapter.rs
  consuming VerifiedLoopRootSourceV1 -> V2 root claim
```

The verified V2 artifact now owns a non-Clone structurally verified source
claim rather than carrying the raw DTO. The prior Dynamic-operation test
fixture no longer aliases condition and body to one physical logical block;
it uses the exact two-block structure required by the common contract.

Focused evidence:

```text
cargo test -q loop_recipe_contract::typed_schema_v2_structure_tests --lib
  6 passed
cargo test -q loop_recipe_contract::typed_schema_v2 --lib
  37 passed
cargo test -q resolved_root_adapter_issues_v2_root_without_manual_coordinates --lib
  1 passed

typed_schema_v2.rs                 717 lines
typed_schema_v2_structure.rs       296 lines
typed_schema_v2_structure_tests.rs 221 lines
```

R0 added no operation, value class, source shape, selector, Recipe producer,
JoinSig, physical route, or production caller. Path-sensitive control remains
the later JoinSigV2 owner.

## I0 — complete producer

After R0 is green, implement the deterministic producer under:

```text
src/mir/compiler/dynamic_full_body_recipe/
  README.md
  mod.rs
  mapping.rs
  claims.rs
  tests.rs (split before the line trigger)
```

The neutral `loop_recipe_contract` must not import this profile. Reuse the
existing provenance family; do not add a `skip_while` or method-name producer
identity.

Acceptance:

```text
unchanged skip_while source
exact normalized counts/table above
all 28 source roles and 6 binding roles consumed exactly once
one verified V2 artifact inside one unverified private candidate
inner Return retained in Recipe
outer Return and two-site Completion retained outside Recipe
ch remains a local-value relation, not B1/C1
two exact CallSlots; no selector refinement
three distinct literal keys and source anchors
zero envelope/Builder/MIR/provider/runtime imports
```

If R0 or I0 exposes another valid unsupported source row, stop and widen the
compiler. Fixture narrowing is not an acceptance strategy.

## Ordered tasks after I0

```text
1. atomic source/Recipe/CallSlot/envelope co-seal
2. iteration-local ch value/Home co-seal
3. JoinSigV2 for Dynamic carrier and Return|Fallthrough
4. semantic-program and Dynamic Fault compatibility co-seal
5. multi-exit Completion consumption + Dynamic Callable Tail/return ABI
6. full preflight and fresh-session physical canary
7. one production cutover + same-commit legacy retry/fallback deletion
```

The multi-exit row is mandatory before physical activation. The existing
completion consumer handles only one explicit return; it must later claim the
inner and outer exact site set once each, rejecting missing, duplicate, and
foreign claims.

## Nonclaims

```text
no source rewrite or fixture narrowing
no selector-specific Text/I64 refinement
no standalone Verified source-to-key product
no source/envelope co-seal in R0 or producer I0
no Home, effect, Fault, JoinSigV2, continuation, Tail ABI, or Completion use
no Builder / MIR / CFG / PHI
no provider/runtime plan or invocation
no retry/fallback
no physical or production activation
```
