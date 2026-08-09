# LOOP-V2-SEMANTIC-PROGRAM-COSEAL-I0

Status: closed; next `DYNAMIC-FAULT-EXIT-TRANSACTION-D0`
Date: 2026-08-10
Design authority:
`loop-recipe-v2-joinsig-dynamic-d0-design-task-2026-08-10.md`

## Goal

Consume the exact existing Dynamic source/Recipe/envelope aggregate, derive
its JoinSig internally, require After from that same JoinSig, and issue one
move-only semantic program without a split/re-pair API.

## Boundary

```text
VerifiedDynamicFullLoopSourceRecipeEnvelopeV2
  -> internal common JoinSig elaboration
  -> internal require_after(L0, B0, Dynamic)
  -> VerifiedLoopSemanticProgramV2
```

The caller supplies none of:

```text
owner
root Loop key
JoinSig
After binding
Continuation
```

The existing two-site Completion remains a sibling. The co-seal proves only
the exact partition: inner source Return is the Recipe Return path; outer
source Return is the Callable Tail path after Loop After.

## Implementation Decision

The V2 path does not add a crate-visible raw `require_after_binding` or expose
`VerifiedLoopAfterBindingV2` as a separately issued product. The neutral
JoinSig owner instead consumes only one verified Recipe and issues one
non-`Clone`, non-splittable control closure:

```text
VerifiedLoopRecipeV2
  -> private common JoinSig elaboration
  -> derive the sole root-owned carrier from the verified Recipe
  -> private raw After lookup inside join_sig
  -> VerifiedLoopJoinClosureV2 {
       JoinSig,
       exact root-carrier After,
     }
```

`VerifiedLoopJoinClosureV2` has no constructor and no `into_parts`. Its After
and JoinSig are borrow-only. Root Loop key, binding key, and class are derived
from the already-verified Recipe rather than supplied by the compiler profile.
Zero or multiple root-owned carriers reject with typed cardinality evidence.

The profile boundary lives under:

```text
dynamic_full_body_recipe/coseal/semantic_program/
```

as a child of the existing envelope owner. This lets the issuer borrow the
private artifact without adding an envelope accessor or `into_parts` API. The
durable bounded product is named
`VerifiedDynamicFullLoopSemanticProgramV2`: the source authority is still the
exact Dynamic full-body envelope, not a universal V2 callable program.

```text
issue_dynamic_full_loop_semantic_program_v2(envelope)
  -> control closure from envelope.artifact.recipe()
  -> move envelope + closure into one product
```

The existing Completion partition is not rechecked here. It was already
sealed by the source issuer, complete claim coverage, and the exact
inner-Return/outer-Tail mapping. The semantic product keeps the whole envelope
and exposes no Completion accessor or consumer.

## Acceptance

- exact source/Recipe/JoinSig/After origin is sealed atomically;
- mixed/foreign Recipe, JoinSig, source product, After, or Completion partition
  rejects;
- After is exactly `L0/B0/Dynamic` and is borrow-only from the semantic
  program;
- no external `from_after`, raw owner, or split issue API is reachable from the
  V2 path;
- V10/ch remains a local relation and is absent from carrier/After identity;
- implementation, focused tests, owner README, public MIR reference, and task
  receipt update land together;
- all touched source files remain below 800 lines.

Focused guards also require:

```text
LoopJoinSigElaboratorV2 production re-export = 0
raw V2 After alias production re-export = 0
semantic issuer inputs = exact envelope only
semantic product into_parts = 0
loop_recipe_contract -> compiler::dynamic_full_body_recipe imports = 0
```

## Nonclaims

```text
Fault exit transaction
Home capability/install/cleanup
physical transfer/layout/CFG
Tail operand physicalization
Completion consumption
DraftSeal / collector / publication
```

## Closeout receipt

The sole production-facing issuer now accepts exactly one
`VerifiedDynamicFullLoopSourceRecipeEnvelopeV2`. It derives the verified
Recipe root and exactly one root-owned carrier, privately elaborates the common
JoinSig, privately requires After, and moves the envelope plus one
non-splittable `VerifiedLoopJoinClosureV2` into
`VerifiedDynamicFullLoopSemanticProgramV2`.

The exact program evidence is `L0/B0/Dynamic`, five edges, one branch, and two
port bindings. V10/ch remains the existing I6-to-I7 iteration-local relation;
V10 and V14 occur in no payload, port, or After identity. Zero and multiple
root carriers reject before a partial JoinSig/After product exists. The V2
elaborator and raw V2 After alias are absent from the production facade, and a
source guard fixes the one-input/no-split/no-physical boundary.

Focused evidence is Dynamic full-body 19/19, semantic-program 3/3, V2 schema
37/37, V1 JoinSig 31/31, plus `cargo check --lib`, diff check, and current
pointer guard. The largest touched Rust source is 367 lines. Completion remains
an unconsumed sibling inside the retained envelope; Fault/Home and all physical
or publication effects remain closed.
